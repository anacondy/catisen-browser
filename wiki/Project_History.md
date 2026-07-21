# Project History

This document records the evolution of the project, including changes in goals, milestones, and decisions.

---

## 📅 Version 1.0 (March 29, 2026)
### **Initial Goals**
- Build a **privacy-focused browser** with no tracking.
- Implement **ad-blocking** using uBlock Origin.
- **Simplify heavy websites** into minimalistic pages.
- **Fix download speed issues** common in Chromium.
- Support **media playback** with uniform controls.
- Ensure **cross-platform compatibility**.  

### **Tech Stack**
- **Browser Engine**: Servo (Rust) or WebKitGTK (C/GTK).
- **UI**: GTK4 (Linux/Windows) or Cocoa (macOS).
- **Ad Blocking**: uBlock Origin.
- **Media Playback**: FFmpeg + WebKitGTK.
- **Download Manager**: libcurl + custom GUI.

### **Changes from Original Plan**
- **Removed**: Chromium-based approach (too bloated).
- **Added**: Servo/WebKitGTK for lightweight rendering.
- **Added**: Custom download manager to fix speed issues.
- **Added**: Text-only mode for simplifying heavy sites.

---

## 📌 Future Versions

### Version 2.0 (TBD)
- **New Features**:
  - Sync across devices.
  - Extension support (e.g., uBlock Origin, Dark Reader).
- **Improvements**:
  - Faster media playback.
  - Better mobile support.

### Version 3.0 (TBD)
- **New Features**:
  - Tor network integration.
  - Built-in VPN.
- **Improvements**:
  - More customization options.

---

## 📝 Decision Log
| Date       | Change                          | Reason                          |
|------------|---------------------------------|---------------------------------|
| 2026-03-29 | Switched to Servo/WebKitGTK     | Chromium is bloated.            |
| 2026-03-29 | Added custom download manager   | Fix Chromium’s download issues. |
| 2026-03-29 | Added text-only mode            | Simplify heavy sites.           |
| 2026-03-29 | Upgraded to true GTK4 bindings  | Replaced legacy GTK3 bindings (`gtk` crate) with correct `gtk4` crate and resolved build/API issues. |
| 2026-04-02 | Added high-refresh FPS pacing and diagnostics FPS view | Improve smoothness for 90/120/144Hz+ displays and observable performance. |
| 2026-04-02 | Added Tor route metadata diagnostics | Show proxy, Tor confirmation, exit IP, and exit country in diagnostics. |
| 2026-04-02 | Added Tor/clearnet benchmark scripts and dated diagnosis report | Enable reproducible regression tracking for TTFC/TTFB/TTI/Full load and Tor success ratios. |

---

## 📚 Resources
- [Original Vision Document](initial_vision.md)
- [Tech Stack Details](wiki/Technical_Details.md)