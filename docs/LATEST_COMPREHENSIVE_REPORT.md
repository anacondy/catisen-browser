# Catisen Renderer & Network Stealth Report (May 28, 2026)

## 1. Network Stealth Improvements
To mitigate aggressive bot protection 403s on mainstream websites, we injected canonical Chromium headers into `reqwest` inside `src/network/client.rs`. 
- Added `Sec-Ch-Ua`, `Sec-Ch-Ua-Mobile`, `Sec-Ch-Ua-Platform` mocking Windows x64.
- Added `Sec-Fetch-Dest` with standard document headers.
- Emulated a standard HTTP context via `Accept-Language` padding.

### Stealth Matrices Results
Tested using `catisen.exe --url <SITE> --headless` via updated script:
* **Google (`https://google.com`)**: SUCCESS (Status 200). Fetched in ~1.4s. Bypassed default scrape prevention.
* **Reddit (`https://reddit.com`)**: BLOCKED (Status 403). Reddit relies heavily on JA3 TLS signatures, dropping `reqwest/native-tls` despite correctly formatted HTTP headers. Full scraping requires a bypass engine.
* **Instagram (`https://instagram.com`)**: SUCCESS (Status 200). Initial payload of 675KB successfully downloaded without redirect loops.

## 2. Darknet & Onion Routing
The `.onion` resolver was successfully tested utilizing Tor background proxies.
* **Ahmia Darknet (`http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion`)**: 
  - Over `.onion` proxy parameter `--tor`: SUCCESS (Status 200). Target resolved in ~30s through local `socks5h://127.0.0.1:9150`.
  - Visual mode correctly intercepts the request and injects it into the native EGUI UI.

## 3. CPU Renderer Pipeline Upgrades
As an alternative to relying on an incomplete and heavy Servo dependency (15GB build footprint), the fallback `image::RgbaImage` layout engine `src/servo_renderer.rs` received fundamental upgrades:
- **Media Canvas Wrappers**: Added parsing and colored placeholders for `<video>`, `<audio>`, and `<iframe>`. No longer drops these layout boxes entirely.
- **Image Paint Processing**: Discarded `[IMG]` text fallbacks. We implemented async Tokio blocks to download actual inline images, resize their buffers via `image::imageops::thumbnail`, and copy them dynamically into the canvas struct. 
- **Block vs Inline Layout**: Greatly improved `layout_stage` handling nested inline elements. 
- Simulated scrollbars are now painted dynamically based on vertical offset overflow.

## Summary
The updated testing script (`scripts/test-sites-advanced.ps1`) verifies the browser's capability to route seamlessly between Clearnet header-heavy services and Darknet onion routing, and the desktop GUI app renders layout structures effectively on purely CPU memory bounds. Phase 1 and 1.5 of the current development track have successfully completed.