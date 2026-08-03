# Troubleshooting

This document lists common issues and their solutions.

---

## ❌ Build Errors

### **Servo Build Fails**
- **Error**: Missing dependencies.
- **Solution**:
  ```bash
  sudo apt-get install cmake pkg-config libssl-dev libgtk-3-dev
  ```

### **GTK4 App Panics on Startup**
- **Error**: The app crashes immediately upon running with an `assets/...` not found error.
- **Solution**: Ensure your working directory contains the `assets/mentality_bg.png` file, or comment out the `Image::from_file` call during development.

### **WebKitGTK Build Fails**
- **Error**: Ninja build tool not found.
- **Solution**:
  ```bash
  sudo apt-get install ninja-build
  ```

---

## 📡 Ad Blocking Not Working

### **Issue**: Ads still appear.
- **Solution**:
  - Update uBlock Origin’s filter lists.
  - Check if the site uses anti-adblocking scripts (whitelist if necessary).

---

## ⚡ Downloads Are Slow

### **Issue**: Downloads are throttled or canceled.
- **Solution**:
  - Use **libcurl** instead of browser-based downloads.
  - Enable multi-threaded downloads in the download manager.

---

## 🎬 Media Playback Issues

### **Issue**: Videos/audio don’t play.
- **Solution**:
  - Ensure FFmpeg and WebKitGTK are properly linked.
  - Check for missing codecs.

---

## 📱 UI Looks Broken

### **Issue**: Styles/CSS not applied.
- **Solution**:
  - Verify `custom.css` is loaded.
  - Check GTK theme compatibility.

---

## 🔍 Performance Is Bad

### **Issue**: Browser is slow.
- **Solution**:
  - Disable heavy extensions.
  - Use Servo for better memory management.

---

## 📚 Further Help
- [GitHub Issues](https://github.com/anacondy/catisen/issues)
- [Discussions](https://github.com/anacondy/catisen/discussions)

---

## 🧅 Tor Connected But Requests Fail

### Symptoms
- Diagnostics shows Tor enabled but requests fail or timeout.
- `.onion` pages do not load.

### Fix
- If using Tor Browser, use SOCKS `9150`:
  - `CATISEN_TOR_PROXY=socks5h://127.0.0.1:9150`
- If using standalone Tor service, use SOCKS `9050`:
  - `CATISEN_TOR_PROXY=socks5h://127.0.0.1:9050`
- If unset, Catisen auto-detects 9050/9150.

### Validate
- Open diagnostics (`Ctrl + D`) and confirm:
  - Proxy endpoint
  - `IsTor=yes`
  - Exit IP and country

---

## 🎨 Visual Mode Shows Text Instead of Full Page

### Why It Happens
- Visual mode is currently snapshot-driven, not a full interactive DOM renderer.
- If no compatible headless browser can be executed, Catisen falls back to payload preview text.

### Fix
- Install one of: Edge, Chrome, Brave, Firefox, or Tor Firefox.
- Optionally set explicit browser path:
  - `CATISEN_VISUAL_BROWSER=<full-path-to-browser-exe>`
- Check diagnostics logs for `[Visual]` messages.

---

## ⚡ Live Speed Shows 0 MB/s

### Why It Happens
- Speed drops to zero when no chunks are currently arriving.
- Some sites transfer data in bursts, then idle.

### Current Behavior
- Diagnostics uses rolling speed updates plus average request-speed fallback.
- High-refresh repaint pacing improves live updates.

### Improve Responsiveness
- Set higher repaint target:
  - `CATISEN_TARGET_FPS=120` or `CATISEN_TARGET_FPS=144`