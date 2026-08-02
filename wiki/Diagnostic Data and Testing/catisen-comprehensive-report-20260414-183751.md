# Catisen Comprehensive Test Report

Generated: 2026-04-14 18.37.51

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

- [REQ] ts=18:37:26 status=200 bytes=8331 mode=Source Code tor=false ttfb=676 ttfc=678 ttfr=693 tti=718 full=678 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=18:37:29 status=200 bytes=7571 mode=Source Code tor=false ttfb=655 ttfc=655 ttfr=670 tti=695 full=655 url=https://bot.sannysoft.com/ headers=date: Tue, 14 Apr 2026 13:07:27 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=18:37:31 status=200 bytes=131634 mode=Source Code tor=false ttfb=694 ttfc=695 ttfr=718 tti=743 full=703 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 14 Apr 2026 02:57:30 GMT
- [REQ] ts=18:37:36 status=200 bytes=533309 mode=Source Code tor=false ttfb=2224 ttfc=2234 ttfr=3143 tti=3168 full=3128 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=18:37:39 status=200 bytes=694923 mode=Source Code tor=false ttfb=740 ttfc=878 ttfr=1469 tti=1494 full=1454 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=18:37:42 status=0 bytes=0 mode=ERROR tor=true ttfb=1034 ttfc=1034 ttfr=1034 tti=1034 full=1034 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=18:37:44 status=0 bytes=0 mode=ERROR tor=false ttfb=676 ttfc=676 ttfr=676 tti=676 full=676 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=18:37:47 status=200 bytes=35116 mode=Text-Only tor=false ttfb=1641 ttfc=1642 ttfr=1657 tti=1682 full=1642 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 14 Apr 2026 13:07:46 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=18:37:50 status=200 bytes=0 mode=Binary tor=false ttfb=1405 ttfc=1405 ttfr=1405 tti=1405 full=1421 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 14 Apr 2026 13:07:48 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 18:37:26 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 676 | 678 | 693 | 718 | 678 |
| 18:37:29 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 655 | 655 | 670 | 695 | 655 |
| 18:37:31 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 694 | 695 | 718 | 743 | 703 |
| 18:37:36 | https://www.netflix.com/ | 200 | Source Code | False | 533309 | 2224 | 2234 | 3143 | 3168 | 3128 |
| 18:37:39 | https://www.bbc.co.uk/ | 200 | Source Code | False | 694923 | 740 | 878 | 1469 | 1494 | 1454 |
| 18:37:42 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1034 | 1034 | 1034 | 1034 | 1034 |
| 18:37:44 | https://example.com/ | 0 | ERROR | False | 0 | 676 | 676 | 676 | 676 | 676 |
| 18:37:47 | https://news.ycombinator.com/ | 200 | Text-Only | False | 35116 | 1641 | 1642 | 1657 | 1682 | 1642 |
| 18:37:50 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1405 | 1405 | 1405 | 1405 | 1421 |

Averages: TTFB=1082.78 ms, TTFC=1099.67 ms, TTFR=1273.89 ms, TTI=1290.56 ms, Full=1265.67 ms, Bytes=156764.89

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260414-183750.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 678 | 678 | 424.53 | 354.36 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 655 | 655 | 607.03 | 974.77 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 703 | 695 | 660.89 | 297.99 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 3128 | 2234 | 63181.4 | 1079.1 |  |  |  | Chrome: No successful runs (failures=1, err=[15864:16708:0414/184010.556:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1454 | 878 | 6352.71 | 904.61 |  |  |  | Chrome: No successful runs (failures=1, err=[18048:21652:0414/184056.099:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
