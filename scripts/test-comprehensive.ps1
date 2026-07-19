param(
    [string]$CatisenPath = ".\target\debug\catisen.exe",
    [int]$RunSecondsPerCase = 14,
    [int]$MaxCases = 0,
    [int]$BrowserRunsPerSite = 1,
    [int]$TorRetriesOnNoReq = 2,
    [string]$TorProxy = "socks5h://127.0.0.1:9150",
    [int]$TargetFps = 144,
    [switch]$SkipBrowserBaseline,
    [switch]$FailOnGateErrors
)
Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
$ErrorActionPreference = "Continue"
$env:CATISEN_TEST_MODE = "1"
$env:CATISEN_LOG_FILE = "target/catisen-debug.log"

# Import headless utilities
. ".\scripts\headless-utils.ps1"

function Write-Section {
    param([string]$Text)
    Write-Host "`n=== $Text ===" -ForegroundColor Cyan
}

function Write-DetailedHeader {
    param([string]$ReportPath)
    $timestamps = Get-FormattedTimestamps
    $systemMetrics = Get-SystemMetrics
    $hostName = $env:COMPUTERNAME
    $osInfo = Get-WmiObject win32_operatingsystem | Select-Object -Property @{n="OSName";e={$_.Name}}, @{n="BuildNumber";e={$_.BuildNumber}} -First 1
    
    $headerMd = @"
# Catisen Comprehensive Test Report

**Generated:** $($timestamps.IST) (IST) | $($timestamps.PST) (PST)

## Execution Environment
- **Computer:** $hostName
- **OS:** $($osInfo.OSName) (Build $($osInfo.BuildNumber))
- **Unix Timestamp:** $($timestamps.Unix)

## Test Configuration
- **Catisen Binary:** $CatisenPath
- **Run Seconds/Case:** $RunSecondsPerCase
- **Tor Proxy:** $TorProxy
- **Target FPS:** $TargetFps
- **Total Cases:** $($Script:cases.Count)

## Scenario Summary

| Target URL | TTFB (ms) | TTFC (ms) | TTFR (ms) | TTI (ms) | Full Load (ms) | Status | Bytes | Mode | CPU% | RAM% | GPU% | Render FPS | Ad blocks | Ext. Blocks | Data Saved | IST | PST | Device |
|------------|-----------|-----------|-----------|----------|----------------|--------|-------|------|------|------|------|------------|-----------|-------------|------------|-----|-----|--------|
"@
    
    $headerMd | Set-Content -Path $ReportPath
}

function Add-MetricsRowToReport {
    param(
        [string]$ReportPath,
        [hashtable]$Row
    )
    
    # Format row for markdown table
    "| $($Row.URL) | $($Row.TTFB) | $($Row.TTFC) | $($Row.TTFr) | $($Row.TTI) | $($Row.FullLoad) | $($Row.Status) | $($Row.Bytes) | $($Row.Mode) | $($Row.CPUPercent) | $($Row.RAMPercent) | $($Row.GPUPercent) | $($Row.RenderFPS) | $($Row.Adblocks) | $($Row.ExtBlocks) | $($Row.DataSaved) | $($Row.IST) | $($Row.PST) | $($Row.Device) |" | Add-Content -Path $ReportPath
}

function Get-BrowserPath {
    param([string[]]$Candidates)

    foreach ($p in $Candidates) {
        if (Test-Path $p) {
            return $p
        }
    }

    return $null
}

function Normalize-UrlKey {
    param([string]$Url)

    try {
        $uri = [uri]$Url
        $scheme = $uri.Scheme.ToLowerInvariant()
        $host = $uri.Host.ToLowerInvariant()
        $path = $uri.AbsolutePath
        if ([string]::IsNullOrWhiteSpace($path)) {
            $path = "/"
        }
        if ($path.Length -gt 1) {
            $path = $path.TrimEnd('/')
        }
        return "${scheme}://$host$path"
    } catch {
        return $Url.Trim().ToLowerInvariant().TrimEnd('/')
    }
}

