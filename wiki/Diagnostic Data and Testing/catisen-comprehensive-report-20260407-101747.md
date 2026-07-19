# Catisen Comprehensive Test Report

Generated: 2026-04-07 10.17.47

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
| Binary-Download-Interception | https://speed.hetzner.de/100MB.bin | False | disabled | windows | source | 144 |

## Raw Catisen Request Lines

- [REQ] ts=10:17:28 status=200 bytes=8331 mode=Source Code tor=false ttfb=676 ttfc=677 ttfr=696 tti=721 full=681 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=10:17:30 status=200 bytes=25648 mode=Source Code tor=false ttfb=694 ttfc=695 ttfr=715 tti=740 full=700 url=https://bot.sannysoft.com/ headers=date: Tue, 07 Apr 2026 04:47:31 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=10:17:35 status=200 bytes=131635 mode=Source Code tor=false ttfb=2499 ttfc=2500 ttfr=2530 tti=2555 full=2515 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 07 Apr 2026 04:47:35 GMT
- [REQ] ts=10:17:39 status=200 bytes=574870 mode=Source Code tor=false ttfb=2124 ttfc=2142 ttfr=2980 tti=3005 full=2965 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=10:17:42 status=200 bytes=710755 mode=Source Code tor=false ttfb=663 ttfc=772 ttfr=1388 tti=1413 full=1373 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=10:17:45 status=0 bytes=0 mode=ERROR tor=true ttfb=1159 ttfc=1159 ttfr=1159 tti=1159 full=1159 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 10:17:28 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 676 | 677 | 696 | 721 | 681 |
| 10:17:30 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 694 | 695 | 715 | 740 | 700 |
| 10:17:35 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 2499 | 2500 | 2530 | 2555 | 2515 |
| 10:17:39 | https://www.netflix.com/ | 200 | Source Code | False | 574870 | 2124 | 2142 | 2980 | 3005 | 2965 |
| 10:17:42 | https://www.bbc.co.uk/ | 200 | Source Code | False | 710755 | 663 | 772 | 1388 | 1413 | 1373 |
| 10:17:45 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1159 | 1159 | 1159 | 1159 | 1159 |

Averages: TTFB=1302.5 ms, TTFC=1324.17 ms, TTFR=1578 ms, TTI=1598.83 ms, Full=1565.5 ms, Bytes=241873.17

## Tor + Geo + Fingerprint Assertion Gates

| Case | ReqCfgFound | ReqFound | Status | Notes |
|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | True | True | PASS | All gates satisfied |
| BotSannysoft-Hardened | True | True | PASS | All gates satisfied |
| BrowserScan-Android-Profile | True | True | PASS | All gates satisfied |
| Geo-Locked-Netflix-Probe | True | True | PASS | All gates satisfied |
| Geo-Locked-BBC-Probe | True | True | PASS | All gates satisfied |
| Onion-Tor-Probe | True | True | PASS | All gates satisfied |
| Visual-Mode-Bridge | False | False | FAIL | missing REQCFG; missing REQ |
| Text-Mode-Cleaning | False | False | FAIL | missing REQCFG; missing REQ |
| Binary-Download-Interception | False | False | FAIL | missing REQCFG; missing REQ |

Gate Summary: PASS=6, FAIL=3

## Binary Download Assertion

| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |
|---|---|---|---|---|---|
| True | 0 | 0 | 0 | FAIL_NO_HANDOFF | No handoff log and no new downloaded file detected. |

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 681 | 677 | 672.58 | 304.35 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[11424:21348:0407/101759.925:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://bot.sannysoft.com/ | 700 | 695 | 453.07 | 381.63 |  |  |  | Chrome: No successful runs (failures=1, err=[3192:25016:0407/101806.653:ERROR:chrome\browser\web_applications\os_integration\os_integration_manager.cc:258] Can't perform OS integration while the browser is shutting down.); Brave: No successful runs (failures=1, err=[23768:21180:0407/101823.112:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.browserscan.net/ | 2515 | 2500 | 496.55 | 221.82 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[11596:17896:0407/101838.227:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.netflix.com/ | 2965 | 2142 | 4630.69 | 1398.32 |  |  |  | Chrome: No successful runs (failures=1, err=[17556:8880:0407/101902.545:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1, err=[8352:21104:0407/101909.838:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1373 | 772 | 6056.54 | 315.13 |  |  |  | Chrome: No successful runs (failures=1, err=[17376:21452:0407/101942.025:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[14092:15328:0407/102001.039:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
