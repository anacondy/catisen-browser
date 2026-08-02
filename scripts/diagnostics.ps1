Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
Write-Host "Monitoring Catisen Diagnostics..." -ForegroundColor Cyan
Get-Content -Path target\catisen-debug.log -Wait -Tail 10 -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host $_
}


exit $LASTEXITCODE
