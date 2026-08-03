# Catisen Detailed Test Report
$ErrorActionPreference = "Continue"

$exe = ".\target\debug\catisen.exe"
$logFile = "target\catisen-debug.log"

$sites = @(
    @{ name="Google"; url="https://google.com"; tor=$false },
    @{ name="Reddit"; url="https://reddit.com"; tor=$false },
    @{ name="Instagram"; url="https://instagram.com"; tor=$false },
    @{ name="Ahmia (Darknet)"; url="http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion"; tor=$true }
)

Write-Host "Starting Catisen Advanced Tests..." -ForegroundColor Cyan

foreach ($site in $sites) {
    Write-Host "Testing $($site.name) ($($site.url))"
    Clear-Content $logFile -ErrorAction SilentlyContinue
    
    $myargs = @("--url", "`"$($site.url)`"", "--headless")
    if ($site.tor) { $myargs += "--tor" }

    $timer = Measure-Command {
        $proc = Start-Process -FilePath $exe -ArgumentList $myargs -WindowStyle Hidden -PassThru
        $proc.WaitForExit(30000) | Out-Null
        if (-not $proc.HasExited) {
            $proc.Kill()
        }
    }

    $reqLogs = Get-Content $logFile -ErrorAction SilentlyContinue | Select-String "\[REQ\]"
    if ($reqLogs) {
        $lastReq = $reqLogs[-1].ToString()
        if ($lastReq -match "status=(\d+)") {
            $status = $matches[1]
            if ($status -eq "200") {
                Write-Host " SUCCESS [Status 200, $($timer.TotalSeconds)s]" -ForegroundColor Green
            } else {
                Write-Host " BLOCKED [Status $status, $($timer.TotalSeconds)s]" -ForegroundColor Yellow
            }
        }
    } else {
        Write-Host " TIMEOUT/FAIL [No Data, $($timer.TotalSeconds)s]" -ForegroundColor Red
    }
}
