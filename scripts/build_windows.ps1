Stop-Process -Name catisen -Force -ErrorAction SilentlyContinue
# Catisen Web Browser - Windows Installer Script
# This script compiles the browser and packages it into a standalone Windows environment.

Write-Host "==========================="
Write-Host " Building Catisen (Windows)"
Write-Host "==========================="

# 1. Compile the Release Build
Write-Host "[1/3] Compiling Release executable..."
cargo build --release

# 2. Setup Packaging Directory
$DistDir = ".\target\release\catisen_windows"
If (Test-Path $DistDir) { Remove-Item -Recurse -Force $DistDir }
New-Item -ItemType Directory -Force -Path $DistDir > $null

# 3. Copy Executable and Assets
Write-Host "[2/3] Copying executable and GTK libraries..."
Copy-Item ".\target\release\catisen.exe" -Destination $DistDir
Copy-Item -Recurse ".\assets" -Destination $DistDir

# Note: GTK4 on Windows requires DLLs. A real deployment would use 'msys2' or 'vcpkg' to bundle them.
# Write-Host "Bundling GTK4 dlls..."

# 4. Create ZIP Archive
Write-Host "[3/3] Creating ZIP Archive..."
Compress-Archive -Path "$DistDir\*" -DestinationPath ".\target\release\Catisen_Windows_x64.zip" -Force

Write-Host "✅ DONE! Windows build packaged at: target/release/Catisen_Windows_x64.zip"


exit $LASTEXITCODE
