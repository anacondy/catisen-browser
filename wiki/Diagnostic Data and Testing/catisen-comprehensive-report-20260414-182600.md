# Catisen Comprehensive Test Report

Generated: 2026-04-14 18.26.00

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

- [REQ] ts=18:25:37 status=200 bytes=8331 mode=Source Code tor=false ttfb=544 ttfc=545 ttfr=560 tti=585 full=545 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=18:25:39 status=200 bytes=7571 mode=Source Code tor=false ttfb=668 ttfc=668 ttfr=683 tti=708 full=668 url=https://bot.sannysoft.com/ headers=date: Tue, 14 Apr 2026 12:55:38 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=18:25:41 status=200 bytes=131634 mode=Source Code tor=false ttfb=362 ttfc=363 ttfr=380 tti=405 full=365 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Tue, 14 Apr 2026 02:57:30 GMT
- [REQ] ts=18:25:45 status=200 bytes=577379 mode=Source Code tor=false ttfb=1787 ttfc=1799 ttfr=2603 tti=2628 full=2588 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=18:25:48 status=200 bytes=698201 mode=Source Code tor=false ttfb=387 ttfc=416 ttfr=748 tti=773 full=733 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=18:25:50 status=0 bytes=0 mode=ERROR tor=true ttfb=1045 ttfc=1045 ttfr=1045 tti=1045 full=1045 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=18:25:53 status=0 bytes=0 mode=ERROR tor=false ttfb=765 ttfc=765 ttfr=765 tti=765 full=765 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=18:25:56 status=200 bytes=34970 mode=Text-Only tor=false ttfb=1867 ttfc=1868 ttfr=1884 tti=1909 full=1869 url=https://news.ycombinator.com/ headers=server: nginx | date: Tue, 14 Apr 2026 12:55:55 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=18:25:59 status=200 bytes=0 mode=Binary tor=false ttfb=1432 ttfc=1432 ttfr=1432 tti=1432 full=1441 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Tue, 14 Apr 2026 12:55:57 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 18:25:37 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 544 | 545 | 560 | 585 | 545 |
| 18:25:39 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 668 | 668 | 683 | 708 | 668 |
| 18:25:41 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 362 | 363 | 380 | 405 | 365 |
| 18:25:45 | https://www.netflix.com/ | 200 | Source Code | False | 577379 | 1787 | 1799 | 2603 | 2628 | 2588 |
| 18:25:48 | https://www.bbc.co.uk/ | 200 | Source Code | False | 698201 | 387 | 416 | 748 | 773 | 733 |
| 18:25:50 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1045 | 1045 | 1045 | 1045 | 1045 |
| 18:25:53 | https://example.com/ | 0 | ERROR | False | 0 | 765 | 765 | 765 | 765 | 765 |
| 18:25:56 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34970 | 1867 | 1868 | 1884 | 1909 | 1869 |
| 18:25:59 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1432 | 1432 | 1432 | 1432 | 1441 |

Averages: TTFB=984.11 ms, TTFC=989 ms, TTFR=1122.22 ms, TTI=1138.89 ms, Full=1113.22 ms, Bytes=162009.56

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260414-182559.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 545 | 545 | 536.11 | 277.35 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 668 | 668 | 441.6 | 452.81 |  |  |  | Chrome: No successful runs (failures=1, err=[2636:14120:0414/182626.681:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 365 | 363 | 366.29 | 224.24 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 2588 | 1799 | 4614.39 | 965.62 |  |  |  | Chrome: No successful runs (failures=1, err=[11976:12160:0414/182709.825:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 733 | 416 | 4468.54 | 1019.16 |  |  |  | Chrome: No successful runs (failures=1, err=[16148:5944:0414/182738.388:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://docs.google.com/presentation/installwebapp?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