function Set-CatisenEnv {
    param(
        [string]$Geo,
        [string]$Profile,
        [string]$View,
        [bool]$Hardening,
        [bool]$SpoofCanvas,
        [bool]$SpoofWebdriver,
        [int]$Fps
    )

    $env:CATISEN_GEO_LOCATION = $Geo
    $env:CATISEN_BROWSER_PROFILE = $Profile
    $env:CATISEN_VIEW_MODE = $View
    $env:CATISEN_FP_HARDENING = if ($Hardening) { "true" } else { "false" }
    $env:CATISEN_SPOOF_CANVAS_WEBGL = if ($SpoofCanvas) { "true" } else { "false" }
    $env:CATISEN_SPOOF_WEBDRIVER = if ($SpoofWebdriver) { "true" } else { "false" }
    $env:CATISEN_TOR_PROXY = $TorProxy
    $env:CATISEN_TARGET_FPS = [string]$Fps
}

function Wait-ForRequestTelemetry {
    param(
        [int]$ProcessId,
        [string]$DebugLogPath,
        [string]$Url,
        [int]$TimeoutSec,
        [int]$StartLogCount
    )

        $hostFragment = ""
        try {
            $hostFragment = ([uri]$Url).Host.ToLowerInvariant()
        } catch {
            $hostFragment = $Url.ToLowerInvariant()
        }

    $deadline = (Get-Date).AddSeconds($TimeoutSec)
    while ((Get-Date) -lt $deadline) {
        Start-Sleep -Milliseconds 350

        $content = Get-Content -Path $DebugLogPath -ErrorAction SilentlyContinue
        if ($null -eq $content) { continue }
        $tail = $content | Select-Object -Skip $StartLogCount

        foreach ($line in $tail) {
            if ($line -match '^\[REQ\]') {
                $lowerLine = $line.ToLowerInvariant()
                if ($line.Contains($Url) -or $lowerLine.Contains($hostFragment) -or $lowerLine.Contains("mode=error")) {
                    return $true
                }
            }
            if ($line -match '❌ Network Error:') {
                return $true
            }
        }

        if (-not (Get-Process -Id $ProcessId -ErrorAction SilentlyContinue)) {
            return
        }
    }

    return $false
}

function Run-CatisenCase {
    param([hashtable]$Case)

    $fps = if ($Case.ContainsKey("Fps")) { [int]$Case.Fps } else { $TargetFps }
    Set-CatisenEnv -Geo $Case.Geo -Profile $Case.Profile -View $Case.View -Hardening $Case.Hardening -SpoofCanvas $Case.SpoofCanvas -SpoofWebdriver $Case.SpoofWebdriver -Fps $fps

    $args = @("--url", $Case.Url, "--headless")
    if ($Case.Tor) {
        $args += "--tor"
    }

    $attemptMax = if ($Case.Tor) { [math]::Max(1, $TorRetriesOnNoReq + 1) } else { 1 }
    for ($attempt = 1; $attempt -le $attemptMax; $attempt++) {
        Add-Content -Path "target/catisen-debug.log" -Value "[CASE-START] $($Case.Name) attempt=$attempt/$attemptMax url=$($Case.Url) tor=$($Case.Tor) geo=$($Case.Geo) profile=$($Case.Profile) view=$($Case.View) fps=$fps tor_proxy=$TorProxy"

        $startLogCount = 0
        if (Test-Path "target/catisen-debug.log") {
            $startFileContent = Get-Content -Path "target/catisen-debug.log" -ErrorAction SilentlyContinue
            if ($startFileContent) { $startLogCount = $startFileContent.Count }
        }

        Write-Host ("Running case: {0} (attempt {1}/{2})" -f $Case.Name, $attempt, $attemptMax) -ForegroundColor Yellow
        $proc = Start-Process -FilePath $CatisenPath -ArgumentList $args -WindowStyle Hidden -PassThru

        $reqFound = Wait-ForRequestTelemetry -ProcessId $proc.Id -DebugLogPath "target/catisen-debug.log" -Url $Case.Url -TimeoutSec $RunSecondsPerCase -StartLogCount $startLogCount

        if (-not $proc.HasExited) {
            Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
        }

        Add-Content -Path "target/catisen-debug.log" -Value "[CASE-END] $($Case.Name) attempt=$attempt req_found=$reqFound"

        if ($reqFound -or -not $Case.Tor -or $attempt -ge $attemptMax) {
            break
        }

        Write-Host ("NO_REQ for case '{0}', retrying ({1}/{2})..." -f $Case.Name, $attempt, $attemptMax) -ForegroundColor DarkYellow
        Start-Sleep -Milliseconds 400
    }
}

