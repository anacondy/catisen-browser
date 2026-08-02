# Catisen Comprehensive Test Report

Generated: 2026-04-13 18.12.11

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

- [REQ] ts=18:11:44 status=200 bytes=8331 mode=Source Code tor=false ttfb=712 ttfc=714 ttfr=735 tti=760 full=720 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=18:11:47 status=200 bytes=25648 mode=Source Code tor=false ttfb=647 ttfc=653 ttfr=674 tti=699 full=659 url=https://bot.sannysoft.com/ headers=date: Mon, 13 Apr 2026 12:41:47 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=18:11:49 status=200 bytes=131634 mode=Source Code tor=false ttfb=527 ttfc=533 ttfr=554 tti=579 full=539 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Mon, 13 Apr 2026 03:15:33 GMT
- [REQ] ts=18:11:53 status=200 bytes=533227 mode=Source Code tor=false ttfb=2296 ttfc=2296 ttfr=3070 tti=3095 full=3055 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=18:11:58 status=200 bytes=674424 mode=Source Code tor=false ttfb=2041 ttfc=2173 ttfr=2836 tti=2861 full=2821 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: HIT
- [REQ] ts=18:12:00 status=0 bytes=0 mode=ERROR tor=true ttfb=1051 ttfc=1051 ttfr=1051 tti=1051 full=1051 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=18:12:03 status=0 bytes=0 mode=ERROR tor=false ttfb=1401 ttfc=1401 ttfr=1401 tti=1401 full=1401 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=18:12:07 status=200 bytes=34569 mode=Text-Only tor=false ttfb=1822 ttfc=1824 ttfr=1848 tti=1873 full=1833 url=https://news.ycombinator.com/ headers=server: nginx | date: Mon, 13 Apr 2026 12:42:07 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=18:12:10 status=200 bytes=0 mode=Binary tor=false ttfb=1773 ttfc=1773 ttfr=1773 tti=1773 full=1778 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Mon, 13 Apr 2026 12:42:10 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 18:11:44 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 712 | 714 | 735 | 760 | 720 |
| 18:11:47 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 647 | 653 | 674 | 699 | 659 |
| 18:11:49 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 527 | 533 | 554 | 579 | 539 |
| 18:11:53 | https://www.netflix.com/ | 200 | Source Code | False | 533227 | 2296 | 2296 | 3070 | 3095 | 3055 |
| 18:11:58 | https://www.bbc.co.uk/ | 200 | Source Code | False | 674424 | 2041 | 2173 | 2836 | 2861 | 2821 |
| 18:12:00 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1051 | 1051 | 1051 | 1051 | 1051 |
| 18:12:03 | https://example.com/ | 0 | ERROR | False | 0 | 1401 | 1401 | 1401 | 1401 | 1401 |
| 18:12:07 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34569 | 1822 | 1824 | 1848 | 1873 | 1833 |
| 18:12:10 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1773 | 1773 | 1773 | 1773 | 1778 |

Averages: TTFB=1363.33 ms, TTFC=1379.78 ms, TTFR=1549.11 ms, TTI=1565.78 ms, Full=1539.67 ms, Bytes=156425.89

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260413-181210.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 720 | 714 | 961.48 | 326.67 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 659 | 653 | 442.74 | 374.63 |  |  |  | Chrome: No successful runs (failures=1, err=[12904:11796:0413/181233.768:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 539 | 533 | 501.33 | 331.41 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 3055 | 2296 | 5794.61 | 1051.55 |  |  |  | Chrome: No successful runs (failures=1, err=[19208:4644:0413/181317.268:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 2821 | 2173 | 8361.25 | 697.11 |  |  |  | Chrome: No successful runs (failures=1, err=[18476:3872:0413/181353.912:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
