Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
param(
    [string]$CatisenPath = ".\target\debug\catisen.exe",
    [int]$RunSecondsPerCase = 12,
    [string]$TorProxy = "socks5h://127.0.0.1:9150",
    [int]$TargetFps = 144,
    [int]$MaxSites = 0,
    [int]$RetriesOnNoReq = 2,
    [int]$TextModeExtraSeconds = 6
)

$ErrorActionPreference = "Continue"
$env:CATISEN_TEST_MODE = "1"
$env:CATISEN_LOG_FILE = "target/catisen-debug.log"

# Import headless utilities
. ".\scripts\headless-utils.ps1"

function Write-Section {
    param([string]$Text)
    Write-Host "`n=== $Text ===" -ForegroundColor Cyan
}

# Pre-test cleanup
Write-Section "Tor Site List Matrix - Pre-Test Cleanup"
Kill-AllCatisenProcesses

function Normalize-OnionUrl {
    param([string]$Url)

    if ($Url -match '^https?://') {
        return $Url
    }

    return "http://$Url"
}

function Parse-CatisenRequestLine {
    param([string]$Line)

    $pattern = '^\[REQ\]\s+ts=(?<ts>\S+)\s+status=(?<status>\d+)\s+bytes=(?<bytes>\d+)\s+mode=(?<mode>.+?)\s+tor=(?<tor>\w+)\s+ttfb=(?<ttfb>\d+)\s+ttfc=(?<ttfc>\d+)\s+ttfr=(?<ttfr>\d+)\s+tti=(?<tti>\d+)\s+full=(?<full>\d+)\s+url=(?<url>\S+)\s+headers=(?<headers>.*)$'

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
            headers = $Matches.headers
        }
    }

    return $null
}

function Wait-ForRequestTelemetry {
    param(
        [int]$ProcessId,
        [string]$DebugLogPath,
        [string]$Url,
        [int]$TimeoutSec
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

        $tail = Get-Content -Path $DebugLogPath -Tail 220 -ErrorAction SilentlyContinue
        foreach ($line in $tail) {
            if ($line -match '^\[REQ\]') {
                $lowerLine = $line.ToLowerInvariant()
                if ($line.Contains($Url) -or $lowerLine.Contains($hostFragment) -or $lowerLine.Contains("mode=error")) {
                    return
                }
            }
            if ($line -match '❌ Network Error:') {
                return
            }
        }

        if (-not (Get-Process -Id $ProcessId -ErrorAction SilentlyContinue)) {
            return
        }
    }
}

if (-not (Test-Path $CatisenPath)) {
    Write-Host "Binary not found at $CatisenPath. Build first using: cargo build" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path "target")) {
    New-Item -Path "target" -ItemType Directory | Out-Null
}

$debugLog = "target/catisen-debug.log"
if (Test-Path $debugLog) {
    Remove-Item $debugLog -Force -ErrorAction SilentlyContinue
}
New-Item -Path $debugLog -ItemType File | Out-Null

$sites = @(
    @{ Name = "DuckDuckGo Onion"; Url = "duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion" },
    @{ Name = "Ahmia"; Url = "juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion" },
    @{ Name = "Torch"; Url = "xmh57jrknzkhv6y3ls3ubitzfqnkrwxhopf5aygthi7d6rplyvk3noyd.onion" },
    @{ Name = "BBC News Onion"; Url = "bbcnewsd73hkzno2ini43t4gblxvycyac5aw4gnv7t2rccijh7745uqd.onion" },
    @{ Name = "ProPublica"; Url = "p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion" },
    @{ Name = "New York Times Onion"; Url = "nytimesn7cgmftshazwhfgzm37qxb44r64ytbb2dj3x62d2lljsciiyd.onion" },
    @{ Name = "Facebook Onion"; Url = "facebookwkhpilnemxj7asaniu7vnjjbiltxjqhye3mhbshg7kx5tfyd.onion" },
    @{ Name = "ProtonMail Onion"; Url = "protonmailrmez3lotccipjhktkjuu3f3q2z3u2q3k4b4q4b4q.onion" },
    @{ Name = "OnionShare"; Url = "lldan5gahapx5k7iafb3s4ikijc4ni7gx5iywdflkba5y2ezyg6sjgyd.onion" },
    @{ Name = "F-Droid Onion"; Url = "fdroidorg6cooksyluodepej4erfctzk7rrjpjbbr6wx24jh3lqyfwyd.onion" }
)

if ($MaxSites -gt 0) {
    $sites = $sites | Select-Object -First $MaxSites
}

$viewModes = @("source", "text")

$results = @()
$cursor = 0

Write-Section "Tor Site List Matrix"
Write-Host "Cases: $($sites.Count * $viewModes.Count) (all Tor ON)" -ForegroundColor Yellow