function Get-CurlBaseline {
    param([string]$Url)

    $tmpFile = Join-Path $env:TEMP ("catisen-baseline-" + [guid]::NewGuid().ToString() + ".tmp")
    $fmt = "dns=%{time_namelookup};connect=%{time_connect};ttfb=%{time_starttransfer};total=%{time_total};size=%{size_download};status=%{http_code}"
    $result = & curl.exe -L -s -o $tmpFile -w $fmt $Url 2>$null

    if (Test-Path $tmpFile) {
        Remove-Item $tmpFile -Force -ErrorAction SilentlyContinue
    }

    return $result
}

function Parse-CurlTotalMs {
    param([string]$CurlMetrics)

    if ($CurlMetrics -match "total=(?<total>[0-9.]+)") {
        return [math]::Round(([double]$Matches.total * 1000.0), 2)
    }

    return $null
}

function Get-HeadBaseline {
    param([string]$Url)

    $ms = (Measure-Command {
        try {
            Invoke-WebRequest -Uri $Url -Method Head -TimeoutSec 20 -UseBasicParsing | Out-Null
        } catch {
            # Some targets block HEAD; still useful as a rough baseline.
        }
    }).TotalMilliseconds

    return [math]::Round($ms, 2)
}

function Parse-CatisenRequestLine {
    param([string]$Line)

    $pattern = '^\[REQ\]\s+ts=(?<ts>\S+)\s+status=(?<status>\d+)\s+bytes=(?<bytes>\d+)\s+mode=(?<mode>.+?)\s+tor=(?<tor>\w+)\s+ttfb=(?<ttfb>\d+)\s+ttfc=(?<ttfc>\d+)\s+ttfr=(?<ttfr>\d+)\s+tti=(?<tti>\d+)\s+full=(?<full>\d+)\s+url=(?<url>\S+)\s+cpu=(?<cpu>[0-9.]+)\s+ram=(?<ram>[0-9.]+)\s+gpu=(?<gpu>[0-9.]+)\s+fps=(?<fps>[0-9.]+)\s+device=(?<device>\S+)\s+adblocks=(?<adblocks>\d+)\s+data_saved=(?<datasaved>\d+)\s+ist="(?<ist>.*?)"\s+pst="(?<pst>.*?)"\s+headers=(?<headers>.*)$'

    if ($Line -match $pattern) {
        return [pscustomobject]@{
            ts = $Matches.ts
            status = [int]$Matches.status
            bytes = [int]$Matches.bytes
            mode = $Matches.mode
            tor = ($Matches.tor -eq "true")
            ttfb = [int]$Matches.ttfb
            ttfc = [int]$Matches.ttfc
            ttfr = [int]$Matches.ttfr
            tti = [int]$Matches.tti
            full = [int]$Matches.full
            url = $Matches.url
            cpu = $Matches.cpu
            ram = $Matches.ram
            gpu = $Matches.gpu
            fps = $Matches.fps
            device = $Matches.device
            adblocks = $Matches.adblocks
            datasaved = $Matches.datasaved
            ist = $Matches.ist
            pst = $Matches.pst
            headers = $Matches.headers
        }
    }

    return $null
}

function Parse-CatisenReqCfgLine {
    param([string]$Line)

    $pattern = '^\[REQCFG\]\s+url=(?<url>\S+)\s+mode=(?<mode>\S+)\s+tor=(?<tor>\w+)\s+geo=(?<geo>\w+)\s+profile=(?<profile>\w+)\s+hardening=(?<hardening>\w+)\s+canvas=(?<canvas>\w+)\s+webdriver=(?<webdriver>\w+)\s+injected=(?<injected>\w+)$'

    if ($Line -match $pattern) {
        return [pscustomobject]@{
            url = $Matches.url
            mode = $Matches.mode.ToLowerInvariant()
            tor = ($Matches.tor -eq "true")
            geo = $Matches.geo.ToLowerInvariant()
            profile = $Matches.profile.ToLowerInvariant()
            hardening = ($Matches.hardening -eq "true")
            canvas = ($Matches.canvas -eq "true")
            webdriver = ($Matches.webdriver -eq "true")
            injected = ($Matches.injected -eq "true")
        }
    }

    return $null
}

