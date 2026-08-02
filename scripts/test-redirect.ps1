Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
$catisenPath = ".\target\debug\catisen.exe"
$redirectSites = @("https://youtube.com", "https://twitter.com", "https://bit.ly/any-short-link")
foreach ($site in $redirectSites) {
    Write-Host "Testing redirect on $site" -ForegroundColor Magenta
    Start-Process $catisenPath -WindowStyle Hidden "--url `"$site`" --headless"
    Start-Sleep -Seconds 5
}


exit $LASTEXITCODE