foreach ($mode in $viewModes) {
    foreach ($site in $sites) {
        $url = Normalize-OnionUrl -Url $site.Url

        $timeoutSec = $RunSecondsPerCase
        if ($mode -eq "text") {
            $timeoutSec += [Math]::Max(0, $TextModeExtraSeconds)
        }

        $env:CATISEN_VIEW_MODE = $mode
        $env:CATISEN_TOR_PROXY = $TorProxy
        $env:CATISEN_TARGET_FPS = [string]$TargetFps
        $env:CATISEN_GEO_LOCATION = "disabled"
        $env:CATISEN_BROWSER_PROFILE = "linux"
        $env:CATISEN_FP_HARDENING = "true"
        $env:CATISEN_SPOOF_CANVAS_WEBGL = "true"
        $env:CATISEN_SPOOF_WEBDRIVER = "true"

        Write-Host ("Running site='{0}' mode='{1}' timeout={2}s retries={3}" -f $site.Name, $mode, $timeoutSec, $RetriesOnNoReq) -ForegroundColor Green

        $finalReq = $null
        $finalErrors = @()
        $attemptUsed = 0
        $targetHost = ""
        try {
            $targetHost = ([uri]$url).Host.ToLowerInvariant()
        } catch {
            $targetHost = $url.ToLowerInvariant()
        }

        for ($attempt = 1; $attempt -le (1 + $RetriesOnNoReq); $attempt++) {
            $attemptUsed = $attempt
            Add-Content -Path $debugLog -Value "[CASE-START] site=$($site.Name) mode=$mode url=$url attempt=$attempt"

            $proc = Start-Process -FilePath $CatisenPath -ArgumentList @("--url", $url, "--tor", "--headless") -WindowStyle Hidden -PassThru
            Wait-ForRequestTelemetry -ProcessId $proc.Id -DebugLogPath $debugLog -Url $url -TimeoutSec $timeoutSec

            if (-not $proc.HasExited) {
                Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
            }

            Add-Content -Path $debugLog -Value "[CASE-END] site=$($site.Name) mode=$mode attempt=$attempt"

            $allLines = Get-Content -Path $debugLog -ErrorAction SilentlyContinue
            if ($null -eq $allLines) {
                $allLines = @()
            }

            $newLines = @()
            if ($allLines.Count -gt $cursor) {
                $newLines = $allLines[$cursor..($allLines.Count - 1)]
                $cursor = $allLines.Count
            }

            $reqObjects = @()
            foreach ($ln in $newLines) {
                if ($ln -match '^\[REQ\]') {
                    $parsed = Parse-CatisenRequestLine -Line $ln
                    if ($null -ne $parsed) {
                        $parsedHost = ""
                        try {
                            $parsedHost = ([uri]$parsed.url).Host.ToLowerInvariant()
                        } catch {
                            $parsedHost = $parsed.url.ToLowerInvariant()
                        }

                        if ($parsed.url -eq $url -or $parsedHost -eq $targetHost -or $parsed.mode -eq "ERROR") {
                            $reqObjects += $parsed
                        }
                    }
                }
            }

            $errors = $newLines | Where-Object { $_ -match '\[ERROR\]' -or $_ -match 'Network Error' }
            if ($errors) {
                $finalErrors += $errors
            }

            if ($reqObjects.Count -gt 0) {
                $finalReq = $reqObjects[-1]
                break
            }

            if ($attempt -lt (1 + $RetriesOnNoReq)) {
                Write-Host ("NO_REQ for site='{0}' mode='{1}', retrying ({2}/{3})..." -f $site.Name, $mode, $attempt, (1 + $RetriesOnNoReq)) -ForegroundColor DarkYellow
            }
        }

        if ($null -ne $finalReq) {
            $isSuccess = ($finalReq.status -gt 0 -and $finalReq.bytes -gt 0)
            $errorInfo = ""
            if (-not $isSuccess) {
                $errorInfo = "status=$($finalReq.status), bytes=$($finalReq.bytes)"
            }
            if ($attemptUsed -gt 1) {
                if ([string]::IsNullOrWhiteSpace($errorInfo)) {
                    $errorInfo = "req acquired after retry attempt=$attemptUsed"
                } else {
                    $errorInfo = "$errorInfo; req acquired after retry attempt=$attemptUsed"
                }
            }

            $results += [pscustomobject]@{
                Site = $site.Name
                Url = $url
                ViewMode = $mode
                ReqFound = $true
                Success = $isSuccess
                HttpStatus = $finalReq.status
                TorUsed = $finalReq.tor
                Bytes = $finalReq.bytes
                TTFB = $finalReq.ttfb
                TTFC = $finalReq.ttfc
                TTFR = $finalReq.ttfr
                TTI = $finalReq.tti
                Full = $finalReq.full
                Outcome = if ($isSuccess) { "OK" } else { "FAIL_HTTP" }
                ErrorInfo = $errorInfo
            }
        } else {
            $errorInfo = ($finalErrors | Select-Object -Unique) -join " || "
            if ([string]::IsNullOrWhiteSpace($errorInfo)) {
                $errorInfo = "No request telemetry after $attemptUsed attempt(s)"
            }

            $results += [pscustomobject]@{
                Site = $site.Name
                Url = $url
                ViewMode = $mode
                ReqFound = $false
                Success = $false
                HttpStatus = ""
                TorUsed = $true
                Bytes = ""
                TTFB = ""
                TTFC = ""
                TTFR = ""
                TTI = ""
                Full = ""
                Outcome = "NO_REQ"
                ErrorInfo = $errorInfo
            }
        }
    }
}

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$reportPath = "target/catisen-tor-sites-report-$stamp.md"
$latestReportPath = "target/catisen-tor-sites-report.md"
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

