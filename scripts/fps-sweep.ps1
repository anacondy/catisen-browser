Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
param(
    [string]$CatisenPath = ".\target\debug\catisen.exe",
    [string]$Url = "https://example.com/",
    [int]$RunSecondsPerTarget = 12,
    [int[]]$TargetFpsList = @(90, 120, 144, 150),
    [switch]$UseTor,
    [string]$TorProxy = "socks5h://127.0.0.1:9150"
)

$ErrorActionPreference = "Continue"

function Write-Section {
    param([string]$Text)
    Write-Host "`n=== $Text ===" -ForegroundColor Cyan
}

function Parse-FpsLine {
    param([string]$Line)

    $pattern = '^\[FPS\]\s+ts=(?<ts>\S+)\s+target=(?<target>\d+)\s+current=(?<current>[0-9.]+)\s+mode=Visual$'
    if ($Line -match $pattern) {
        return [pscustomobject]@{
            ts = $Matches.ts
            target = [int]$Matches.target
            current = [double]$Matches.current
        }
    }

    return $null
}

function Parse-ReqLine {
    param([string]$Line)

    $pattern = '^\[REQ\]\s+ts=(?<ts>\S+)\s+status=(?<status>\d+)\s+bytes=(?<bytes>\d+)\s+mode=(?<mode>.+?)\s+tor=(?<tor>\w+)\s+ttfb=(?<ttfb>\d+)\s+ttfc=(?<ttfc>\d+)\s+ttfr=(?<ttfr>\d+)\s+tti=(?<tti>\d+)\s+full=(?<full>\d+)\s+url=(?<url>\S+)\s+headers=(?<headers>.*)$'
    if ($Line -match $pattern) {
        return [pscustomobject]@{
            ts = $Matches.ts
            status = [int]$Matches.status
            bytes = [int]$Matches.bytes
            mode = $Matches.mode
            ttfc = [int]$Matches.ttfc
            full = [int]$Matches.full
            url = $Matches.url
        }
    }

    return $null
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

$results = @()
$cursor = 0

Write-Section "FPS Sweep"
Write-Host "Targets: $($TargetFpsList -join ', ')" -ForegroundColor Yellow
Write-Host "URL: $Url" -ForegroundColor Yellow

foreach ($targetFps in $TargetFpsList) {
    $env:CATISEN_TARGET_FPS = [string]$targetFps
    $env:CATISEN_VIEW_MODE = "visual"
    $env:CATISEN_FPS_TELEMETRY = "true"
    $env:CATISEN_GEO_LOCATION = "disabled"

    if ($UseTor) {
        $env:CATISEN_TOR_PROXY = $TorProxy
    }

    Add-Content -Path $debugLog -Value "[FPS-CASE-START] target=$targetFps url=$Url tor=$UseTor"
    Write-Host "Running target FPS=$targetFps" -ForegroundColor Green

    $args = @("--url", $url, "--headless")
    if ($UseTor) {
        $args += "--tor"
    }

    $proc = Start-Process -FilePath $CatisenPath -ArgumentList $args -WindowStyle Hidden -PassThru

    $memorySamplesMb = @()
    $deadline = (Get-Date).AddSeconds($RunSecondsPerTarget)
    while ((Get-Date) -lt $deadline) {
        Start-Sleep -Milliseconds 500

        if (-not (Get-Process -Id $proc.Id -ErrorAction SilentlyContinue)) {
            break
        }

        $p = Get-Process -Id $proc.Id -ErrorAction SilentlyContinue
        if ($p) {
            $memorySamplesMb += [math]::Round(($p.WorkingSet64 / 1MB), 2)
        }
    }

    if (-not $proc.HasExited) {
        Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
    }

    Add-Content -Path $debugLog -Value "[FPS-CASE-END] target=$targetFps"

    $allLines = Get-Content -Path $debugLog -ErrorAction SilentlyContinue
    if ($null -eq $allLines) {
        $allLines = @()
    }

    $newLines = @()
    if ($allLines.Count -gt $cursor) {
        $newLines = $allLines[$cursor..($allLines.Count - 1)]
        $cursor = $allLines.Count
    }

    $fpsSamples = @()
    $reqRows = @()

    foreach ($line in $newLines) {
        $fpsParsed = Parse-FpsLine -Line $line
        if ($null -ne $fpsParsed -and $fpsParsed.target -eq $targetFps) {
            $fpsSamples += $fpsParsed.current
        }

        $reqParsed = Parse-ReqLine -Line $line
        if ($null -ne $reqParsed -and $reqParsed.url -eq $Url) {
            $reqRows += $reqParsed
        }
    }

    $avgFps = $null
    $minFps = $null
    $maxFps = $null
    if ($fpsSamples.Count -gt 0) {
        $avgFps = [math]::Round((($fpsSamples | Measure-Object -Average).Average), 2)
        $minFps = [math]::Round((($fpsSamples | Measure-Object -Minimum).Minimum), 2)
        $maxFps = [math]::Round((($fpsSamples | Measure-Object -Maximum).Maximum), 2)
    }

    $avgMem = $null
    $peakMem = $null
    if ($memorySamplesMb.Count -gt 0) {
        $avgMem = [math]::Round((($memorySamplesMb | Measure-Object -Average).Average), 2)
        $peakMem = [math]::Round((($memorySamplesMb | Measure-Object -Maximum).Maximum), 2)
    }

    $avgTtfc = $null
    $avgFull = $null
    if ($reqRows.Count -gt 0) {
        $avgTtfc = [math]::Round((($reqRows | Measure-Object -Property ttfc -Average).Average), 2)
        $avgFull = [math]::Round((($reqRows | Measure-Object -Property full -Average).Average), 2)
    }

    $results += [pscustomobject]@{
        TargetFps = $targetFps
        Samples = $fpsSamples.Count
        AvgFps = $avgFps
        MinFps = $minFps
        MaxFps = $maxFps
        AvgMemMb = $avgMem
        PeakMemMb = $peakMem
        AvgTtfcMs = $avgTtfc
        AvgFullMs = $avgFull
    }
}

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$reportPath = "target/catisen-fps-sweep-report-$stamp.md"
$latestReportPath = "target/catisen-fps-sweep-report.md"
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

"# Catisen FPS Sweep Report`n`nGenerated: $now`n" | Set-Content -Path $reportPath
"URL: $Url" | Add-Content -Path $reportPath
"RunSecondsPerTarget: $RunSecondsPerTarget" | Add-Content -Path $reportPath
"Targets: $($TargetFpsList -join ', ')" | Add-Content -Path $reportPath
"UseTor: $UseTor" | Add-Content -Path $reportPath
if ($UseTor) {
    "TorProxy: $TorProxy" | Add-Content -Path $reportPath
}

"`n## Stability Table`n" | Add-Content -Path $reportPath
"| Target FPS | Samples | Avg FPS | Min FPS | Max FPS | Avg Mem MB | Peak Mem MB | Avg TTFC ms | Avg Full ms | Delta (Avg FPS - Target) |`n|---|---|---|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath

$baseline = $results | Where-Object { $_.TargetFps -eq ($TargetFpsList[0]) } | Select-Object -First 1
$baselineAvg = $null
if ($baseline) { $baselineAvg = $baseline.AvgFps }

foreach ($r in $results) {
    $delta = $null
    if ($null -ne $r.AvgFps) {
        $delta = [math]::Round(($r.AvgFps - $r.TargetFps), 2)
    }
    "| $($r.TargetFps) | $($r.Samples) | $($r.AvgFps) | $($r.MinFps) | $($r.MaxFps) | $($r.AvgMemMb) | $($r.PeakMemMb) | $($r.AvgTtfcMs) | $($r.AvgFullMs) | $delta |" | Add-Content -Path $reportPath
}

"`n## Deltas vs Baseline`n" | Add-Content -Path $reportPath
if ($null -ne $baselineAvg) {
    "Baseline target: $($TargetFpsList[0]) (Avg FPS=$baselineAvg)" | Add-Content -Path $reportPath
    foreach ($r in $results) {
        if ($r.TargetFps -eq $TargetFpsList[0]) { continue }
        if ($null -ne $r.AvgFps) {
            $d = [math]::Round(($r.AvgFps - $baselineAvg), 2)
            "- Target $($r.TargetFps): Avg FPS delta vs baseline = $d" | Add-Content -Path $reportPath
        } else {
            "- Target $($r.TargetFps): No FPS samples" | Add-Content -Path $reportPath
        }
    }
} else {
    "- Baseline had no FPS samples." | Add-Content -Path $reportPath
}

try {
    Copy-Item -Path $reportPath -Destination $latestReportPath -Force -ErrorAction Stop
} catch {
    Write-Host "Could not overwrite $latestReportPath (likely open in editor). Timestamped report saved instead." -ForegroundColor DarkYellow
}

Write-Section "FPS Sweep Complete"
Write-Host "Diagnostics log: $debugLog" -ForegroundColor Yellow
Write-Host "FPS report: $reportPath" -ForegroundColor Yellow


exit $LASTEXITCODE
