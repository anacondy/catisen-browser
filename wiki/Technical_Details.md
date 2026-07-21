# Technical Details

This document provides in-depth explanations of the tech stack and design decisions.

---

## 🛠️ Browser Engine

### **Servo (Rust)**
- **Pros**:
  - Memory-safe (Rust).
  - Designed for parallelism (fast).
  - Experimental but actively developed.
- **Cons**:
  - Less mature than WebKit/Blink.
  - Limited mobile support.

### **WebKitGTK (C/GTK)**
- **Pros**:
  - Stable and widely used (Safari, GNOME Web).
  - Good mobile support via WebKitGTK.
- **Cons**:
  - Less modern than Servo.
  - C-based (more prone to bugs).

**Decision**: Start with **WebKitGTK** (stable) and experiment with **Servo** (future-proof).

---

## 🎨 UI & Frontend

### **GTK4**
- **Why GTK4?**
  - Native look and feel.
  - Easy theming with CSS.
  - Cross-platform (Linux, Windows, macOS).
- **Customization**:
  - Override default styles in `custom.css`.
  - Use your preferred fonts (e.g., Fira Code).
  - Big buttons for mobile-friendliness.

### **Cocoa (macOS)**
- For macOS-specific UI (native menus, etc.).
- Less customizable than GTK.

---

## 🚫 Ad Blocking

### **uBlock Origin**
- **Why?**
  - Most effective open-source ad blocker.
  - Low performance impact.
- **Integration**:
  - Fork uBlock Origin Core.
  - Modify browser engine to use its filter lists.

---

## 📥 Download Manager

### **libcurl**
- **Why?**
  - No browser throttling.
  - Supports multi-threaded downloads.
  - Resume interrupted downloads.
- **Custom GUI**:
  - Pause/resume buttons.
  - Progress bar with speed indicators.

---

## 🎬 Media Playback

### **FFmpeg + WebKitGTK**
- **FFmpeg**: Decode audio/video files.
- **WebKitGTK**: Native playback with hardware acceleration.
- **Uniform Controls**:
  - Big play/pause, volume, PiP buttons.
  - CSS styling for consistency.

---

## 🔒 Privacy

### **No Telemetry**
- Disable all tracking in the browser engine.
- Use **DuckDuckGo** or **Startpage** as default search.

### **No Search History**
- Clear all data on exit.
- No session storage.

---

## 📱 Mobile Support

### **Qt for Mobile**
- Cross-platform mobile UI.
- Works on Android/iOS (limited).

### **WebView2 (Windows/Android)**
- For Android/iOS, use WebView2 for basic rendering.

---

## 📊 Performance Optimization
- **Memory**: Use Rust (Servo) or C (WebKitGTK) for low overhead.
- **Rendering**: Lazy-load images/videos.
- **JS/CSS**: Strip out heavy scripts (like Brave Shields).

---

## 📚 Further Reading
- [Servo Documentation](https://servo.org/docs/)
- [WebKitGTK Documentation](https://webkitgtk.org/documentation.html)
- [uBlock Origin Core](https://github.com/gorhill/uBlock)
- [GTK4 Tutorial](https://docs.gtk.org/gtk4/) 
### Custom Geolocation Spoofing
HTML5 .getCurrentPosition is overriden globally via JavaScript injection directly into the source rendering view (ViewMode::SourceCode). We simultaneously spoof the X-Forwarded-For inside curl's http_headers(list) to bypass initial server IP validation. Supported locations: US, UK, Japan, Australia.

---

## Current Runtime Architecture (April 2026)

### UI Runtime
- Primary frontend runtime is `eframe/egui` (default feature).
- Diagnostics panel (`Ctrl + D`) now tracks:
  - TTFB / TTFC / TTFR / TTI / Full load
  - Live throughput and average request speed fallback
  - Tor route metadata (proxy, Tor confirmation, exit IP/country)
  - Render FPS and target refresh profile

### High Refresh / High FPS Path
- UI repaint loop is paced by target FPS (`CATISEN_TARGET_FPS` or Settings -> Performance).
- Supported profiles include 90/100/120/144/150+ for high-end monitors.
- FPS is measured from frame deltas and smoothed for diagnostics display.

### Tor Network Path
- Proxy endpoint can be set explicitly via `CATISEN_TOR_PROXY`.
- Auto-detection probes local SOCKS endpoints (9050, 9150) if unset.
- Tor route probe uses `check.torproject.org/api/ip` and `ipapi.co` for exit-country diagnostics.

### Visual Mode
- Visual mode currently executes a snapshot renderer stage:
  - Attempts headless screenshot capture using local browser binaries.
  - Emits snapshot diagnostics in log stream.
  - Falls back to textual payload preview when snapshot capture is unavailable.
- Full interactive Servo DOM/CSS painting remains an active next phase.

### Automation & Benchmarking
- `scripts/test-comprehensive.ps1`: clearnet + Tor scenario matrix, parsed diagnostics, baseline comparison.
- `scripts/test-tor-site-list.ps1`: onion-list matrix in source/text mode with Tor proxy support.