function Measure-ChromiumBrowser {
    param(
        [string]$BrowserName,
        [string]$Executable,
        [string]$Url,
        [int]$Runs = 1
    )

    if ([string]::IsNullOrWhiteSpace($Executable)) {
        return [pscustomobject]@{
            Browser = $BrowserName
            Url = $Url
            Runs = 0
            AvgMs = $null
            MinMs = $null
            MaxMs = $null
            Status = "Not installed"
        }
    }

    $samples = @()
    $failures = 0
    $lastError = ""

    for ($i = 1; $i -le $Runs; $i++) {
        $tmpOut = Join-Path $env:TEMP ("catisen-$BrowserName-out-" + [guid]::NewGuid().ToString() + ".txt")
        $tmpErr = Join-Path $env:TEMP ("catisen-$BrowserName-err-" + [guid]::NewGuid().ToString() + ".txt")

        $argumentSets = @(
            @("--headless=new", "--disable-gpu", "--disable-extensions", "--disable-background-networking", "--dump-dom", $Url),
            @("--headless", "--disable-gpu", "--disable-extensions", "--disable-background-networking", "--dump-dom", $Url)
        )

        $runSucceeded = $false
        foreach ($args in $argumentSets) {
            $start = Get-Date
            $proc = Start-Process -FilePath $Executable -ArgumentList $args -PassThru -WindowStyle Hidden -RedirectStandardOutput $tmpOut -RedirectStandardError $tmpErr

            try {
                Wait-Process -Id $proc.Id -Timeout 45 -ErrorAction Stop
                $elapsed = (New-TimeSpan -Start $start -End (Get-Date)).TotalMilliseconds
                if ($proc.ExitCode -eq 0) {
                    $samples += $elapsed
                    $runSucceeded = $true
                    break
                }
            } catch {
                Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
            }

            if (Test-Path $tmpErr) {
                $errLine = Get-Content -Path $tmpErr -ErrorAction SilentlyContinue | Select-Object -First 1
                if ($errLine) {
                    $lastError = $errLine
                }
            }
        }

        if (-not $runSucceeded) {
            $failures++
        }

        Remove-Item $tmpOut -Force -ErrorAction SilentlyContinue
        Remove-Item $tmpErr -Force -ErrorAction SilentlyContinue
    }

    if ($samples.Count -eq 0) {
        return [pscustomobject]@{
            Browser = $BrowserName
            Url = $Url
            Runs = 0
            AvgMs = $null
            MinMs = $null
            MaxMs = $null
            Status = if ([string]::IsNullOrWhiteSpace($lastError)) {
                "No successful runs (failures=$failures)"
            } else {
                "No successful runs (failures=$failures, err=$lastError)"
            }
        }
    }

    return [pscustomobject]@{
        Browser = $BrowserName
        Url = $Url
        Runs = $samples.Count
        AvgMs = [math]::Round((($samples | Measure-Object -Average).Average), 2)
        MinMs = [math]::Round((($samples | Measure-Object -Minimum).Minimum), 2)
        MaxMs = [math]::Round((($samples | Measure-Object -Maximum).Maximum), 2)
        Status = if ($failures -gt 0) { "Partial (failures=$failures)" } else { "OK" }
    }
}

Write-Section "Catisen Comprehensive Feature Test"

# Initial cleanup: Kill any remnant processes before starting
Kill-AllCatisenProcesses

if (-not (Test-Path $CatisenPath)) {
    Write-Host "Binary not found at $CatisenPath. Build first using: cargo build" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path "target")) {
    New-Item -ItemType Directory -Path "target" | Out-Null
}

$downloadsDir = "target/downloads"
$existingDownloads = @{}
if (Test-Path $downloadsDir) {
    Get-ChildItem -Path $downloadsDir -File -ErrorAction SilentlyContinue | ForEach-Object {
        $existingDownloads[$_.FullName] = $_.Length
    }
}

$debugLog = "target/catisen-debug.log"
if (Test-Path $debugLog) {
    Remove-Item $debugLog -Force -ErrorAction SilentlyContinue
}
New-Item -Path $debugLog -ItemType File | Out-Null

