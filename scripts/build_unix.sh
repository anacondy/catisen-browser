#!/bin/bash
# Catisen Web Browser - Linux/macOS Installer Script
# Compiles the browser and creates standard UNIX distributions (like a DEB package).

echo "==========================="
echo " Building Catisen (UNIX) "
echo "==========================="

# 1. Compile the Release Build
echo "[1/3] Compiling Release executable..."
cargo build --release

# 2. Check if we are on Linux to build a DEB file
if [ "$(uname)" == "Linux" ]; then
    echo "[2/3] Using cargo-deb to package out a .deb installer..."
    
    # Check if cargo-deb is installed
    if ! command -v cargo-deb &> /dev/null; then
        echo "cargo-deb not found. Installing now..."
        cargo install cargo-deb
    fi
    
    # Build the .deb file
    cargo deb
    
    echo "✅ DONE! Debian Linux installer created in target/debian/"

elif [ "$(uname)" == "Darwin" ]; then
    echo "[2/3] Packaging macOS Application Bundle (.app)..."
    
    APP_NAME="Catisen.app"
    CONTENTS="target/release/$APP_NAME/Contents"
    
    mkdir -p "$CONTENTS/MacOS"
    mkdir -p "$CONTENTS/Resources"
    
    # Copy executable
    cp target/release/catisen "$CONTENTS/MacOS/"
    # Copy assets
    cp -r assets/* "$CONTENTS/Resources/"
    
    echo "✅ DONE! macOS App bundle created at target/release/$APP_NAME"
fi
