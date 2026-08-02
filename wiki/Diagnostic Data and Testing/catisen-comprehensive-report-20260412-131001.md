# Catisen Comprehensive Test Report

Generated: 2026-04-12 13.10.01

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

- [REQ] ts=13:09:38 status=200 bytes=8331 mode=Source Code tor=false ttfb=589 ttfc=592 ttfr=615 tti=640 full=600 url=https://abrahamjuliot.github.io/creepjs/ headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=13:09:39 status=200 bytes=25648 mode=Source Code tor=false ttfb=659 ttfc=662 ttfr=1147 tti=1172 full=1132 url=https://bot.sannysoft.com/ headers=date: Sun, 12 Apr 2026 07:39:39 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=13:09:40 status=200 bytes=131635 mode=Source Code tor=false ttfb=720 ttfc=721 ttfr=747 tti=772 full=732 url=https://www.browserscan.net/ headers=content-type: text/html;charset=utf-8 | transfer-encoding: chunked | connection: keep-alive | date: Sun, 12 Apr 2026 05:12:37 GMT
- [REQ] ts=13:09:43 status=200 bytes=577469 mode=Source Code tor=false ttfb=1684 ttfc=1696 ttfr=2499 tti=2524 full=2484 url=https://www.netflix.com/ headers=server: envoy | x-frame-options: DENY | content-security-policy-report-only: default-src https: wss: 'unsafe-inline' 'unsafe-eval'; font-src https: data: ; img-src  https: data: blob: ; media-src https: blob: ; worker-src https: blob: ; report-uri https://www.netflix.com/log/www/csp/1; | accept-ch: Sec-CH-UA-Platform-Version,Sec-CH-UA-Model
- [REQ] ts=13:09:45 status=200 bytes=743075 mode=Source Code tor=false ttfb=822 ttfc=950 ttfr=1616 tti=1641 full=1601 url=https://www.bbc.co.uk/ headers=connection: keep-alive | vary: X-BBC-Edge-Scheme,x-id-oidc-signedin,Accept-Encoding | content-type: text/html | belfrage-cache-status: MISS
- [REQ] ts=13:09:54 status=200 bytes=4727 mode=Source Code tor=true ttfb=4374 ttfc=4374 ttfr=4395 tti=4420 full=4380 url=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion headers=server: nginx/1.22.1 | date: Sun, 12 Apr 2026 07:39:54 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=13:09:56 status=0 bytes=0 mode=ERROR tor=false ttfb=1957 ttfc=1957 ttfr=1957 tti=1957 full=1957 url=https://example.com/ headers=error: Network request failed for https://example.com/ on attempt 2/2: error sending request for url (https://example.com/)
- [REQ] ts=13:09:58 status=200 bytes=34548 mode=Text-Only tor=false ttfb=1474 ttfc=1476 ttfr=1498 tti=1523 full=1483 url=https://news.ycombinator.com/ headers=server: nginx | date: Sun, 12 Apr 2026 07:39:59 GMT | content-type: text/html; charset=utf-8 | transfer-encoding: chunked
- [REQ] ts=13:10:00 status=200 bytes=0 mode=Binary tor=false ttfb=1618 ttfc=1618 ttfr=1618 tti=1618 full=1624 url=https://ash-speed.hetzner.com/100MB.bin headers=server: nginx | date: Sun, 12 Apr 2026 07:40:00 GMT | content-type: application/octet-stream | content-length: 104857600

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 13:09:38 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 589 | 592 | 615 | 640 | 600 |
| 13:09:39 | https://bot.sannysoft.com/ | 200 | Source Code | False | 25648 | 659 | 662 | 1147 | 1172 | 1132 |
| 13:09:40 | https://www.browserscan.net/ | 200 | Source Code | False | 131635 | 720 | 721 | 747 | 772 | 732 |
| 13:09:43 | https://www.netflix.com/ | 200 | Source Code | False | 577469 | 1684 | 1696 | 2499 | 2524 | 2484 |
| 13:09:45 | https://www.bbc.co.uk/ | 200 | Source Code | False | 743075 | 822 | 950 | 1616 | 1641 | 1601 |
| 13:09:54 | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion | 200 | Source Code | True | 4727 | 4374 | 4374 | 4395 | 4420 | 4380 |
| 13:09:56 | https://example.com/ | 0 | ERROR | False | 0 | 1957 | 1957 | 1957 | 1957 | 1957 |
| 13:09:58 | https://news.ycombinator.com/ | 200 | Text-Only | False | 34548 | 1474 | 1476 | 1498 | 1523 | 1483 |
| 13:10:00 | https://ash-speed.hetzner.com/100MB.bin | 200 | Binary | False | 0 | 1618 | 1618 | 1618 | 1618 | 1624 |

Averages: TTFB=1544.11 ms, TTFC=1560.67 ms, TTFR=1788 ms, TTI=1807.44 ms, Full=1777 ms, Bytes=169492.56

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
- C:\Users\iassh\catisen\target\downloads\100MB-20260412-131000.bin (0 bytes)

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 600 | 592 | 478.32 | 392.35 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 1132 | 662 | 460.59 | 422.73 |  |  |  | Chrome: No successful runs (failures=1, err=[25948:27728:0412/131023.351:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://docs.google.com/presentation/installwebapp?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ | 732 | 721 | 1261.75 | 568.01 |  |  |  | Chrome: No successful runs (failures=1, err=[26636:13080:0412/131042.227:ERROR:chrome\browser\web_applications\externally_managed_app_manager.cc:680] https://mail.google.com/mail/installwebapp?usp=chrome_default from install source 1 failed to install with reason 21); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ | 2484 | 1696 | 5084.98 | 1178.6 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ | 1601 | 950 | 5350.56 | 492.06 |  |  |  | Chrome: No successful runs (failures=1, err=[26436:26368:0412/131211.071:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
