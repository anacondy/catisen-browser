# 2 nd April Diagnose

Date: 2026-04-02
Project: Catisen
Scope: Runtime diagnostics, Tor routing, visual mode behavior, high-refresh pacing, and network performance baselines.

## 1) Build and Test Configuration
- Build profile: `cargo build` (`dev`)
- UI mode: `egui_ui`
- Target FPS used in tests: `144`
- Tor proxy used in tests: `socks5h://127.0.0.1:9150` (Tor Browser)

## 2) Test Assets and Commands Used
- Comprehensive matrix: `scripts/test-comprehensive.ps1`
  - Output: `target/catisen-comprehensive-report-20260402-230116.md`
- Tor onion matrix: `scripts/test-tor-site-list.ps1`
  - Output: `target/catisen-tor-sites-report-20260402-230653.md`

## 3) Clearnet Diagnostics (Parsed from Ctrl+D-equivalent [REQ] data)
Source report: `target/catisen-comprehensive-report-20260402-230116.md`

Averages (6 successful rows):
- TTFB: 691.17 ms
- TTFC: 738.00 ms
- TTFR: 2003.17 ms
- TTI: 2028.17 ms
- Full Load: 1988.17 ms

Per-site highlights:
- CreepJS: TTFC 456 ms, Full 461 ms
- BotSannysoft: TTFC 508 ms, Full 653 ms
- BrowserScan: TTFC 726 ms, Full 898 ms
- Netflix: TTFC 1831 ms, Full 3645 ms
- BBC: TTFC 611 ms, Full 5976 ms
- Visual mode probe (`example.com`): status 200, mode `Visual`, Full 296 ms

## 4) Darknet / Tor Diagnostics
Source report: `target/catisen-tor-sites-report-20260402-230653.md`

Summary:
- Total cases: 20
- Cases with telemetry [REQ]: 7
- Successful fetches (`status>0` and `bytes>0`): 5

Successful examples (source mode):
- DuckDuckGo onion: status 200, bytes 160963, TTFC 5872 ms, Full 8996 ms
- Ahmia onion: status 200, bytes 4727, TTFC 4678 ms, Full 4796 ms
- Torch onion: status 200, bytes 68, TTFC 4955 ms, Full 4956 ms
- OnionShare onion: status 200, bytes 11230, TTFC 7571 ms, Full 7804 ms
- F-Droid onion: status 200, bytes 14862, TTFC 5044 ms, Full 5100 ms

Observed Tor route metadata from raw debug logs:
- `IsTor=yes`
- Exit IP observed: `185.220.101.17`
- Exit country observed: `Germany`

## 5) Category Scores (0-10)
- Network pipeline stability: 7.6/10
  - Good clearnet completion rates and rich telemetry.
  - Some long-tail latency spikes on heavy pages.
- Tor routing robustness: 6.3/10
  - Correct Tor confirmation and exit-country reporting.
  - Mixed success across onion targets (expected due host availability/handshake variance).
- Diagnostics quality (Ctrl+D + log pipeline): 8.5/10
  - Strong coverage: TTFB/TTFC/TTFR/TTI/Full, headers, Tor route info, speed indicators.
- Visual mode usability: 6.4/10
  - Better than before (real visual pipeline path exists), but still snapshot/fallback class, not interactive DOM.
- High refresh/high FPS readiness: 7.9/10
  - Target FPS pacing implemented (90/100/120/144/150+), FPS now visible in diagnostics.
- Automation and regression readiness: 8.2/10
  - Two strong matrix scripts, timestamped reports, and reproducible baseline workflow.

## 6) Why Visual Mode Can Still Look Like Text
Current visual mode is a robust transitional renderer:
- It first tries browser-backed snapshot capture.
- If snapshot capture is unavailable for the target runtime, it falls back to preview text.
- This is expected until full interactive Servo paint/composition is shipped.

## 7) What Was Improved in This Cycle
- Added Tor proxy auto-detection (`9050`/`9150`) with environment override.
- Added Tor route diagnostics (proxy, Tor confirmation, exit IP, exit country).
- Improved rolling live speed calculation and average request speed fallback.
- Added high-refresh target FPS controls and measured FPS display in diagnostics.
- Improved test scripts to wait for telemetry rather than fixed blind timeout.

## 8) Remaining Priority Gaps
- Full interactive visual rendering (true Servo DOM/CSS paint output).
- Runtime-context anti-fingerprint hooks (beyond injected source payload).
- More resilient browser baseline path for Chrome/Brave on this machine.
- Stronger Tor test orchestration for text-mode completion coverage across all onion targets.

## 9) Baseline Comparison Guidance for Future Versions
Use these as consistent anchors in future runs:
- `scripts/test-comprehensive.ps1 -RunSecondsPerCase 14 -MaxCases 7 -BrowserRunsPerSite 1 -TorProxy "socks5h://127.0.0.1:9150" -TargetFps 144`
- `scripts/test-tor-site-list.ps1 -RunSecondsPerCase 16 -TorProxy "socks5h://127.0.0.1:9150" -TargetFps 144`

Compare deltas for:
- Avg TTFC
- Avg Full Load
- Tor success ratio (`successful fetches / total cases`)
- Visual-mode success rate
- FPS stability at selected refresh targets
