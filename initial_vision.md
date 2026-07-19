# Initial Vision Document

## 📅 Version: 1.0
## 📌 Date: March 29, 2026

---

## 🎯 Core Goals
1. **Build a privacy-focused browser** with no tracking or search history.
2. **Implement ad-blocking** using uBlock Origin’s filter lists.
3. **Simplify heavy websites** into minimalistic, text-based pages.
4. **Fix download speed issues** and failed downloads.
5. **Support media playback** with uniform, mobile-friendly controls.
6. **Ensure cross-platform compatibility** (PC, Linux, macOS, mobile).

---

## 🛠️ Tech Stack Selection
### **Browser Engine**
- **Option 1**: [Servo](https://servo.org/) (Rust-based, experimental but fast).
- **Option 2**: [WebKitGTK](https://webkitgtk.org/) (stable, widely used).

### **UI & Frontend**
- **Option 1**: GTK4 (for Linux/Windows).
- **Option 2**: Cocoa (for macOS).

### **Ad Blocking**
- [uBlock Origin](https://github.com/gorhill/uBlock) for filtering.

### **Media Playback**
- **FFmpeg** for decoding.
- **WebKitGTK** for native playback.

### **Download Manager**
- **libcurl** for direct downloads.
- Custom GUI for pause/resume.

### **Privacy**
- Disable all telemetry.
- No search history or tracking.

---

## 📊 Timeline
| Task                  | Est. Time | Status      |
|-----------------------|-----------|-------------|
| Core Engine Setup     | 2-4 weeks | Not Started |
| Ad Blocking           | 1-2 weeks | Not Started |
| UI Customization      | 1-2 weeks | Not Started |
| Download Manager      | 2-3 weeks | Not Started |
| Media Playback        | 1-2 weeks | Not Started |
| Privacy Hardening     | 1 week    | Not Started |
| Cross-Platform Dev    | 2-3 weeks | Not Started |
| Testing & Bug Fixes   | 3-4 weeks | Not Started |

---

## 📝 Changes from Original Vision
- **Removed**: Chromium-based approach (due to bloatedness).
- **Added**: Servo/WebKitGTK for lightweight rendering.
- **Added**: Custom download manager to fix speed issues.
- **Added**: Text-only mode for simplifying heavy sites.

---

## 🔍 Key Decisions
- **Why Servo/WebKitGTK?**
  - Servo is Rust-based (memory-safe, fast).
  - WebKitGTK is stable and widely supported.
- **Why not Electron?**
  - Electron is bloated; we want a lightweight browser.
- **Why uBlock Origin?**
  - It’s the most effective open-source ad blocker.

---

## 📌 Next Steps
1. Set up the dev environment (Rust/GTK/CMake).
2. Start with a minimal UI (URL bar + blank page).
3. Integrate uBlock Origin and test on a few sites.
4. Build the download manager (libcurl + GUI).
5. Design your theme (GTK CSS + fonts).
6. Optimize media playback (FFmpeg + WebKitGTK).
7. Test, debug, and iterate.

---

## 📚 Resources
- [Servo GitHub](https://github.com/servo/servo)
- [WebKitGTK Documentation](https://webkitgtk.org/documentation.html)
- [uBlock Origin Core](https://github.com/gorhill/uBlock)
- [GTK4 Tutorial](https://docs.gtk.org/gtk4/)