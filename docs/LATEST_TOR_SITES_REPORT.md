# Catisen Tor Site Matrix Report

Generated: 2026-04-14 18.04.14

RunSecondsPerCase: 12
RetriesOnNoReq: 2
TextModeExtraSeconds: 6
TorProxy: socks5h://127.0.0.1:9150
TargetFps: 144

## Summary

- Total Cases: 20
- Cases with telemetry [REQ]: 3
- Cases without [REQ]: 17
- Cases with successful fetch (status>0 and bytes>0): 2

## Detailed Results

| Site | Mode | ReqFound | Status | TorUsed | Bytes | TTFB | TTFC | TTFR | TTI | Full | Outcome |
|---|---|---|---|---|---|---|---|---|---|---|---|
| DuckDuckGo Onion | source | True | 200 | True | 163299 | 4239 | 4246 | 4291 | 4316 | 4276 | OK |
| Ahmia | source | True | 200 | True | 4727 | 1864 | 1865 | 1880 | 1905 | 1865 | OK |
| Torch | source | True | 0 | True | 0 | 9097 | 9097 | 9097 | 9097 | 9097 | FAIL_HTTP |
| BBC News Onion | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| ProPublica | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| New York Times Onion | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| Facebook Onion | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| ProtonMail Onion | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| OnionShare | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| F-Droid Onion | source | False |  | True |  |  |  |  |  |  | NO_REQ |
| DuckDuckGo Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| Ahmia | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| Torch | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| BBC News Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| ProPublica | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| New York Times Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| Facebook Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| ProtonMail Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| OnionShare | text | False |  | True |  |  |  |  |  |  | NO_REQ |
| F-Droid Onion | text | False |  | True |  |  |  |  |  |  | NO_REQ |

## Failures / Missing Telemetry

- Site=Torch, Mode=source, URL=http://xmh57jrknzkhv6y3ls3ubitzfqnkrwxhopf5aygthi7d6rplyvk3noyd.onion, Error=status=0, bytes=0; req acquired after retry attempt=2
- Site=BBC News Onion, Mode=source, URL=http://bbcnewsd73hkzno2ini43t4gblxvycyac5aw4gnv7t2rccijh7745uqd.onion, Error=No request telemetry after 3 attempt(s)
- Site=ProPublica, Mode=source, URL=http://p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion, Error=No request telemetry after 3 attempt(s)
- Site=New York Times Onion, Mode=source, URL=http://nytimesn7cgmftshazwhfgzm37qxb44r64ytbb2dj3x62d2lljsciiyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=Facebook Onion, Mode=source, URL=http://facebookwkhpilnemxj7asaniu7vnjjbiltxjqhye3mhbshg7kx5tfyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=ProtonMail Onion, Mode=source, URL=http://protonmailrmez3lotccipjhktkjuu3f3q2z3u2q3k4b4q4b4q.onion, Error=No request telemetry after 3 attempt(s)
- Site=OnionShare, Mode=source, URL=http://lldan5gahapx5k7iafb3s4ikijc4ni7gx5iywdflkba5y2ezyg6sjgyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=F-Droid Onion, Mode=source, URL=http://fdroidorg6cooksyluodepej4erfctzk7rrjpjbbr6wx24jh3lqyfwyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=DuckDuckGo Onion, Mode=text, URL=http://duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion, Error=No request telemetry after 3 attempt(s)
- Site=Ahmia, Mode=text, URL=http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=Torch, Mode=text, URL=http://xmh57jrknzkhv6y3ls3ubitzfqnkrwxhopf5aygthi7d6rplyvk3noyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=BBC News Onion, Mode=text, URL=http://bbcnewsd73hkzno2ini43t4gblxvycyac5aw4gnv7t2rccijh7745uqd.onion, Error=No request telemetry after 3 attempt(s)
- Site=ProPublica, Mode=text, URL=http://p53lf57qovyuvwsc6xnrppyply3vtqm7l6pcobkmyqsiofyeznfu5uqd.onion, Error=No request telemetry after 3 attempt(s)
- Site=New York Times Onion, Mode=text, URL=http://nytimesn7cgmftshazwhfgzm37qxb44r64ytbb2dj3x62d2lljsciiyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=Facebook Onion, Mode=text, URL=http://facebookwkhpilnemxj7asaniu7vnjjbiltxjqhye3mhbshg7kx5tfyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=ProtonMail Onion, Mode=text, URL=http://protonmailrmez3lotccipjhktkjuu3f3q2z3u2q3k4b4q4b4q.onion, Error=No request telemetry after 3 attempt(s)
- Site=OnionShare, Mode=text, URL=http://lldan5gahapx5k7iafb3s4ikijc4ni7gx5iywdflkba5y2ezyg6sjgyd.onion, Error=No request telemetry after 3 attempt(s)
- Site=F-Droid Onion, Mode=text, URL=http://fdroidorg6cooksyluodepej4erfctzk7rrjpjbbr6wx24jh3lqyfwyd.onion, Error=No request telemetry after 3 attempt(s)

## Per-Mode Averages (Successful Cases Only)

| Mode | Avg TTFB | Avg TTFC | Avg TTFR | Avg TTI | Avg Full |
|---|---|---|---|---|---|
| source | 5066.67 | 5069.33 | 5089.33 | 5106 | 5079.33 |
| text | - | - | - | - | - |
