# Catisen Comprehensive Test Report

Generated: 2026-04-12 14.08.45

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

- [REQ] ts=14:07:28 status=200 bytes=8331 mode=Source Code tor=false ttfb=882 ttfc=884 ttfr=908 tti=933 full=893 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=14:07:33 status=200 bytes=25648 mode=Source Code tor=false ttfb=1568 ttfc=1572 ttfr=1591 tti=1616 full=1576 url=https://bot.sannysoft.com/ headers=date: Sun, 12 Apr 2026 08:37:34 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=14:07:37 status=200 bytes=131635 mode=Source Code tor=false ttfb=798 ttfc=806 ttfr=959 tti=984 full=944 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Sun, 12 Apr 2026 05:12:37 GMT
- [REQ] ts=14:07:42 status=200 bytes=533639 mode=Source Code tor=false ttfb=1909 ttfc=1920 ttfr=2666 tti=2691 full=2651 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=14:07:47 status=200 bytes=722499 mode=Source Code tor=false ttfb=581 ttfc=692 ttfr=1052 tti=1077 full=1037 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=14:08:27 status=200 bytes=4727 mode=Source Code tor=true ttfb=3130 ttfc=3130 ttfr=3147 tti=3172 full=3132 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=server: nginx/1.22.1 | date: Sun, 12 Apr 2026 08:38:26 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=14:08:34 status=0 bytes=0 mode=ERROR tor=false ttfb=1919 ttfc=1919 ttfr=1919 tti=1919 full=1919 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=14:08:38 status=200 bytes=34566 mode=Text-Only tor=false ttfb=2042 ttfc=2043 ttfr=2061 tti=2086 full=2046 url=https://news.ycombinator.com/ headers=server: nginx | date: Sun, 12 Apr 2026 08:38:39 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=14:08:43 status=200 bytes=0 mode=Binary tor=false ttfb=1630 ttfc=1630 ttfr=1630 tti=1630 full=1782 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Sun, 12 Apr 2026 08:38:43 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 14:07:28 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 882 | 884 | 908 | 933 | 893 |
| 14:07:33 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 1568 | 1572 | 1591 | 1616 | 1576 |
| 14:07:37 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 798 | 806 | 959 | 984 | 944 |
| 14:07:42 | https://www.netflix.com/ | 200 | Source Code | False | 533639 | 1909 | 1920 | 2666 | 2691 | 2651 |
| 14:07:47 | https://www.bbc.co.uk/ | 200 | Source Code | False | 722499 | 581 | 692 | 1052 | 1077 | 1037 |
| 14:08:27 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 200 | Source Code | True | 4727 | 3130 | 3130 | 3147 | 3172 | 3132 |
| 14:08:34 | https://example.com/ | 0 | ERROR | False | 0 | 1919 | 1919 | 1919 | 1919 | 1919 |
| 14:08:38 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34566 | 2042 | 2043 | 2061 | 2086 | 2046 |
| 14:08:43 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1630 | 1630 | 1630 | 1630 | 1782 |

Averages: TTFB=1606.56 ms, TTFC=1621.78 ms, TTFR=1770.33 ms, TTI=1789.78 ms, Full=1775.56 ms, Bytes=162338.33

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260412-140843.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 893 | 884 | 469.53 | 309.45 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 1576 | 1572 | 1599.32 | 363.8 |  |  |  | Chrome: No successful runs (failures=1, err=[20536:26748:0412/140917.355:ERROR:chrome\browser\web_applications\os_integration\os_integration_manager.cc:257] Can't perform OS integration while the browser is shutting down.); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 944 | 806 | 576.76 | 220.37 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 2651 | 1920 | 4602.75 | 1283.06 |  |  |  | Chrome: No successful runs (failures=1, err=[22988:9392:0412/141000.739:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1037 | 692 | 3274.93 | 703.31 |  |  |  | Chrome: No successful runs (failures=1, err=[26260:14968:0412/141035.552:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://drive.google.com/drive/installwebapp?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