$cases = @(
    @{ Name = "Fingerprint-CreepJS-Desktop"; Url = "https://abrahamjuliot.github.io/creepjs/"; Tor = $false; Geo = "disabled"; Profile = "windows"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "BotSannysoft-Hardened"; Url = "https://bot.sannysoft.com/"; Tor = $false; Geo = "disabled"; Profile = "linux"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "BrowserScan-Android-Profile"; Url = "https://www.browserscan.net/"; Tor = $false; Geo = "singapore"; Profile = "android"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Geo-Locked-Netflix-Probe"; Url = "https://www.netflix.com/"; Tor = $false; Geo = "switzerland"; Profile = "windows"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Geo-Locked-BBC-Probe"; Url = "https://www.bbc.co.uk/"; Tor = $false; Geo = "uk"; Profile = "macos"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Onion-Tor-Probe"; Url = "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion"; Tor = $true; Geo = "disabled"; Profile = "linux"; View = "source"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Visual-Mode-Bridge"; Url = "https://example.com/"; Tor = $false; Geo = "netherlands"; Profile = "windows"; View = "visual"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Text-Mode-Cleaning"; Url = "https://news.ycombinator.com/"; Tor = $false; Geo = "iceland"; Profile = "windows"; View = "text"; Hardening = $true; SpoofCanvas = $true; SpoofWebdriver = $true },
    @{ Name = "Binary-Download-Interception"; Url = "https://ash-speed.hetzner.com/100MB.bin"; Tor = $false; Geo = "disabled"; Profile = "windows"; View = "source"; Hardening = $false; SpoofCanvas = $false; SpoofWebdriver = $false }
)

if ($MaxCases -gt 0) {
    $cases = $cases | Select-Object -First $MaxCases
}

Write-Section "Running Scenario Matrix"
$index = 1
foreach ($case in $cases) {
    Write-Host ("[{0}/{1}] {2}" -f $index, $cases.Count, $case.Name) -ForegroundColor Green
    Run-CatisenCase -Case $case
    $index++
}

Write-Section "Extracting Catisen Diagnostics"
$reqLines = Select-String -Path $debugLog -Pattern "^\[REQ\]" -ErrorAction SilentlyContinue
if ($null -eq $reqLines -or $reqLines.Count -eq 0) {
    Write-Host "No [REQ] telemetry lines found. Diagnostics capture appears incomplete." -ForegroundColor Red
} else {
    Write-Host ("Captured {0} request telemetry lines." -f $reqLines.Count) -ForegroundColor Green
}

$parsedRequests = @()
if ($reqLines) {
    foreach ($line in $reqLines) {
        $parsed = Parse-CatisenRequestLine -Line $line.Line
        if ($null -ne $parsed) {
            $parsed | Add-Member -NotePropertyName url_key -NotePropertyValue (Normalize-UrlKey -Url $parsed.url)
            $parsedRequests += $parsed
        }
    }
}

$cfgLines = Select-String -Path $debugLog -Pattern "^\[REQCFG\]" -ErrorAction SilentlyContinue
$parsedReqCfg = @()
if ($cfgLines) {
    foreach ($line in $cfgLines) {
        $cfg = Parse-CatisenReqCfgLine -Line $line.Line
        if ($null -ne $cfg) {
            $cfg | Add-Member -NotePropertyName url_key -NotePropertyValue (Normalize-UrlKey -Url $cfg.url)
            $parsedReqCfg += $cfg
        }
    }
}

$gateRows = @()
foreach ($case in $cases) {
    $caseUrlKey = Normalize-UrlKey -Url $case.Url
    $expectedMode = $case.View.ToLowerInvariant()
    $expectedGeo = $case.Geo.ToLowerInvariant()
    $expectedProfile = $case.Profile.ToLowerInvariant()
    $expectedTor = [bool]$case.Tor
    $expectedHardening = [bool]$case.Hardening
    $expectedCanvas = [bool]$case.SpoofCanvas
    $expectedWebdriver = [bool]$case.SpoofWebdriver

    $cfg = @($parsedReqCfg | Where-Object { $_.url_key -eq $caseUrlKey } | Select-Object -Last 1)
    $req = @($parsedRequests | Where-Object { $_.url_key -eq $caseUrlKey } | Select-Object -Last 1)

    $fails = @()

    if ($cfg.Count -eq 0) {
        $fails += "missing REQCFG"
    } else {
        if ($cfg[0].mode -ne $expectedMode) { $fails += "mode mismatch expected=$expectedMode actual=$($cfg[0].mode)" }
        if ($cfg[0].tor -ne $expectedTor) { $fails += "tor cfg mismatch expected=$expectedTor actual=$($cfg[0].tor)" }
        if ($cfg[0].geo -ne $expectedGeo) { $fails += "geo mismatch expected=$expectedGeo actual=$($cfg[0].geo)" }
        if ($cfg[0].profile -ne $expectedProfile) { $fails += "profile mismatch expected=$expectedProfile actual=$($cfg[0].profile)" }
        if ($cfg[0].hardening -ne $expectedHardening) { $fails += "hardening mismatch expected=$expectedHardening actual=$($cfg[0].hardening)" }
        if ($cfg[0].canvas -ne $expectedCanvas) { $fails += "canvas mismatch expected=$expectedCanvas actual=$($cfg[0].canvas)" }
        if ($cfg[0].webdriver -ne $expectedWebdriver) { $fails += "webdriver mismatch expected=$expectedWebdriver actual=$($cfg[0].webdriver)" }

        if ($expectedMode -eq "source") {
            $expectedInjected = ($expectedHardening -or $expectedGeo -ne "disabled")
            if ($cfg[0].injected -ne $expectedInjected) {
                $fails += "injected mismatch expected=$expectedInjected actual=$($cfg[0].injected)"
            }
        }
    }

    if ($req.Count -gt 0) {
        if ($req[0].tor -ne $expectedTor) {
            $fails += "REQ tor mismatch expected=$expectedTor actual=$($req[0].tor)"
        }
    } else {
        $fails += "missing REQ"
    }

    $gateRows += [pscustomobject]@{
        Case = $case.Name
        Url = $case.Url
        ReqCfgFound = ($cfg.Count -gt 0)
        ReqFound = ($req.Count -gt 0)
        Status = if ($fails.Count -eq 0) { "PASS" } else { "FAIL" }
        Notes = if ($fails.Count -eq 0) { "All gates satisfied" } else { ($fails -join "; ") }
    }
}

$chromePath = Get-BrowserPath -Candidates @(
    "$env:ProgramFiles\Google\Chrome\Application\chrome.exe",
    "$env:ProgramFiles(x86)\Google\Chrome\Application\chrome.exe",
    "$env:LOCALAPPDATA\Google\Chrome\Application\chrome.exe"
)

$bravePath = Get-BrowserPath -Candidates @(
    "$env:ProgramFiles\BraveSoftware\Brave-Browser\Application\brave.exe",
    "$env:ProgramFiles(x86)\BraveSoftware\Brave-Browser\Application\brave.exe",
    "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\Application\brave.exe"
)

$safariPath = Get-BrowserPath -Candidates @(
    "$env:ProgramFiles\Safari\Safari.exe",
    "$env:ProgramFiles(x86)\Safari\Safari.exe"
)

$reportStamp = Get-Date -Format "yyyyMMdd-HHmmss"
$reportPath = "target/catisen-comprehensive-report-$reportStamp.md"
$latestReportPath = "target/catisen-comprehensive-report.md"
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
"# Catisen Comprehensive Test Report`n`nGenerated: $now`n" | Set-Content -Path $reportPath
"Config: TorProxy=$TorProxy, TargetFps=$TargetFps`n" | Add-Content -Path $reportPath
"## Scenario Summary`n" | Add-Content -Path $reportPath
"| Case | URL | Tor | Geo | Profile | View | FPS |`n|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($case in $cases) {
    $caseFps = if ($case.ContainsKey("Fps")) { [int]$case.Fps } else { $TargetFps }
    "| $($case.Name) | $($case.Url) | $($case.Tor) | $($case.Geo) | $($case.Profile) | $($case.View) | $caseFps |" | Add-Content -Path $reportPath
}

