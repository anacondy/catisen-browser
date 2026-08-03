<#
.SYNOPSIS
Focused clearnet + darknet smoke diagnostics for Catisen.

.DESCRIPTION
Builds Catisen (unless -SkipBuild), runs a small URL matrix, waits for
[REQ] telemetry, and writes a concise report.
#>

param(
    [string]$CatisenExe = ".\target\debug\catisen.exe",
    [string]$LogFile = ".\catisen_test.log",
    [string]$ReportFile = ".\catisen_test_report.txt",
    [string]$TorProxy = "socks5h://127.0.0.1:9150",
    [switch]$SkipBuild
)

Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue

$ErrorActionPreference = "Continue"
$env:CATISEN_TEST_MODE = "1"
$env:CATISEN_LOG_FILE = "target/catisen-debug.log"

function Parse-ReqLine {
    param([string]$Line)

    $pattern = '^\[REQ\]\s+ts=(?<ts>\S+)\s+status=(?<status>\d+)\s+bytes=(?<bytes>\d+)\s+mode=(?<mode>.+?)\s+tor=(?<tor>\w+)\s+ttfb=(?<ttfb>\d+)\s+ttfc=(?<ttfc>\d+)\s+ttfr=(?<ttfr>\d+)\s+tti=(?<tti>\d+)\s+full=(?<full>\d+)\s+url=(?<url>\S+)\s+headers=(?<headers>.*)$'
    if ($Line -match $pattern) {
        return [pscustomobject]@{
            Status = [int]$Matches.status
            Bytes = [int]$Matches.bytes
            Full = [int]$Matches.full
            Url = $Matches.url
            Tor = ($Matches.tor -eq "true")
            Raw = $Line
        }
    }

    return $null
}

function Get-HostFragment {
    param([string]$Url)

    try {
        $uri = [uri]$Url
        return $uri.Host.ToLowerInvariant()
    } catch {
        return $Url.ToLowerInvariant()
    }
}

function Wait-ForReqTelemetry {
    param(
        [int]$ProcessId,
        [string]$LogPath,
        [string]$Url,
        [int]$TimeoutSec
    )

    $hostFragment = Get-HostFragment -Url $Url
    $deadline = (Get-Date).AddSeconds($TimeoutSec)

    while ((Get-Date) -lt $deadline) {
        Start-Sleep -Milliseconds 350

        $tail = Get-Content -Path $LogPath -Tail 300 -ErrorAction SilentlyContinue
        $lines = @($tail)
        for ($i = $lines.Count - 1; $i -ge 0; $i--) {
            $line = [string]$lines[$i]
            if ($line -match '^\[REQ\]') {
                $parsed = Parse-ReqLine -Line $line
                if ($null -ne $parsed) {
                    $lowerLine = $line.ToLowerInvariant()
                    if ($lowerLine.Contains($hostFragment) -or $lowerLine.Contains("mode=error") -or $parsed.Status -eq 0) {
                        return $parsed
                    }

                    # Redirects can change host. For one-url-per-process runs, any [REQ] is still a valid completion signal.
                    return $parsed
                }
            }
            if ($line.Contains("❌ Network Error:")) {
                return [pscustomobject]@{
                    Status = 0
                    Bytes = 0
                    Full = 0
                    Url = $Url
                    Tor = $false
                    Raw = $line
                    NetworkError = $true
                }
            }
        }

        if (-not (Get-Process -Id $ProcessId -ErrorAction SilentlyContinue)) {
            break
        }
    }

    return $null
}

if (-not $SkipBuild) {
    Write-Host "Building Catisen..." -ForegroundColor Cyan
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed. Stopping tests." -ForegroundColor Red
        exit 1
    }
}

if (Test-Path $LogFile) { Remove-Item $LogFile -Force -ErrorAction SilentlyContinue }
if (Test-Path $ReportFile) { Remove-Item $ReportFile -Force -ErrorAction SilentlyContinue }
New-Item -Path $LogFile -ItemType File | Out-Null

$TestUrls = @(
    @{ Name = "Mozilla (Clearnet)"; Url = "https://www.mozilla.org"; UseTor = $false; Timeout = 12 },
    @{ Name = "Example (Clearnet)"; Url = "https://example.com"; UseTor = $false; Timeout = 10 },
    @{ Name = "Ahmia (Darknet)"; Url = "http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion"; UseTor = $true; Timeout = 28 }
)

Add-Content -Path $ReportFile -Value "CATISEN CLEARNET/DARKNET REPORT"
Add-Content -Path $ReportFile -Value "Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Add-Content -Path $ReportFile -Value "Tor Proxy: $TorProxy"
Add-Content -Path $ReportFile -Value ""

Write-Host "`nStarting focused clearnet/darknet diagnostics`n" -ForegroundColor Green

foreach ($Test in $TestUrls) {
    Write-Host ("Testing {0} -> {1}" -f $Test.Name, $Test.Url) -ForegroundColor Yellow

    $env:CATISEN_LOG_FILE = $LogFile
    $env:CATISEN_VIEW_MODE = "TextOnly"
    $env:CATISEN_TOR_PROXY = $TorProxy

    $args = @("--url", $Test.Url)
    if ($Test.UseTor) {
        $args += "--tor"
    }

    $process = Start-Process -FilePath $CatisenExe -ArgumentList $args -WindowStyle Hidden -PassThru
    $req = Wait-ForReqTelemetry -ProcessId $process.Id -LogPath $LogFile -Url $Test.Url -TimeoutSec $Test.Timeout

    if (-not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    }

    if ($null -eq $req) {
        $resultText = "FAIL_TIMEOUT"
        Write-Host "FAIL timeout waiting for [REQ] telemetry" -ForegroundColor Red
    }
    elseif ($req.PSObject.Properties.Name -contains "NetworkError") {
        $resultText = "FAIL_NETWORK | $($req.Raw)"
        Write-Host $resultText -ForegroundColor Red
    }
    elseif ($req.Status -gt 0 -and $req.Bytes -gt 0) {
        $resultText = "PASS | status=$($req.Status) | full=$($req.Full)ms | bytes=$($req.Bytes)"
        Write-Host $resultText -ForegroundColor Green
    }
    else {
        $resultText = "FAIL_HTTP | status=$($req.Status) | bytes=$($req.Bytes)"
        Write-Host $resultText -ForegroundColor Red
    }

    Add-Content -Path $ReportFile -Value ("Target: {0}" -f $Test.Name)
    Add-Content -Path $ReportFile -Value ("URL: {0}" -f $Test.Url)
    Add-Content -Path $ReportFile -Value ("Tor Enabled: {0}" -f $Test.UseTor)
    Add-Content -Path $ReportFile -Value ("Result: {0}" -f $resultText)
    if ($null -ne $req) {
        Add-Content -Path $ReportFile -Value ("REQ: {0}" -f $req.Raw)
    }
    Add-Content -Path $ReportFile -Value ""

    Clear-Content -Path $LogFile -ErrorAction SilentlyContinue
}

Write-Host ("`nTests complete. Report saved to {0}" -f $ReportFile) -ForegroundColor Cyan


exit $LASTEXITCODE
