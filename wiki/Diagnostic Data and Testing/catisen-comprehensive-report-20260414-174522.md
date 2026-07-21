# Catisen Comprehensive Test Report

Generated: 2026-04-14 17.45.22

Config: TorProxy=socks5h://127.0.0.1:9150, TargetFps=144

## Scenario Summary

| Case | URL | Tor | Geo | Profile | View | FPS |
|---|---|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | https://abrahamjuliot.github.io/creepjs/ | False | disabled | windows | source | 144 |
| BotSannysoft-Hardened | https://bot.sannysoft.com/ | False | disabled | linux | source | 144 |
| BrowserScan-Android-Profile | https://www.browserscan.net/ | False | singapore | android | source | 144 |
| Geo-Locked-Netflix-Probe | https://www.netflix.com/ | False | switzerland | windows | source | 144 |
| Geo-Locked-BBC-Probe | https://www.bbc.co.uk/ | False | uk | macos | source | 144 |
| Onion-Tor-Probe | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | True | disabled | linux | source | 144 |
| Visual-Mode-Bridge | https://example.com/ | False | netherlands | windows | visual | 144 |
| Text-Mode-Cleaning | https://news.ycombinator.com/ | False | iceland | windows | text | 144 |
| Binary-Download-Interception | https://ash-speed.hetzner.com/100MB.bin | False | disabled | windows | source | 144 |

## Raw Catisen Request Lines

- [REQ] ts=17:45:06 status=200 bytes=8331 mode=Source Code tor=false ttfb=469 ttfc=470 ttfr=486 tti=511 full=471 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=17:45:07 status=200 bytes=7571 mode=Source Code tor=false ttfb=516 ttfc=516 ttfr=532 tti=557 full=517 url=https://bot.sannysoft.com/ headers=date: Tue, 14 Apr 2026 12:15:05 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=17:45:08 status=200 bytes=131634 mode=Source Code tor=false ttfb=651 ttfc=653 ttfr=674 tti=699 full=659 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 14 Apr 2026 02:57:30 GMT
- [REQ] ts=17:45:11 status=200 bytes=577385 mode=Source Code tor=false ttfb=1842 ttfc=1854 ttfr=2740 tti=2765 full=2725 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=17:45:13 status=200 bytes=698574 mode=Source Code tor=false ttfb=445 ttfc=456 ttfr=933 tti=958 full=918 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=17:45:17 status=200 bytes=4727 mode=Source Code tor=true ttfb=934 ttfc=935 ttfr=950 tti=975 full=935 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=server: nginx/1.22.1 | date: Tue, 14 Apr 2026 12:15:15 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=17:45:18 status=0 bytes=0 mode=ERROR tor=false ttfb=666 ttfc=666 ttfr=666 tti=666 full=666 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=17:45:20 status=200 bytes=34967 mode=Text-Only tor=false ttfb=1564 ttfc=1565 ttfr=1581 tti=1606 full=1566 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 14 Apr 2026 12:15:18 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=17:45:21 status=200 bytes=0 mode=Binary tor=false ttfb=1247 ttfc=1247 ttfr=1247 tti=1247 full=1250 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 14 Apr 2026 12:15:20 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 17:45:06 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 469 | 470 | 486 | 511 | 471 |
| 17:45:07 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 516 | 516 | 532 | 557 | 517 |
| 17:45:08 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 651 | 653 | 674 | 699 | 659 |
| 17:45:11 | https://www.netflix.com/ | 200 | Source Code | False | 577385 | 1842 | 1854 | 2740 | 2765 | 2725 |
| 17:45:13 | https://www.bbc.co.uk/ | 200 | Source Code | False | 698574 | 445 | 456 | 933 | 958 | 918 |
| 17:45:17 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 200 | Source Code | True | 4727 | 934 | 935 | 950 | 975 | 935 |
| 17:45:18 | https://example.com/ | 0 | ERROR | False | 0 | 666 | 666 | 666 | 666 | 666 |
| 17:45:20 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34967 | 1564 | 1565 | 1581 | 1606 | 1566 |
| 17:45:21 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1247 | 1247 | 1247 | 1247 | 1250 |

Averages: TTFB=926 ms, TTFC=929.11 ms, TTFR=1089.89 ms, TTI=1109.33 ms, Full=1078.56 ms, Bytes=162576.56

## Tor + Geo + Fingerprint Assertion Gates

| Case | ReqCfgFound | ReqFound | Status | Notes |
|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | True | True | PASS | All gates satisfied |
| BotSannysoft-Hardened | True | True | PASS | All gates satisfied |
| BrowserScan-Android-Profile | True | True | PASS | All gates satisfied |
| Geo-Locked-Netflix-Probe | True | True | PASS | All gates satisfied |
| Geo-Locked-BBC-Probe | True | True | PASS | All gates satisfied |
| Onion-Tor-Probe | True | True | PASS | All gates satisfied |
| Visual-Mode-Bridge | True | True | PASS | All gates satisfied |
| Text-Mode-Cleaning | True | True | PASS | All gates satisfied |
| Binary-Download-Interception | True | True | PASS | All gates satisfied |

Gate Summary: PASS=9, FAIL=0

## Binary Download Assertion

| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |
|---|---|---|---|---|---|
| True | 1 | 1 | 0 | PASS | Download handoff log found and new file(s) created. |

New downloaded files:
- C:\Users\iassh\catisen\target\downloads\100MB-20260414-174521.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 471 | 470 | 470.46 | 349.19 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 517 | 516 | 498.16 | 473.75 |  |  |  | Chrome: No successful runs (failures=1, err=[8604:8348:0414/174551.709:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://mail.google.com/chat/download?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 659 | 653 | 437.36 | 226.62 |  |  |  | Chrome: No successful runs (failures=1, err=[23256:23140:0414/174609.833:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://mail.google.com/mail/installwebapp?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 2725 | 1854 | 4927.08 | 974.3 |  |  |  | Chrome: No successful runs (failures=1, err=[6116:16372:0414/174639.440:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 918 | 456 | 3196 | 897.4 |  |  |  | Chrome: No successful runs (failures=1, err=[8236:1040:0414/174725.562:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