"`n## Raw Catisen Request Lines`n" | Add-Content -Path $reportPath
if ($reqLines) {
    foreach ($line in $reqLines) {
        "- $($line.Line)" | Add-Content -Path $reportPath
    }
} else {
    "- No request telemetry found." | Add-Content -Path $reportPath
}

"`n## Parsed Catisen Diagnostics (Ctrl+D Equivalent)`n" | Add-Content -Path $reportPath
if ($parsedRequests.Count -gt 0) {
    "| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |`n|---|---|---|---|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath
    foreach ($r in $parsedRequests) {
        "| $($r.ts) | $($r.url) | $($r.status) | $($r.mode) | $($r.tor) | $($r.bytes) | $($r.ttfb) | $($r.ttfc) | $($r.ttfr) | $($r.tti) | $($r.full) |" | Add-Content -Path $reportPath
    }

    $avgTtfb = [math]::Round((($parsedRequests | Measure-Object -Property ttfb -Average).Average), 2)
    $avgTtfc = [math]::Round((($parsedRequests | Measure-Object -Property ttfc -Average).Average), 2)
    $avgTtfr = [math]::Round((($parsedRequests | Measure-Object -Property ttfr -Average).Average), 2)
    $avgTti = [math]::Round((($parsedRequests | Measure-Object -Property tti -Average).Average), 2)
    $avgFull = [math]::Round((($parsedRequests | Measure-Object -Property full -Average).Average), 2)
    $avgBytes = [math]::Round((($parsedRequests | Measure-Object -Property bytes -Average).Average), 2)

    "`nAverages: TTFB=$avgTtfb ms, TTFC=$avgTtfc ms, TTFR=$avgTtfr ms, TTI=$avgTti ms, Full=$avgFull ms, Bytes=$avgBytes" | Add-Content -Path $reportPath
} else {
    "- No parseable [REQ] telemetry lines found." | Add-Content -Path $reportPath
}

