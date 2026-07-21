# Catisen Comprehensive Test Report

Generated: 2026-04-07 10.46.37

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

- [REQ] ts=10:46:13 status=200 bytes=8331 mode=Source Code tor=false ttfb=513 ttfc=514 ttfr=530 tti=555 full=515 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=10:46:15 status=200 bytes=25648 mode=Source Code tor=false ttfb=682 ttfc=683 ttfr=701 tti=726 full=686 url=https://bot.sannysoft.com/ headers=date: Tue, 07 Apr 2026 05:16:16 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=10:46:17 status=200 bytes=131635 mode=Source Code tor=false ttfb=597 ttfc=609 ttfr=634 tti=659 full=619 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 07 Apr 2026 04:47:35 GMT
- [REQ] ts=10:46:22 status=200 bytes=574888 mode=Source Code tor=false ttfb=2049 ttfc=2060 ttfr=2839 tti=2864 full=2824 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=10:46:25 status=200 bytes=712734 mode=Source Code tor=false ttfb=596 ttfc=596 ttfr=1193 tti=1218 full=1178 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=10:46:27 status=0 bytes=0 mode=ERROR tor=true ttfb=1011 ttfc=1011 ttfr=1011 tti=1011 full=1011 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=10:46:29 status=0 bytes=0 mode=ERROR tor=false ttfb=519 ttfc=519 ttfr=519 tti=519 full=519 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=10:46:33 status=200 bytes=34500 mode=Text-Only tor=false ttfb=2206 ttfc=2207 ttfr=2224 tti=2249 full=2209 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 07 Apr 2026 05:16:33 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=10:46:36 status=200 bytes=0 mode=Binary tor=false ttfb=1369 ttfc=1369 ttfr=1369 tti=1369 full=1372 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 07 Apr 2026 05:16:36 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 10:46:13 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 513 | 514 | 530 | 555 | 515 |
| 10:46:15 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 682 | 683 | 701 | 726 | 686 |
| 10:46:17 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 597 | 609 | 634 | 659 | 619 |
| 10:46:22 | https://www.netflix.com/ | 200 | Source Code | False | 574888 | 2049 | 2060 | 2839 | 2864 | 2824 |
| 10:46:25 | https://www.bbc.co.uk/ | 200 | Source Code | False | 712734 | 596 | 596 | 1193 | 1218 | 1178 |
| 10:46:27 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1011 | 1011 | 1011 | 1011 | 1011 |
| 10:46:29 | https://example.com/ | 0 | ERROR | False | 0 | 519 | 519 | 519 | 519 | 519 |
| 10:46:33 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34500 | 2206 | 2207 | 2224 | 2249 | 2209 |
| 10:46:36 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1369 | 1369 | 1369 | 1369 | 1372 |

Averages: TTFB=1060.22 ms, TTFC=1063.11 ms, TTFR=1224.44 ms, TTI=1241.11 ms, Full=1214.78 ms, Bytes=165304

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260407-104636.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 515 | 514 | 496.77 | 328.98 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[3964:24500:0407/104658.097:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://bot.sannysoft.com/ | 686 | 683 | 522.23 | 437.38 |  |  |  | Chrome: No successful runs (failures=1, err=[24836:5300:0407/104714.476:ERROR:mojo\public\cpp\bindings\lib\interface_endpoint_client.cc:748] Message 594967305 rejected by interface blink.mojom.FrameWidgetHost); Brave: No successful runs (failures=1, err=[15328:3488:0407/104720.969:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.browserscan.net/ | 619 | 609 | 665.24 | 231.95 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[21844:17060:0407/104737.536:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.netflix.com/ | 2824 | 2060 | 62917.23 | 1045.58 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[7784:21576:0407/104859.756:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1178 | 596 | 8198.58 | 780.18 |  |  |  | Chrome: No successful runs (failures=1, err=[8880:20680:0407/104920.740:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[18532:18160:0407/104940.475:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
