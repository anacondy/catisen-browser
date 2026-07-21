# Catisen Comprehensive Test Report

Generated: 2026-04-07 10.30.43

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

- [REQ] ts=10:30:21 status=200 bytes=8331 mode=Source Code tor=false ttfb=496 ttfc=497 ttfr=514 tti=539 full=499 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=10:30:23 status=200 bytes=25648 mode=Source Code tor=false ttfb=512 ttfc=513 ttfr=531 tti=556 full=516 url=https://bot.sannysoft.com/ headers=date: Tue, 07 Apr 2026 05:00:24 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=10:30:25 status=200 bytes=131635 mode=Source Code tor=false ttfb=665 ttfc=665 ttfr=694 tti=719 full=679 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Mon, 06 Apr 2026 06:36:07 GMT
- [REQ] ts=10:30:29 status=200 bytes=574874 mode=Source Code tor=false ttfb=1693 ttfc=1694 ttfr=2099 tti=2124 full=2084 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=10:30:32 status=200 bytes=710832 mode=Source Code tor=false ttfb=483 ttfc=722 ttfr=1181 tti=1206 full=1166 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=10:30:34 status=0 bytes=0 mode=ERROR tor=true ttfb=1027 ttfc=1027 ttfr=1027 tti=1027 full=1027 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=10:30:36 status=0 bytes=0 mode=ERROR tor=false ttfb=606 ttfc=606 ttfr=606 tti=606 full=606 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=10:30:39 status=200 bytes=34496 mode=Text-Only tor=false ttfb=1700 ttfc=1701 ttfr=1718 tti=1743 full=1703 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 07 Apr 2026 05:00:40 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=10:30:42 status=200 bytes=0 mode=Binary tor=false ttfb=1157 ttfc=1157 ttfr=1157 tti=1157 full=1160 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 07 Apr 2026 05:00:43 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 10:30:21 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 496 | 497 | 514 | 539 | 499 |
| 10:30:23 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 512 | 513 | 531 | 556 | 516 |
| 10:30:25 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 665 | 665 | 694 | 719 | 679 |
| 10:30:29 | https://www.netflix.com/ | 200 | Source Code | False | 574874 | 1693 | 1694 | 2099 | 2124 | 2084 |
| 10:30:32 | https://www.bbc.co.uk/ | 200 | Source Code | False | 710832 | 483 | 722 | 1181 | 1206 | 1166 |
| 10:30:34 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1027 | 1027 | 1027 | 1027 | 1027 |
| 10:30:36 | https://example.com/ | 0 | ERROR | False | 0 | 606 | 606 | 606 | 606 | 606 |
| 10:30:39 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34496 | 1700 | 1701 | 1718 | 1743 | 1703 |
| 10:30:42 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1157 | 1157 | 1157 | 1157 | 1160 |

Averages: TTFB=926.56 ms, TTFC=953.56 ms, TTFR=1058.56 ms, TTI=1075.22 ms, Full=1048.89 ms, Bytes=165090.67

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260407-103042.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 499 | 497 | 462.77 | 233.74 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[14308:22688:0407/103053.173:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://bot.sannysoft.com/ | 516 | 513 | 433.81 | 514.6 |  |  |  | Chrome: No successful runs (failures=1, err=[12684:24628:0407/103104.789:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[15704:6656:0407/103114.538:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.browserscan.net/ | 679 | 665 | 691.16 | 370.58 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[23288:2240:0407/103130.784:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.netflix.com/ | 2084 | 1694 | 4655.98 | 1143 |  |  |  | Chrome: No successful runs (failures=1, err=[21996:22048:0407/103154.198:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[16288:12164:0407/103204.164:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1166 | 722 | 5883.58 | 817.07 |  |  |  | Chrome: No successful runs (failures=1, err=[23124:17116:0407/103242.982:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[20524:24908:0407/103308.061:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