$binaryCaseEnabled = @($cases | Where-Object { $_.Name -eq "Binary-Download-Interception" }).Count -gt 0
$downloadStartLines = Select-String -Path $debugLog -Pattern "\[Download\] Starting binary transfer" -ErrorAction SilentlyContinue

$newDownloads = @()
if (Test-Path $downloadsDir) {
    $newDownloads = @(Get-ChildItem -Path $downloadsDir -File -ErrorAction SilentlyContinue | Where-Object {
        -not $existingDownloads.ContainsKey($_.FullName)
    })
}

$downloadBytesTotal = 0
if ($newDownloads.Count -gt 0) {
    $sumObj = $newDownloads | Measure-Object -Property Length -Sum
    if ($sumObj.Sum) {
        $downloadBytesTotal = [int64]$sumObj.Sum
    }
}

"`n## Tor + Geo + Fingerprint Assertion Gates`n" | Add-Content -Path $reportPath
"| Case | ReqCfgFound | ReqFound | Status | Notes |`n|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($g in $gateRows) {
    "| $($g.Case) | $($g.ReqCfgFound) | $($g.ReqFound) | $($g.Status) | $($g.Notes) |" | Add-Content -Path $reportPath
}

$gatePassCount = @($gateRows | Where-Object { $_.Status -eq "PASS" }).Count
$gateFailCount = @($gateRows | Where-Object { $_.Status -eq "FAIL" }).Count
"`nGate Summary: PASS=$gatePassCount, FAIL=$gateFailCount" | Add-Content -Path $reportPath

$binaryAssertionStatus = "N/A"
$binaryAssertionNotes = "Binary case not included in this run."
if ($binaryCaseEnabled) {
    if ($downloadStartLines -and $newDownloads.Count -gt 0) {
        $binaryAssertionStatus = "PASS"
        $binaryAssertionNotes = "Download handoff log found and new file(s) created."
    } elseif ($downloadStartLines -and $newDownloads.Count -eq 0) {
        $binaryAssertionStatus = "FAIL_NO_FILE"
        $binaryAssertionNotes = "Handoff log found, but no new file detected in target/downloads."
    } elseif (-not $downloadStartLines -and $newDownloads.Count -gt 0) {
        $binaryAssertionStatus = "WARN_FILE_WITHOUT_LOG"
        $binaryAssertionNotes = "New file detected, but no explicit [Download] handoff log line was found."
    } else {
        $binaryAssertionStatus = "FAIL_NO_HANDOFF"
        $binaryAssertionNotes = "No handoff log and no new downloaded file detected."
    }
}

