# How to Build Catisen

This guide will help you compile the browser from source.

---

## 📌 Prerequisites
- **Linux**: GTK4, Rust, CMake, libcurl, FFmpeg
- **Windows**: MSVC, GTK4, Rust, CMake, libcurl, FFmpeg
- **macOS**: XCode, Homebrew, Rust, CMake, libcurl, FFmpeg

---

## 🛠️ Build Instructions

### **Option 1: Servo (Rust)**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install dependencies
sudo apt-get install cmake pkg-config libssl-dev libgtk-3-dev
# Clone Servo
cd ~
git clone https://github.com/servo/servo.git
cd servo
# Build
./mach build --release
# Run
./mach run
```

### **Option 2: WebKitGTK (C/GTK)**
```bash
# Install dependencies
sudo apt-get install build-essential cmake ninja-build gtk-doc-tools gobject-introspection libgtk-3-dev libwebkit2gtk-4.0-dev
# Clone WebKitGTK
cd ~
git clone https://git.webkit.org/WebKit.git webkitgtk
cd webkitgtk
# Build
cmake -S . -B build -G Ninja
cmake --build build
# Run
./build/bin/WebKitWebDriver
```

---

## 📌 Post-Build Steps
1. **Integrate uBlock Origin**:
   - Clone [uBlock Origin Core](https://github.com/gorhill/uBlock).
   - Modify the browser’s engine to use uBlock’s filter lists.

2. **Custom UI**:
   - Override default GTK/CSS styles in `custom.css`.
   - Use your preferred fonts and color scheme.

3. **Download Manager**:
   - Use `libcurl` to implement download functionality.
   - Add a simple GUI for pause/resume.

4. **Media Playback**:
   - Ensure FFmpeg and WebKitGTK are properly linked.

---

## 📚 Troubleshooting
- **Missing dependencies**: Install them via your package manager.
- **Build errors**: Check the [Servo](https://servo.org/docs/) or [WebKitGTK](https://webkitgtk.org/documentation.html) docs.
- **Crashes**: Run with `RUST_BACKTRACE=1` (for Servo) or debug symbols (for WebKitGTK).

---

## 🤝 Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to the build process.