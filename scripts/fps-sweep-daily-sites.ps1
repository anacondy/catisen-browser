param(
    [string]$CatisenPath = ".\target\debug\catisen.exe",
    [int]$RunSecondsPerCase = 10,
    [int[]]$TargetFpsList = @(90, 120, 144, 150),
    [string[]]$Urls = @(
        "https://www.wikipedia.org/",
        "https://github.com/",
        "https://www.reddit.com/",
        "https://news.ycombinator.com/",
        "https://stackoverflow.com/",
        "https://arstechnica.com/"
    )
)
Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
        "https://www.wikipedia.org/",
        "https://github.com/",
        "https://www.reddit.com/",
        "https://www.youtube.com/",
        "https://www.bbc.com/",
        "https://www.amazon.com/"
    )
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

Write-Section "Daily Sites High-Refresh FPS Sweep"
Write-Host "Targets: $($TargetFpsList -join ', ')" -ForegroundColor Yellow
Write-Host "Sites: $($Urls.Count)" -ForegroundColor Yellow

foreach ($url in $Urls) {
    foreach ($target in $TargetFpsList) {
        $env:CATISEN_TARGET_FPS = [string]$target
        $env:CATISEN_VIEW_MODE = "visual"
        $env:CATISEN_FPS_TELEMETRY = "true"
        $env:CATISEN_VISUAL_ENGINE = "servo-spike"

        Add-Content -Path $debugLog -Value "[FPS-DAILY-START] url=$url target=$target"
        Write-Host "Running $url @ $target FPS" -ForegroundColor Green

        $proc = Start-Process -FilePath $CatisenPath -ArgumentList @("--url", $url, "--headless") -WindowStyle Hidden -PassThru

        $memorySamples = @()
        $deadline = (Get-Date).AddSeconds($RunSecondsPerCase)
        while ((Get-Date) -lt $deadline) {
            Start-Sleep -Milliseconds 500

            if (-not (Get-Process -Id $proc.Id -ErrorAction SilentlyContinue)) {
                break
            }

            $p = Get-Process -Id $proc.Id -ErrorAction SilentlyContinue
            if ($p) {
                $memorySamples += [math]::Round(($p.WorkingSet64 / 1MB), 2)
            }
        }

        if (-not $proc.HasExited) {
            Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
        }

        Add-Content -Path $debugLog -Value "[FPS-DAILY-END] url=$url target=$target"

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
        foreach ($line in $newLines) {
            $fps = Parse-FpsLine -Line $line
            if ($null -ne $fps -and $fps.target -eq $target) {
                $fpsSamples += $fps.current
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
        if ($memorySamples.Count -gt 0) {
            $avgMem = [math]::Round((($memorySamples | Measure-Object -Average).Average), 2)
            $peakMem = [math]::Round((($memorySamples | Measure-Object -Maximum).Maximum), 2)
        }

        $results += [pscustomobject]@{
            Url = $url
            TargetFps = $target
            Samples = $fpsSamples.Count
            AvgFps = $avgFps
            MinFps = $minFps
            MaxFps = $maxFps
            AvgMemMb = $avgMem
            PeakMemMb = $peakMem
            DeltaToTarget = if ($null -ne $avgFps) { [math]::Round(($avgFps - $target), 2) } else { $null }
        }
    }
}

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$reportPath = "target/catisen-fps-daily-sites-report-$stamp.md"
$latestReportPath = "target/catisen-fps-daily-sites-report.md"
$now = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

"# Catisen Daily-Sites High-Refresh Report`n`nGenerated: $now`n" | Set-Content -Path $reportPath
"RunSecondsPerCase: $RunSecondsPerCase" | Add-Content -Path $reportPath
"Targets: $($TargetFpsList -join ', ')" | Add-Content -Path $reportPath
"VisualEngine: servo-spike" | Add-Content -Path $reportPath

"`n## Per Site / Target Results`n" | Add-Content -Path $reportPath
"| URL | Target FPS | Samples | Avg FPS | Min FPS | Max FPS | Delta to Target | Avg Mem MB | Peak Mem MB |`n|---|---|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($r in $results) {
    "| $($r.Url) | $($r.TargetFps) | $($r.Samples) | $($r.AvgFps) | $($r.MinFps) | $($r.MaxFps) | $($r.DeltaToTarget) | $($r.AvgMemMb) | $($r.PeakMemMb) |" | Add-Content -Path $reportPath
}

"`n## Aggregate By Target`n" | Add-Content -Path $reportPath
"| Target FPS | Avg of Avg FPS | Min of Avg FPS | Max of Avg FPS | Avg Delta to Target | Avg Mem MB | Peak Mem MB |`n|---|---|---|---|---|---|---|" | Add-Content -Path $reportPath
foreach ($target in $TargetFpsList) {
    $rows = @($results | Where-Object { $_.TargetFps -eq $target -and $null -ne $_.AvgFps })
    if ($rows.Count -gt 0) {
        $avgAvg = [math]::Round((($rows | Measure-Object -Property AvgFps -Average).Average), 2)
        $minAvg = [math]::Round((($rows | Measure-Object -Property AvgFps -Minimum).Minimum), 2)
        $maxAvg = [math]::Round((($rows | Measure-Object -Property AvgFps -Maximum).Maximum), 2)
        $avgDelta = [math]::Round((($rows | Measure-Object -Property DeltaToTarget -Average).Average), 2)
        $avgMem = [math]::Round((($rows | Measure-Object -Property AvgMemMb -Average).Average), 2)
        $peakMem = [math]::Round((($rows | Measure-Object -Property PeakMemMb -Maximum).Maximum), 2)
        "| $target | $avgAvg | $minAvg | $maxAvg | $avgDelta | $avgMem | $peakMem |" | Add-Content -Path $reportPath
    } else {
        "| $target | - | - | - | - | - | - |" | Add-Content -Path $reportPath
    }
}

try {
    Copy-Item -Path $reportPath -Destination $latestReportPath -Force -ErrorAction Stop
} catch {
    Write-Host "Could not overwrite $latestReportPath (likely open in editor). Timestamped report saved instead." -ForegroundColor DarkYellow
}

Write-Section "Daily Sites FPS Sweep Complete"
Write-Host "Diagnostics log: $debugLog" -ForegroundColor Yellow
Write-Host "FPS report: $reportPath" -ForegroundColor Yellow

$Failed = $false
foreach ($target in $TargetFpsList) {
    $rows = @($results | Where-Object { $_.TargetFps -eq $target -and $null -ne $_.AvgFps })
    if ($rows.Count -gt 0) {
        $avgDelta = [math]::Round((($rows | Measure-Object -Property DeltaToTarget -Average).Average), 2)
        if ([math]::Abs($avgDelta) -gt 25.0) {
            "`n[FAIL] Target $target FPS failed acceptance threshold: Avg Delta $avgDelta is worse than +/- 25.0 FPS limit" | Add-Content -Path $reportPath
            Write-Host "[FAIL] Target $target FPS failed threshold: Avg Delta $avgDelta is worse than +/- 25.0" -ForegroundColor Red
            $Failed = $true
        } else {
            "`n[PASS] Target $target FPS meets acceptance threshold: Avg Delta $avgDelta is within +/- 25.0 FPS limit" | Add-Content -Path $reportPath
            Write-Host "[PASS] Target $target FPS meets threshold: Avg Delta $avgDelta is within +/- 25.0" -ForegroundColor Green
        }
    }
}
if ($Failed) {
    Write-Host "`nTest Failed: FPS stability was out of thresholds." -ForegroundColor Red
    exit 1
} else {
    Write-Host "`nTest Passed: FPS targets are stable!" -ForegroundColor Green
    exit 0
}


## Acceptance Thresholds Evaluation`n" | Add-Content -Path $reportPath
$failed = $false
foreach ($target in $targetFpsList) {
    if ($target -ne 0) {
        $rows = @($results | Where-Object { $_.TargetFps -eq $target -and $null -ne $_.AvgFps })
        if ($rows.Count -gt 0) {
            $avgDelta = [math]::Round((( $rows | Measure-Object -Property DeltaToTarget -Average).Average), 2)
            if ([math]::Abs($avgDelta) -gt 25.0) {
                "`n[FAIL] Target $target FPS failed acceptance threshold: Avg Delta $avgDelta is worse than +/- 25.0 FPS limit" | Add-Content -Path $reportPath
                Write-Host "[FAIL] Target $target FPS failed acceptance threshold: Avg Delta $avgDelta is worse than +/- 25.0 FPS limit" -ForegroundColor Red
                $failed = $true
            } else {
                "`n[PASS] Target $target FPS meets acceptance threshold: Avg Delta $avgDelta is within +/- 25.0 FPS limit" | Add-Content -Path $reportPath
                Write-Host "[PASS] Target $target FPS meets acceptance threshold: Avg Delta $avgDelta is within +/- 25.0 FPS limit" -ForegroundColor Green
            }
        }
    }
}

if ($failed) {
    Write-Host "`nTest Failed: FPS stability was out of thresholds." -ForegroundColor Red
    exit 1
} else {
    Write-Host "`nTest Passed: FPS targets are stable!" -ForegroundColor Green
    exit 0
}
