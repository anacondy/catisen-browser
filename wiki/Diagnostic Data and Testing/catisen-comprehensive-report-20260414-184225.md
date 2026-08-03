# Catisen Comprehensive Test Report

Generated: 2026-04-14 18.42.25

Config: TorProxy=socks5h://127.0.0.1:9150, TargetFps=144

## Scenario Summary

| Case | URL | Tor | Geo | Profile | View | FPS |
|---|---|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | https://abrahamjuliot.github.io/creepjs/ | False | disabled | windows | source | 144 |
| BotSannysoft-Hardened | https://bot.sannysoft.com/ | False | disabled | linux | source | 144 |
| BrowserScan-Android-Profile | https://www.browserscan.net/ | False | singapore | android | source | 144 |

## Raw Catisen Request Lines

- [REQ] ts=18:42:21 status=200 bytes=8331 mode=Source Code tor=false ttfb=365 ttfc=367 ttfr=382 tti=407 full=367 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=18:42:23 status=200 bytes=7571 mode=Source Code tor=false ttfb=426 ttfc=426 ttfr=442 tti=467 full=427 url=https://bot.sannysoft.com/ headers=date: Tue, 14 Apr 2026 13:12:21 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=18:42:25 status=200 bytes=131634 mode=Source Code tor=false ttfb=353 ttfc=354 ttfr=371 tti=396 full=356 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 14 Apr 2026 02:57:30 GMT

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 18:42:21 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 365 | 367 | 382 | 407 | 367 |
| 18:42:23 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 426 | 426 | 442 | 467 | 427 |
| 18:42:25 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 353 | 354 | 371 | 396 | 356 |

Averages: TTFB=381.33 ms, TTFC=382.33 ms, TTFR=398.33 ms, TTI=423.33 ms, Full=383.33 ms, Bytes=49178.67

## Tor + Geo + Fingerprint Assertion Gates

| Case | ReqCfgFound | ReqFound | Status | Notes |
|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | True | True | PASS | All gates satisfied |
| BotSannysoft-Hardened | True | True | PASS | All gates satisfied |
| BrowserScan-Android-Profile | True | True | PASS | All gates satisfied |

Gate Summary: PASS=3, FAIL=0

## Binary Download Assertion

| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |
|---|---|---|---|---|---|
| False | 0 | 0 | 0 | N/A | Binary case not included in this run. |

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 367 | 367 | 299.41 | 254.58 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 427 | 426 | 772.81 | 667.44 |  |  |  | Chrome: No successful runs (failures=1, err=[8408:18672:0414/184249.790:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://mail.google.com/chat/download?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 356 | 354 | 611.48 | 226.58 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
