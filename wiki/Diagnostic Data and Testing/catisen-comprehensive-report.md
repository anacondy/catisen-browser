# Catisen Comprehensive Test Report

Generated: 2026-04-14 19.15.21

Config: TorProxy=socks5h://127.0.0.1:9150, TargetFps=144

## Scenario Summary

| Case | URL | Tor | Geo | Profile | View | FPS |
|---|---|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | https://abrahamjuliot.github.io/creepjs/ | False | disabled | windows | source | 144 |
| BotSannysoft-Hardened | https://bot.sannysoft.com/ | False | disabled | linux | source | 144 |

## Raw Catisen Request Lines

- [REQ] ts=19:15:20 status=200 bytes=8331 mode=Source Code tor=false ttfb=220 ttfc=220 ttfr=235 tti=260 full=220 url=https://abrahamjuliot.github.io/creepjs/ cpu=0.00 ram=0.00 gpu=0.00 fps=0.00 device=N/A adblocks=0 data_saved=0 ist="N/A" pst="N/A" headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=19:15:20 status=200 bytes=8331 mode=Source Code tor=false ttfb=220 ttfc=220 ttfr=237 tti=262 full=222 url=https://abrahamjuliot.github.io/creepjs/ cpu=0.00 ram=0.00 gpu=0.00 fps=0.00 device=N/A adblocks=0 data_saved=0 ist="N/A" pst="N/A" headers=connection: keep-alive | vary: Accept-Encoding | server: GitHub.com | content-type: text/html; charset=utf-8
- [REQ] ts=19:15:21 status=200 bytes=7571 mode=Source Code tor=false ttfb=507 ttfc=507 ttfr=524 tti=549 full=509 url=https://bot.sannysoft.com/ cpu=0.00 ram=0.00 gpu=0.00 fps=0.00 device=N/A adblocks=0 data_saved=0 ist="N/A" pst="N/A" headers=date: Tue, 14 Apr 2026 13:45:20 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive
- [REQ] ts=19:15:21 status=200 bytes=7571 mode=Source Code tor=false ttfb=514 ttfc=514 ttfr=533 tti=558 full=518 url=https://bot.sannysoft.com/ cpu=0.00 ram=0.00 gpu=0.00 fps=0.00 device=N/A adblocks=0 data_saved=0 ist="N/A" pst="N/A" headers=date: Tue, 14 Apr 2026 13:45:20 GMT | content-type: text/html; charset=UTF-8 | transfer-encoding: chunked | connection: keep-alive

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

| Timestamp | URL | Status | Mode | Tor | Bytes | TTFB ms | TTFC ms | TTFR ms | TTI ms | Full ms |
|---|---|---|---|---|---|---|---|---|---|---|
| 19:15:20 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 220 | 220 | 235 | 260 | 220 |
| 19:15:20 | https://abrahamjuliot.github.io/creepjs/ | 200 | Source Code | False | 8331 | 220 | 220 | 237 | 262 | 222 |
| 19:15:21 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 507 | 507 | 524 | 549 | 509 |
| 19:15:21 | https://bot.sannysoft.com/ | 200 | Source Code | False | 7571 | 514 | 514 | 533 | 558 | 518 |

Averages: TTFB=365.25 ms, TTFC=365.25 ms, TTFR=382.25 ms, TTI=407.25 ms, Full=367.25 ms, Bytes=7951

## Tor + Geo + Fingerprint Assertion Gates

| Case | ReqCfgFound | ReqFound | Status | Notes |
|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | True | True | PASS | All gates satisfied |
| BotSannysoft-Hardened | True | True | PASS | All gates satisfied |

Gate Summary: PASS=2, FAIL=0

## Binary Download Assertion

| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |
|---|---|---|---|---|---|
| False | 0 | 0 | 0 | N/A | Binary case not included in this run. |

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ | 221 | 220 | 498.06 | 421.91 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ | 513.5 | 510.5 | 456.11 | 383.37 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
