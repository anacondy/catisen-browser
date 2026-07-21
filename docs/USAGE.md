# How to Use Catisen

---

## 📌 Installation
1. **Download the latest release** from [GitHub Releases](https://github.com/anacondy/catisen/releases).
2. **Extract the archive** to a folder.
3. **Run the executable**:
   - Linux: `./catisen`
   - Windows: `catisen.exe`
   - macOS: `open Catisen.app`

---

## 🛠️ Features

### **Ad Blocking**
- Built-in ad blocker using uBlock Origin’s filter lists.
- Toggle ad blocking in the settings menu.

### **Minimalistic UI**
- Dark mode by default (customizable).
- Big, mobile-friendly buttons for audio/video sites.

### **Privacy**
- No search history or tracking.
- Clear all data on exit.

### **Download Manager**
- Pause/resume downloads.
- Multi-threaded downloads for speed.

### **Media Playback**
- Supports audio/video files.
- Picture-in-picture mode.

---

## 🎛️ Controls
| Action               | Shortcut         | Description                     |
|----------------------|------------------|---------------------------------|
| New Tab              | Ctrl+T           | Open a new tab.                 |
| Close Tab            | Ctrl+W           | Close current tab.              |
| Toggle Ad Blocking   | Ctrl+Shift+A     | Enable/disable ad blocking.     |
| Toggle Dark Mode     | Ctrl+Shift+D     | Switch between light/dark mode. |
| Downloads            | Ctrl+J           | Open downloads manager.          |
| Settings             | Ctrl+,           | Open settings menu.             |

---

## 📚 FAQ
### **How do I add a custom theme?**
1. Go to **Settings > Appearance**. 
2. Click **Import Theme** and select your `.css` file.

### **How do I block a specific site?**
1. Go to **Settings > Privacy**. 
2. Add the URL to the block list.

### **How do I enable text-only mode?**
1. Go to **Settings > Performance**. 
2. Toggle **Simplify Heavy Sites**.

### **How do I reset the browser?**
1. Go to **Settings > Advanced**. 
2. Click **Reset to Defaults**.

---

## 🤝 Feedback
- Report bugs or suggest features on [GitHub Issues](https://github.com/anacondy/catisen/issues).
- Join the discussion on [Discussions](https://github.com/anacondy/catisen/discussions).

---

## 🚀 High Refresh & FPS Tuning
- Catisen now supports high-refresh frame pacing for high-end displays.
- In **Settings -> Performance**, choose target refresh/FPS profiles such as **90 / 100 / 120 / 144 / 150 / 165 / 240**.
- You can also set this from terminal:
   - `CATISEN_TARGET_FPS=144`

This setting controls UI repaint pacing and improves responsiveness for diagnostics and live stats.

---

## 🧅 Tor Routing & Exit Diagnostics
- Tor routing can use either:
   - `socks5h://127.0.0.1:9050` (Tor service)
   - `socks5h://127.0.0.1:9150` (Tor Browser)
- If `CATISEN_TOR_PROXY` is not set, Catisen auto-detects local Tor SOCKS ports.
- In diagnostics (`Ctrl + D`), Tor panel now shows:
   - Active proxy
   - Tor confirmation (`IsTor`)
   - Exit IP
   - Exit country
   - Bridge hint (if supplied)

Optional environment variables:
- `CATISEN_TOR_PROXY=socks5h://127.0.0.1:9150`
- `CATISEN_TOR_BRIDGE_HINT=Snowflake`

---

## 🎨 Visual Mode Clarification
- Visual mode now defaults to a **Servo-spike staged pipeline**:
   - DOM parse
   - Style resolve
   - Layout
   - Paint/composite to a frame texture used by egui
- Stage timings and pipeline outputs are written to `target/servo_spike/manifests/*.json`.
- Composited frame outputs are written to `target/servo_spike/frames/*.png`.
- Snapshot bridge remains available by setting:
   - `CATISEN_VISUAL_ENGINE=snapshot`

This is still a spike implementation, not yet full Servo engine DOM/CSS/compositor integration.

---

## 🧪 Benchmark & Regression Scripts
- Comprehensive clearnet/Tor benchmark:
   - `./scripts/test-comprehensive.ps1 -RunSecondsPerCase 14 -TorProxy "socks5h://127.0.0.1:9150" -TargetFps 144 -TorRetriesOnNoReq 2 -FailOnGateErrors`
- Tor site matrix (source + text mode):
   - `./scripts/test-tor-site-list.ps1 -RunSecondsPerCase 16 -RetriesOnNoReq 2 -TextModeExtraSeconds 6 -TorProxy "socks5h://127.0.0.1:9150" -TargetFps 144`
- Daily-site high-refresh sweep (visual spike path):
   - `./scripts/fps-sweep-daily-sites.ps1 -RunSecondsPerCase 8 -TargetFpsList @(90,120,144,150)`

Reports are generated under `target/` with timestamped filenames.