# Catisen Comprehensive Test Report

Generated: 2026-04-07 10.24.13

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

- [REQ] ts=10:23:51 status=200 bytes=8331 mode=Source Code tor=false ttfb=267 ttfc=267 ttfr=284 tti=309 full=269 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=10:23:53 status=200 bytes=25648 mode=Source Code tor=false ttfb=818 ttfc=818 ttfr=836 tti=861 full=821 url=https://bot.sannysoft.com/ headers=date: Tue, 07 Apr 2026 04:53:54 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=10:23:56 status=200 bytes=131635 mode=Source Code tor=false ttfb=692 ttfc=700 ttfr=1035 tti=1060 full=1020 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 07 Apr 2026 04:47:35 GMT
- [REQ] ts=10:24:00 status=200 bytes=574882 mode=Source Code tor=false ttfb=2210 ttfc=2210 ttfr=3062 tti=3087 full=3047 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=10:24:03 status=200 bytes=710755 mode=Source Code tor=false ttfb=669 ttfc=999 ttfr=1595 tti=1620 full=1580 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=10:24:06 status=0 bytes=0 mode=ERROR tor=true ttfb=1024 ttfc=1024 ttfr=1024 tti=1024 full=1024 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=10:24:07 status=0 bytes=0 mode=ERROR tor=false ttfb=637 ttfc=637 ttfr=637 tti=637 full=637 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=10:24:10 status=200 bytes=34495 mode=Text-Only tor=false ttfb=1549 ttfc=1549 ttfr=1566 tti=1591 full=1551 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 07 Apr 2026 04:54:11 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=10:24:13 status=0 bytes=0 mode=ERROR tor=false ttfb=1017 ttfc=1017 ttfr=1017 tti=1017 full=1017 url=https://speed.hetzner.de/100MB.bin headers=error: Network request failed for https://speed.hetzner.de/100MB.bin on attempt 2/2: error sending request for url (https://speed.hetzner.de/100MB.bin)

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 10:23:51 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 267 | 267 | 284 | 309 | 269 |
| 10:23:53 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 818 | 818 | 836 | 861 | 821 |
| 10:23:56 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 692 | 700 | 1035 | 1060 | 1020 |
| 10:24:00 | https://www.netflix.com/ | 200 | Source Code | False | 574882 | 2210 | 2210 | 3062 | 3087 | 3047 |
| 10:24:03 | https://www.bbc.co.uk/ | 200 | Source Code | False | 710755 | 669 | 999 | 1595 | 1620 | 1580 |
| 10:24:06 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1024 | 1024 | 1024 | 1024 | 1024 |
| 10:24:07 | https://example.com/ | 0 | ERROR | False | 0 | 637 | 637 | 637 | 637 | 637 |
| 10:24:10 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34495 | 1549 | 1549 | 1566 | 1591 | 1551 |
| 10:24:13 | https://speed.hetzner.de/100MB.bin | 0 | ERROR | False | 0 | 1017 | 1017 | 1017 | 1017 | 1017 |

Averages: TTFB=987 ms, TTFC=1024.56 ms, TTFR=1228.44 ms, TTI=1245.11 ms, Full=1218.44 ms, Bytes=165082.89

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
| True | 0 | 0 | 0 | FAIL_NO_HANDOFF | No handoff log and no new downloaded file detected. |

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 269 | 267 | 242.41 | 343.31 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[5800:19564:0407/102425.167:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://bot.sannysoft.com/ | 821 | 818 | 441.27 | 399.3 |  |  |  | Chrome: No successful runs (failures=1, err=[16472:7384:0407/102439.318:ERROR:chrome\browser\web_applications\os_integration\os_integration_manager.cc:258] Can't perform OS integration while the browser is shutting down.); Brave: No successful runs (failures=1, err=[3976:13572:0407/102445.786:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.browserscan.net/ | 1020 | 700 | 1045.61 | 254.54 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[18776:16488:0407/102501.104:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.netflix.com/ | 3047 | 2210 | 50809.53 | 1086 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1, err=[13104:17108:0407/102619.587:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1580 | 999 | 5335.06 | 1185.49 |  |  |  | Chrome: No successful runs (failures=1, err=[25224:22388:0407/102648.515:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1, err=[11716:10760:0407/102706.043:ERROR:chrome\browser\ui\views\user_education\impl\browser_user_education_interface_impl.cc:154] Attempting to show IPH IPH_DiscardRing before browser initialization complete; IPH will not be shown.); Unavailable on Windows |