"`n## Binary Download Assertion`n" | Add-Content -Path $reportPath
"| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |`n|---|---|---|---|---|---|" | Add-Content -Path $reportPath
"| $binaryCaseEnabled | $(@($downloadStartLines).Count) | $($newDownloads.Count) | $downloadBytesTotal | $binaryAssertionStatus | $binaryAssertionNotes |" | Add-Content -Path $reportPath

if ($newDownloads.Count -gt 0) {
    "`nNew downloaded files:" | Add-Content -Path $reportPath
    foreach ($f in $newDownloads) {
        "- $($f.FullName) ($($f.Length) bytes)" | Add-Content -Path $reportPath
    }
}

if (-not $SkipBrowserBaseline) {
    Write-Section "Collecting Browser and Network Baselines"
    "`n## Browser/Network Comparison`n" | Add-Content -Path $reportPath
    "| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |`n|---|---|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath

    $benchmarkUrls = ($cases | ForEach-Object { $_["Url"] } | Select-Object -Unique | Select-Object -First 5)
    foreach ($url in $benchmarkUrls) {
        $urlHost = ""
        try {
            $urlHost = ([uri]$url).Host.ToLowerInvariant()
        } catch {}

        $catisenRows = @($parsedRequests | Where-Object {
            try {
                ([uri]$_.url).Host.ToLowerInvariant() -eq $urlHost
            } catch {
                $false
            }
        })
        $catisenFull = $null
        $catisenTtfc = $null
        if ($catisenRows.Count -gt 0) {
            $catisenFull = [math]::Round((($catisenRows | Measure-Object -Property full -Average).Average), 2)
            $catisenTtfc = [math]::Round((($catisenRows | Measure-Object -Property ttfc -Average).Average), 2)
        }

        $curlMetrics = Get-CurlBaseline -Url $url
        $curlTotalMs = Parse-CurlTotalMs -CurlMetrics $curlMetrics
        $headMs = Get-HeadBaseline -Url $url

        $chromeResult = Measure-ChromiumBrowser -BrowserName "Chrome" -Executable $chromePath -Url $url -Runs $BrowserRunsPerSite
        $braveResult = Measure-ChromiumBrowser -BrowserName "Brave" -Executable $bravePath -Url $url -Runs $BrowserRunsPerSite

        $safariAvg = $null
        $safariNote = "Unavailable on Windows"
        if ($safariPath) {
            $safariNote = "Installed but not benchmarked (headless mode unsupported in this script)"
        }

        $notes = @()
        if ($chromeResult.Status -ne "OK") { $notes += "Chrome: $($chromeResult.Status)" }
        if ($braveResult.Status -ne "OK") { $notes += "Brave: $($braveResult.Status)" }
        if (-not [string]::IsNullOrWhiteSpace($safariNote)) { $notes += $safariNote }
        $noteText = if ($notes.Count -gt 0) { ($notes -join "; ") } else { "OK" }

        "| $url | $catisenFull | $catisenTtfc | $curlTotalMs | $headMs | $($chromeResult.AvgMs) | $($braveResult.AvgMs) | $safariAvg | $noteText |" | Add-Content -Path $reportPath
        Write-Host ("Comparison done for: {0}" -f $url) -ForegroundColor DarkCyan
    }
}

Write-Section "Run Complete"
try {
    Copy-Item -Path $reportPath -Destination $latestReportPath -Force -ErrorAction Stop
} catch {
    Write-Host "Could not overwrite $latestReportPath (likely open in editor). Timestamped report saved instead." -ForegroundColor DarkYellow
}

Write-Host "Diagnostics log: $debugLog" -ForegroundColor Yellow
Write-Host "Comprehensive report: $reportPath" -ForegroundColor Yellow
Write-Host "Tip: run scripts/diagnostics.ps1 in another terminal for live log stream." -ForegroundColor Gray

if ($FailOnGateErrors -and $gateFailCount -gt 0) {
    Write-Host "Gate assertions failed: $gateFailCount" -ForegroundColor Red
    Kill-AllCatisenProcesses
    exit 2
}

# Final cleanup: Kill all hanging Catisen processes
Kill-AllCatisenProcesses
Write-Host "Test suite complete. All processes cleaned up." -ForegroundColor Green

exit $LASTEXITCODE