"# Catisen Tor Site Matrix Report`n`nGenerated: $now`n" | Set-Content -Path $reportPath
"RunSecondsPerCase: $RunSecondsPerCase" | Add-Content -Path $reportPath
"RetriesOnNoReq: $RetriesOnNoReq" | Add-Content -Path $reportPath
"TextModeExtraSeconds: $TextModeExtraSeconds" | Add-Content -Path $reportPath
"TorProxy: $TorProxy" | Add-Content -Path $reportPath
"TargetFps: $TargetFps" | Add-Content -Path $reportPath
"`n## Summary`n" | Add-Content -Path $reportPath

$okCount = @($results | Where-Object { $_.ReqFound -eq $true }).Count
$noReqCount = @($results | Where-Object { $_.ReqFound -eq $false }).Count
$successCount = @($results | Where-Object { $_.Success -eq $true }).Count
"- Total Cases: $($results.Count)" | Add-Content -Path $reportPath
"- Cases with telemetry [REQ]: $okCount" | Add-Content -Path $reportPath
"- Cases without [REQ]: $noReqCount" | Add-Content -Path $reportPath
"- Cases with successful fetch (status>0 and bytes>0): $successCount" | Add-Content -Path $reportPath

"`n## Detailed Results`n" | Add-Content -Path $reportPath
"| Site | Mode | ReqFound | Status | TorUsed | Bytes | TTFB | TTFC | TTFR | TTI | Full | Outcome |`n|---|---|---|---|---|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($r in $results) {
    "| $($r.Site) | $($r.ViewMode) | $($r.ReqFound) | $($r.HttpStatus) | $($r.TorUsed) | $($r.Bytes) | $($r.TTFB) | $($r.TTFC) | $($r.TTFR) | $($r.TTI) | $($r.Full) | $($r.Outcome) |" | Add-Content -Path $reportPath
}

"`n## Failures / Missing Telemetry`n" | Add-Content -Path $reportPath
$fails = $results | Where-Object { $_.Outcome -ne "OK" }
if ($fails.Count -gt 0) {
    foreach ($f in $fails) {
        "- Site=$($f.Site), Mode=$($f.ViewMode), URL=$($f.Url), Error=$($f.ErrorInfo)" | Add-Content -Path $reportPath
    }
} else {
    "- None" | Add-Content -Path $reportPath
}

"`n## Per-Mode Averages (Successful Cases Only)`n" | Add-Content -Path $reportPath
"| Mode | Avg TTFB | Avg TTFC | Avg TTFR | Avg TTI | Avg Full |`n|---|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($m in $viewModes) {
    $modeRows = $results | Where-Object { $_.ViewMode -eq $m -and $_.ReqFound -eq $true }
    if ($modeRows.Count -gt 0) {
        $avgTtfb = [math]::Round((($modeRows | Measure-Object -Property TTFB -Average).Average), 2)
        $avgTtfc = [math]::Round((($modeRows | Measure-Object -Property TTFC -Average).Average), 2)
        $avgTtfr = [math]::Round((($modeRows | Measure-Object -Property TTFR -Average).Average), 2)
        $avgTti = [math]::Round((($modeRows | Measure-Object -Property TTI -Average).Average), 2)
        $avgFull = [math]::Round((($modeRows | Measure-Object -Property Full -Average).Average), 2)
        "| $m | $avgTtfb | $avgTtfc | $avgTtfr | $avgTti | $avgFull |" | Add-Content -Path $reportPath
    } else {
        "| $m | - | - | - | - | - |" | Add-Content -Path $reportPath
    }
}

try {
    Copy-Item -Path $reportPath -Destination $latestReportPath -Force -ErrorAction Stop
} catch {
    Write-Host "Could not overwrite $latestReportPath (likely open in editor). Timestamped report saved instead." -ForegroundColor DarkYellow
}

Write-Section "Tor Matrix Complete"
Write-Host "Diagnostics log: $debugLog" -ForegroundColor Yellow
Write-Host "Tor report: $reportPath" -ForegroundColor Yellow
Write-Host "Tor proxy used: $TorProxy" -ForegroundColor Yellow

# Final cleanup: Kill all hanging Catisen processes
Kill-AllCatisenProcesses
Write-Host "Test suite complete. All processes cleaned up." -ForegroundColor Green

exit $LASTEXITCODE
