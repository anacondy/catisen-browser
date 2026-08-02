# Catisen Comprehensive Test Report

Generated: 2026-04-14 17.39.22

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

- No request telemetry found.

## Parsed Catisen Diagnostics (Ctrl+D Equivalent)

- No parseable [REQ] telemetry lines found.

## Tor + Geo + Fingerprint Assertion Gates

| Case | ReqCfgFound | ReqFound | Status | Notes |
|---|---|---|---|---|
| Fingerprint-CreepJS-Desktop | False | False | FAIL | missing REQCFG; missing REQ |
| BotSannysoft-Hardened | False | False | FAIL | missing REQCFG; missing REQ |
| BrowserScan-Android-Profile | False | False | FAIL | missing REQCFG; missing REQ |
| Geo-Locked-Netflix-Probe | False | False | FAIL | missing REQCFG; missing REQ |
| Geo-Locked-BBC-Probe | False | False | FAIL | missing REQCFG; missing REQ |
| Onion-Tor-Probe | False | False | FAIL | missing REQCFG; missing REQ |
| Visual-Mode-Bridge | False | False | FAIL | missing REQCFG; missing REQ |
| Text-Mode-Cleaning | False | False | FAIL | missing REQCFG; missing REQ |
| Binary-Download-Interception | False | False | FAIL | missing REQCFG; missing REQ |

Gate Summary: PASS=0, FAIL=9

## Binary Download Assertion

| Scenario Included | Handoff Logs | New Files | Total New Bytes | Status | Notes |
|---|---|---|---|---|---|
| True | 0 | 0 | 0 | FAIL_NO_HANDOFF | No handoff log and no new downloaded file detected. |

## Browser/Network Comparison

| URL | Catisen Full ms | Catisen TTFC ms | curl Total ms | PowerShell HEAD ms | Chrome Avg ms | Brave Avg ms | Safari Avg ms | Notes |
|---|---|---|---|---|---|---|---|---|
| https://abrahamjuliot.github.io/creepjs/ |  |  | 226.91 | 612.03 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://bot.sannysoft.com/ |  |  | 487.59 | 507.7 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.browserscan.net/ |  |  | 855.3 | 216.19 |  |  |  | Chrome: No successful runs (failures=1); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.netflix.com/ |  |  | 4750.69 | 997.84 |  |  |  | Chrome: No successful runs (failures=1, err=[3236:11256:0414/174036.296:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: PHONE_REGISTRATION_ERROR); Brave: No successful runs (failures=1); Unavailable on Windows |
| https://www.bbc.co.uk/ |  |  | 6042.02 | 954.67 |  |  |  | Chrome: No successful runs (failures=1, err=[21148:20604:0414/174115.699:ERROR:google_apis\gcm\engine\registration_request.cc:291] Registration response error message: DEPRECATED_ENDPOINT); Brave: No successful runs (failures=1); Unavailable on Windows |
