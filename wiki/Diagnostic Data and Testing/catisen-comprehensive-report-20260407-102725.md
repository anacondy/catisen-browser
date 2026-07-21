# Catisen Comprehensive Test Report

Generated: 2026-04-07 10.27.25

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

- [REQ] ts=10:27:01 status=200 bytes=8331 mode=Source Code tor=false ttfb=344 ttfc=344 ttfr=361 tti=386 full=346 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=10:27:03 status=200 bytes=25648 mode=Source Code tor=false ttfb=471 ttfc=472 ttfr=490 tti=515 full=475 url=https://bot.sannysoft.com/ headers=date: Tue, 07 Apr 2026 04:57:03 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=10:27:05 status=200 bytes=131635 mode=Source Code tor=false ttfb=599 ttfc=600 ttfr=621 tti=646 full=606 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 07 Apr 2026 04:47:35 GMT
- [REQ] ts=10:27:10 status=200 bytes=574874 mode=Source Code tor=false ttfb=2266 ttfc=2281 ttfr=3072 tti=3097 full=3057 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=10:27:14 status=200 bytes=710832 mode=Source Code tor=false ttfb=795 ttfc=926 ttfr=2091 tti=2116 full=2076 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=10:27:17 status=0 bytes=0 mode=ERROR tor=true ttfb=1058 ttfc=1058 ttfr=1058 tti=1058 full=1058 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=10:27:19 status=0 bytes=0 mode=ERROR tor=false ttfb=667 ttfc=667 ttfr=667 tti=667 full=667 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=10:27:22 status=200 bytes=34496 mode=Text-Only tor=false ttfb=1764 ttfc=1765 ttfr=1781 tti=1806 full=1766 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 07 Apr 2026 04:57:23 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=10:27:25 status=200 bytes=0 mode=Binary tor=false ttfb=1186 ttfc=1186 ttfr=1186 tti=1186 full=1188 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 07 Apr 2026 04:57:25 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 10:27:01 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 344 | 344 | 361 | 386 | 346 |
| 10:27:03 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 471 | 472 | 490 | 515 | 475 |
| 10:27:05 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 599 | 600 | 621 | 646 | 606 |
| 10:27:10 | https://www.netflix.com/ | 200 | Source Code | False | 574874 | 2266 | 2281 | 3072 | 3097 | 3057 |
| 10:27:14 | https://www.bbc.co.uk/ | 200 | Source Code | False | 710832 | 795 | 926 | 2091 | 2116 | 2076 |
| 10:27:17 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1058 | 1058 | 1058 | 1058 | 1058 |
| 10:27:19 | https://example.com/ | 0 | ERROR | False | 0 | 667 | 667 | 667 | 667 | 667 |
| 10:27:22 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34496 | 1764 | 1765 | 1781 | 1806 | 1766 |
| 10:27:25 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1186 | 1186 | 1186 | 1186 | 1188 |

Averages: TTFB=1016.67 ms, TTFC=1033.22 ms, TTFR=1258.56 ms, TTI=1275.22 ms, Full=1248.78 ms, Bytes=165090.67

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
| True | 1 | 1 | 0 | FAIL_NO_HANDOFF | No handoff log and no new downloaded file detected. |

New downloaded files:
- C:\Users\iassh\catisen\target\downloads\100MB.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 346 | 344 | 409.08 | 414.31 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[11556:23968:0407/102735.806:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://bot.sannysoft.com/ | 475 | 472 | 681.38 | 439.81 |  |  |  | Chrome: No successful runs (failures=1, err=[11740:1344:0407/102743.393:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[15540:18804:0407/102802.123:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.browserscan.net/ | 606 | 600 | 1251.36 | 231.11 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[14812:11604:0407/102819.878:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.netflix.com/ | 3057 | 2281 | 55529.22 | 1093.47 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[11924:21844:0407/102933.168:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.bbc.co.uk/ | 2076 | 926 | 6789.48 | 291.34 |  |  |  | Chrome: No successful runs (failures=1, err=[9076:23596:0407/102949.765:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[6844:18920:0407/103009.317:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
