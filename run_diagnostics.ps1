Write-Host "========================================" -ForegroundColor Cyan
Write-Host " ?? Starting Catisen in Diagnostic Mode " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

cargo build

Write-Host "Starting Browser..." -ForegroundColor Yellow
$job = Start-Job { cargo run }

Write-Host "Tail Live Application Output:" -ForegroundColor Green
while ($job.State -eq "Running") {
    Receive-Job -Job $job | ForEach-Object {
        if ($_ -match "\[ERROR\]") {
            Write-Host $_ -ForegroundColor Red
        } elseif ($_ -match "\[INFO\]" -or $_ -match "\[Network\]") {
            Write-Host $_ -ForegroundColor Green
        } elseif ($_ -match "TTFB") {
            Write-Host $_ -ForegroundColor Yellow
        } else {
            Write-Host $_ -ForegroundColor Cyan
        }
    }
    Start-Sleep -Milliseconds 500
}
Write-Host "Browser Closed." -ForegroundColor Gray
Receive-Job -Job $job
Remove-Job -Job $job
