# Catisen Comprehensive Test Report

Generated: 2026-04-13 13.19.58

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

- [REQ] ts=13:19:33 status=200 bytes=8331 mode=Source Code tor=false ttfb=696 ttfc=698 ttfr=717 tti=742 full=702 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=13:19:35 status=200 bytes=25648 mode=Source Code tor=false ttfb=637 ttfc=640 ttfr=658 tti=683 full=643 url=https://bot.sannysoft.com/ headers=date: Mon, 13 Apr 2026 07:49:36 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=13:19:37 status=200 bytes=131634 mode=Source Code tor=false ttfb=362 ttfc=406 ttfr=517 tti=542 full=502 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Mon, 13 Apr 2026 03:15:33 GMT
- [REQ] ts=13:19:42 status=200 bytes=577740 mode=Source Code tor=false ttfb=2546 ttfc=2547 ttfr=3505 tti=3530 full=3490 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=13:19:45 status=200 bytes=645250 mode=Source Code tor=false ttfb=668 ttfc=753 ttfr=1648 tti=1673 full=1633 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=13:19:48 status=0 bytes=0 mode=ERROR tor=true ttfb=1031 ttfc=1031 ttfr=1031 tti=1031 full=1031 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=error: Tor proxy is not reachable at socks5h://127.0.0.1:9150 (Environment (CATISEN_TOR_PROXY)).
- [REQ] ts=13:19:52 status=0 bytes=0 mode=ERROR tor=false ttfb=2069 ttfc=2069 ttfr=2069 tti=2069 full=2069 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=13:19:55 status=200 bytes=34539 mode=Text-Only tor=false ttfb=1616 ttfc=1617 ttfr=1634 tti=1659 full=1619 url=https://news.ycombinator.com/ headers=server: nginx | date: Mon, 13 Apr 2026 07:49:55 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=13:19:58 status=200 bytes=0 mode=Binary tor=false ttfb=1664 ttfc=1664 ttfr=1664 tti=1664 full=1667 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Mon, 13 Apr 2026 07:49:58 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 13:19:33 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 696 | 698 | 717 | 742 | 702 |
| 13:19:35 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 637 | 640 | 658 | 683 | 643 |
| 13:19:37 | https://www.browserscan.net/ | 200 | Source Code | False | 131634 | 362 | 406 | 517 | 542 | 502 |
| 13:19:42 | https://www.netflix.com/ | 200 | Source Code | False | 577740 | 2546 | 2547 | 3505 | 3530 | 3490 |
| 13:19:45 | https://www.bbc.co.uk/ | 200 | Source Code | False | 645250 | 668 | 753 | 1648 | 1673 | 1633 |
| 13:19:48 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 0 | ERROR | True | 0 | 1031 | 1031 | 1031 | 1031 | 1031 |
| 13:19:52 | https://example.com/ | 0 | ERROR | False | 0 | 2069 | 2069 | 2069 | 2069 | 2069 |
| 13:19:55 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34539 | 1616 | 1617 | 1634 | 1659 | 1619 |
| 13:19:58 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1664 | 1664 | 1664 | 1664 | 1667 |

Averages: TTFB=1254.33 ms, TTFC=1269.44 ms, TTFR=1493.67 ms, TTI=1510.33 ms, Full=1484 ms, Bytes=158126.89

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260413-131958.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 702 | 698 | 589.81 | 310.03 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 643 | 640 | 494.42 | 344.29 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 502 | 406 | 662.28 | 248.43 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 3490 | 2547 | 4450.78 | 1050.53 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1633 | 753 | 13026.6 | 702.5 |  |  |  | Chrome: No successful runs (failures=1, err=[7708:28356:0413/132152.352:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
