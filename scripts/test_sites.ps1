Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
Write-Host "Catisen Test URLs" -ForegroundColor Cyan; Write-Host "Test 1: IP Geolocation (Enable Spoofing in Ctrl+,)"; Write-Host "URL: ipinfo.io/json`n"; Write-Host "Test 2: Banned Site via Tor (Enable Tor in Ctrl+,)"; Write-Host "URL: bbc.com`n"; Write-Host "Test 3: Binary Download (Should trigger DownloadManager)"; Write-Host "URL: speed.hetzner.info/100MB.bin`n";
Write-Host "Test 4: Grey Area Testing (Proxy/Geo Banned bypass check)"; Write-Host "URL: myip.com`n"; Write-Host "Test 5: Live Debug Network Speed Test"; Write-Host "URL: speed.hetzner.info/10GB.bin`n";


exit $LASTEXITCODE
