Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
$catisenPath = ".\target\debug\catisen.exe"
$urls = @("https://youtube.com", "https://reddit.com", "https://1337x.to")
foreach ($u in $urls) { Start-Process $catisenPath -WindowStyle Hidden "--url `"$u`" --tor --headless" }
Write-Host "Instances launched with Tor toggled" -ForegroundColor Yellow


exit $LASTEXITCODE
