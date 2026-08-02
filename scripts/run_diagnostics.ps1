# Catisen Diagnostics Script (Improved June 7, 2026)
# Purpose: Run tests, check FPS, cleanup processes

try {
    Write-Output "=== Catisen Diagnostics - $(Get-Date) ==="
    # Add more robust error handling here
    Get-Process -Name "*catisen*" -ErrorAction SilentlyContinue | Stop-Process -Force
    Write-Output "Cleanup done."
} catch {
    Write-Error "Error in diagnostics: $_"
}
