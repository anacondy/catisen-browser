Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
$catisenPath = ".\target\debug\catisen.exe"
$testUrls = @(
    "https://abrahamjuliot.github.io/creepjs/",
    "https://thepiratebay.org/",
    "https://duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion",
    "https://apple.com"
)
foreach ($url in $testUrls) {
    Write-Host "Launching $url ..." -ForegroundColor Cyan
    $timer = Measure-Command {
        Start-Process $catisenPath -WindowStyle Hidden "--url `"$url`" --headless" -PassThru
    }
    Write-Host "Launch time: $($timer.TotalSeconds)s" -ForegroundColor Green
    Start-Sleep -Seconds 8
}


exit $LASTEXITCODE
