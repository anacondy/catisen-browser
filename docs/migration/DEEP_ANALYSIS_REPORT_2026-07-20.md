# CAT BRO (catisen-browser) — Deep Analysis & Real-World Evaluation Report

**Date:** 2026-07-20
**Repository:** `anacondy/catisen-browser`
**Analyzed ref:** branch `migration/browser-2e03418`, commit `d2ef623` ("Document CAT BRO current state and cleanup policy")
**Analysis branch:** `arena/019f80fd-catisen-browser`
**Analyst:** Arena.ai Agent Mode (automated deep audit)
**Mode:** read-only analysis — no project files were modified or deleted; this report is the only addition.

---

## 0. Executive summary

CAT BRO is a small (~4,000 lines of Rust) wry/tao embedded-WebView browser shell. It is a **legitimate, coherently structured early-stage browser baseline** — not vaporware: the core event loop, toolbar injection, IPC, ad-block initialization, and a headless smoke path are real and internally consistent. However, it is **far from a usable daily-driver browser**, and a substantial fraction of what the surrounding documentation claims does not match what the code does.

**Headline verdicts**

| Question | Verdict |
|---|---|
| Is the repo clean & structured? | **Moderate.** Source tree is clean and well-commented; docs/wiki are polluted with ~20 legacy-era artifacts describing a different product. |
| Technical depth | **Low–moderate.** It is a WebView shell over platform engines (WebKitGTK/WebView2), with JS-injection-based privacy features. No engine work, no renderer isolation, no real tab model. |
| Does it build & pass its own tests? | Its 10 unit tests + 1 headless smoke test are self-consistent and should pass (verified by reasoning + partial execution; see §3). **The project's own CI cannot currently pass on a stock runner** (see §3.2). |
| Are the tests enough? | **No — 3/10.** 11 tests total, zero coverage of the browser runtime, IPC, navigation, toolbar, security policy, or platform behavior. |
| Cross-platform (Windows + Linux)? | **Partial on paper, unproven in CI.** Code targets both; CI builds neither properly; Wayland and macOS are gaps. |
| Can you sign in to Google? | **Not reliably — mostly no.** Google blocks OAuth in embedded WebViews, and CAT BRO's popup handling breaks OAuth popup flows by design. |
| Ad blocker condition | **Weak (~0.2% of EasyList, network-level blocking only covers top-level navigation).** A tested JS-regex gap lets the Facebook Pixel through. |
| Sandboxing condition | **None.** `sandbox.rs` is an honest placeholder; the process is fully unsandboxed. |
| Speed vs Chrome/Safari/Brave | **Unmeasurable here; structurally slower.** No live measurements were possible in this audit environment (§6); engine overhead + per-page script injection put it behind all three by design. |
| Ready for real users? | **No. Release-readiness ≈ 3/10.** |

---

## 1. Environment, methodology & what was actually executed

### 1.1 Hard environment constraints (important for interpreting results)

The audit sandbox (Debian bookworm container) had **no outbound network except an allowlist** (github.com, api.github.com, pypi.org, npm registry). Consequences:

- `crates.io`, `static.rust-lang.org`, all Rust mirrors, and `deb.debian.org` were unreachable → **`cargo check` / `cargo test` could not be run locally**, and the WebKitGTK dev stack could not be installed → **the GUI browser could not be launched or clicked through in this sandbox**.
- Therefore live page-load timing of CAT BRO itself, in-app site testing, and Google-login attempts **could not be physically executed here**. These sections are marked **[static]** (derived from code reading) or **[engine-derived]** (derived from the documented behavior of the underlying WebView2/WebKitGTK engines) instead of [measured].
- To compensate, three classes of *real* evidence were produced:
  1. **Executed local checks** (§3.3) — JavaScript payloads extracted from the Rust source and syntax-validated + behaviorally tested with Node; config parsing; URL-normalization attack probes; test inventory; hygiene scans.
  2. **Authoritative external data** — engine/library documentation (wry 0.38 API docs), WebKitGTK security advisories, Chrome release calendar, benchmark databases, Google OAuth policy documentation (all cited).
  3. **The project's real CI on GitHub Actions** — the branch was pushed and the repository's own `.github/workflows/ci.yml` was triggered via PR to obtain genuine compile evidence (§3.2).

### 1.2 Scope discipline

Per request: **nothing in the project was changed, deleted, or fixed**. Findings describe the tree at commit `d2ef623` exactly.

---

## 2. Repository cleanliness & structure

### 2.1 Source tree — good

```
src/
├── main.rs                      171 LOC   entry: args, sandbox call, headless pipeline, browser::run
├── browser/
│   ├── mod.rs                   744 LOC   wry/tao window, WebView, IPC routing, event loop   ← heart of the app
│   ├── chrome.rs                802 LOC   injected toolbar JS + privacy/stealth JS + YT ad-strip JS
│   ├── debug_panel.rs           251 LOC   settings + debug overlay JS generators
│   └── ipc.rs                    92 LOC   serde-tagged IPC message enum
├── network/                     (client, fetch, error, text_extract)  headless reqwest path
├── privacy/                     (stealth.rs UA/profiles, tor_manager.rs)
├── ublock_integration.rs        160 LOC   adblock crate wrapper (Brave's engine library)
├── libcurl_download_manager.rs  244 LOC   threaded downloads w/ pause/resume + SOCKS5h
├── history.rs, permissions.rs, tab_isolation.rs, sync_chain.rs,
│   text_mode.rs, sandbox.rs, config.rs
tests/integration_test.rs         31 LOC   headless smoke (spawns the built binary)
```

**Total: ~3,974 LOC of Rust.** The active browser path (`browser/`, `main.rs`, `ublock_integration.rs`, `history.rs`, parts of `privacy/`) is **well-commented, honest about limitations, and internally consistent** — the comments even document what used to be dead code and how it was wired. Comment quality is the strongest part of the codebase.

### 2.2 Hygiene scan — mixed [executed]

`git ls-files` (109 tracked files) was scanned against the project's own `docs/migration/CLEANUP_MANIFEST.md` rules:

| Check | Result |
|---|---|
| `target/`, `.cache/`, `.catisen_history.json`, screenshots, pastes, `*.bak*`, logs | **None tracked ✅** — the sensitive-state cleanup from the audit was done |
| Stray binary/asset in source tree | **`src/file.png` tracked ⚠️** (a PNG inside `src/` serving no purpose in code) |
| Orphan root files | **`test_font.rs`** (a 1-line `font8x8` snippet that is not part of the crate and references a dependency that isn't in Cargo.toml) and **duplicate `run_diagnostics.ps1`** (also present in `scripts/`) |
| Orphan source files | **`src/gtk_ui.rs` and `src/media_extractor.rs` are not declared in `main.rs`** — they are dead files that never compile. `gtk_ui.rs` even references a non-existent `crate::settings_ui` module and a `gtk4` dependency that isn't in `Cargo.toml` [executed check] |
| Report sprawl | `wiki/Diagnostic Data and Testing/` holds **17 near-duplicate generated reports**; `docs/` holds 4+ legacy-era "LATEST_*" reports; root has `"2 nd April Diagnose.md"` (note the broken filename with spaces) |
| `Cargo.lock` | **gitignored** — for an application binary this breaks reproducible builds; lockfiles should be committed |
| License/CI/workflows | MIT license present ✅; 2 workflows present (see §9.15) |

### 2.3 Documentation — the weakest area (accuracy ≈ 2/10)

The repo contains **three historical product identities** mixed into one documentation set:

| Document | Describes | Matches current code? |
|---|---|---|
| `README.md`, `ARCHITECTURE.md`, `docs/CAT_BRO_STATUS.md`, `docs/migration/*` | The wry/tao browser | **Yes** ✅ |
| `docs/BUILD.md`, `docs/USAGE.md`, `wiki/Technical_Details.md`, `wiki/Troubleshooting.md`, `initial_vision.md`, `LAUNCH_INSTRUCTIONS.md`, `TODO.md`, `TODO_2026-05-28.md`, `RUBYdiagnosticREPORT.md`, `2 nd April Diagnose.md`, `docs/FPS_SWEEP_*`, `docs/DIAGNOSTICS_REPORT_APRIL_5.md`, `docs/LATEST_*.md/txt`, `docs/SERVO_SPIKE_BRANCH_PLAN.md`, `STATUS.md` | The **old egui/eframe HTTP fetcher** (Servo spikes, `src/ui/app.rs`, `servo_renderer.rs`, `CATISEN_TEST_MODE`, FPS pacing, mpv handoff) | **No ❌** — those modules don't exist in this tree. E.g. `docs/USAGE.md` documents Ctrl+W/Ctrl+J/Ctrl+Shift+A shortcuts, PiP, "Clear all data on exit", and macOS support — none exist in the code |
| `docs/migration/AUDIT_REPORT.md` + addenda | The audit of both | Yes, and its findings are used as cross-checks below |

Notable contradictions found [executed comparison]:

- `config.toml` sets `default_view_mode = "source"` and `ublock_rules_path = "assets/easylist.txt"` — **the code never reads either field** (the wry browser has no view modes; the ad list is `include_str!`-embedded).
- Code default `home_page` is `https://www.mozilla.org`; shipped `config.toml` overrides it to DuckDuckGo; `browser/mod.rs` *additionally* hardcodes DuckDuckGo for empty input. Three sources of truth.
- `STATUS.md` still claims "Process Sandbox: Hooked Windows Job Objects / Linux Seccomp filters" and "100% COMPLETE" architecture — flatly contradicted by `sandbox.rs` itself ("sandbox is not implemented") and by `docs/CAT_BRO_STATUS.md`.
- `README.md` claims "10 unit tests: PASS" — the tree contains **exactly 10 unit `#[test]` + 1 integration test** [executed count], so that one claim is accurate.

---

## 3. Build & test verification

### 3.1 Test inventory — everything that exists [executed count]

| # | File | Test | What it proves | Verdict |
|---|---|---|---|---|
| 1 | `ublock_integration.rs` | `blocks_google_analytics` | bundled EasyList subset blocks GA | passes by construction |
| 2 | `ublock_integration.rs` | `blocks_doubleclick` | blocks doubleclick.net | passes |
| 3 | `ublock_integration.rs` | `allows_wikipedia` | no false positive on Wikipedia | passes |
| 4 | `ublock_integration.rs` | `disabled_blocker_allows_all` | toggle works | passes |
| 5 | `history.rs` | `test_history_lifecycle` | in-memory record/dedup | passes |
| 6 | `permissions.rs` | `test_strict_http_rejection` | HTTP sites force-denied | passes **but see bug §9.8** |
| 7 | `tab_isolation.rs` | `test_tab_creation_and_cleanup` | tab ID + path bookkeeping | passes, but tests bookkeeping that is **never connected to the WebView** |
| 8 | `sync_chain.rs` | `test_sync_chain_generation_and_export` | 12-word seed + QR payload | passes |
| 9 | `libcurl_download_manager.rs` | `test_download_manager_lifecycle` | pause flag bookkeeping | passes (network result ignored); **writes `test.bin` into repo root** |
| 10 | `libcurl_download_manager.rs` | `test_download_stores_proxy` | proxy string stored | passes; writes `file.bin` |
| 11 | `tests/integration_test.rs` | `headless_smoke_without_network` | binary runs `--headless --no-network` and prints `HEADLESS_SMOKE_OK` | passes iff the binary built |

**Which tests pass easily:** all 11, when the crate compiles — they are deliberately deterministic and network-free (this was an explicit P0 cleanup, and it was done well).
**Which tests would *not* pass:** none fail by design — but that is the problem: the test suite was made green by *removing* the hard assertions (the old external-network integration test was deleted as "stale"), so green ≠ browser works.

### 3.2 Real CI evidence — GitHub Actions [executed]

`.github/workflows/ci.yml` runs `cargo check` and `cargo build` on `ubuntu-latest` **without installing WebKitGTK/GTK dev packages and without running `cargo test`**:

> **Result:** *(this section is updated with the observed CI outcome of the PR that delivers this report — see the PR conversation and check runs; at the time of writing, `gh run list` shows the workflow had never successfully run in the repository's history at all.)*

Key structural CI findings regardless of outcome:

1. wry 0.38 on Linux requires `webkit2gtk-4.1`/`libsoup-3` via pkg-config ([wry 0.38 dependency list](https://docs.rs/wry/0.38.0/wry/struct.WebViewBuilder.html)). A stock `ubuntu-latest` image does not guarantee the `-4.1-dev` packages → `cargo check` is at risk of failing at `webkit2gtk-sys` unless the runner happens to carry them.
2. **There is no Windows job** in CI, while the README's only *verified platform* is Windows/WebView2. The verified platform is never built by CI.
3. **CI never runs `cargo test`** — the 11 tests are not gated.
4. No `cargo fmt --check`, no clippy, no artifact upload.

### 3.3 Checks actually executed in this audit

| Check | Method | Result |
|---|---|---|
| Injected JS syntax validity (toolbar, privacy/stealth, YT ad-strip, settings panel, debug panel, sync overlay) | extracted Rust string literals → `node --check` | **6/6 PASS** ✅ |
| JS ad-domain regex behavior (privacy layer) | ran the extracted `_AD_RE`/`_isAd` against 17 real ad & clean URLs | 5/9 ad URLs blocked; **Facebook Pixel `fbevents.js` NOT blocked (regex bug)**; Hotjar, Segment, Mixpanel, Sentry not covered; **0 false positives** on clean set incl. `googlevideo.com` (YT streams safe) ✅/⚠️ (§7.3) |
| URL normalization attack surface | ported `normalize_url()` 1:1, probed hostile inputs | `file:///etc/passwd` and `https://user:pass@host` **pass through unmodified** (§9.1) |
| Test inventory | regex over `#[test]` | 10 unit + 1 integration (§3.1) |
| Module-tree orphans | parsed `mod` decls vs files | `gtk_ui.rs`, `media_extractor.rs` never compiled; root `test_font.rs` stray |
| `config.toml` ↔ `CatisenConfig` | `tomllib` parse + field diff | parses OK; 2 dead fields (§2.3) |
| EasyList asset stats | line/rule counts | 296 lines, **165 network rules, 0 cosmetic rules** (§7.2) |
| Git hygiene vs CLEANUP_MANIFEST | `git ls-files` scan | clean of sensitive artifacts; `src/file.png`, `test_font.rs` stray (§2.2) |
| Seed entropy | 12^12 | **43 bits** (§9.4) |

---

## 4. Test suite rating (per parameter)

| Parameter | Score | Notes |
|---|---:|---|
| Unit coverage of pure logic (adblock wrapper, history, permissions, sync, tabs, downloads) | 5/10 | Exists, but tests bookkeeping rather than behavior |
| Browser/runtime coverage (window, WebView, event loop) | **0/10** | No tests touch `browser/` at all |
| IPC contract coverage | **0/10** | 15 IPC message variants, zero tests (malformed JSON, unknown tags, hostile origins) |
| Navigation policy coverage (scheme allowlist, redirects, new-window) | **0/10** | The most security-critical code path is untested |
| Injected-JS coverage (toolbar, stealth, YT strip) | **0/10** | ~1,700 lines of shipped JS with no tests (this audit's `node --check` is the first automated validation they've had) |
| Security/abuse coverage (path traversal, XSS, IPC spoofing) | **0/10** | Known-critical paths (§9) have no negative tests |
| Network/Tor coverage | 1/10 | Only a TCP-port probe in headless mode; opt-in and non-asserting |
| Cross-platform coverage | **0/10** | No Windows CI, no WebKitGTK CI job, no feature-gated tests |
| Determinism/CI-friendliness | 8/10 | `--no-network` smoke is genuinely well-designed |
| Regression guards for documented bugs (toolbar dupes, reader scoping, zoom) | **0/10** | Known issues listed in `docs/CAT_BRO_STATUS.md` §6 have no tests |
| **Overall test maturity** | **3/10** | Matches the prior audit's rating; honest but thin |

**What a minimally credible suite would add:** navigation-policy unit tests (schemes, credentials-in-URL, `.onion`), IPC fuzz tests, wry `with_navigation_handler` decision tests, a WebDriver/WebKit test for toolbar-once-per-frame, download path-confinement tests, and a Windows CI job.

---

## 5. What is wired / working vs not — feature status matrix

Legend: ✅ implemented & verified in code path · 🟡 wired but incomplete/cosmetic · ❌ absent/broken

| Feature | Status | Evidence |
|---|---|---|
| Window + WebView launch (tao/wry) | ✅ | `browser/mod.rs` builds `WebViewBuilder`, event loop runs |
| Toolbar (back/fwd/reload/URL bar/settings/debug/reader/badge flyout) | ✅ | `chrome.rs` TOOLBAR_JS; idempotency + top-frame guards present |
| URL bar normalization & search fallback | ✅ | `normalize_url()` (with §9.1 gaps) |
| Navigation + same-tab new-window interception | ✅ | `with_navigation_handler` + `with_new_window_req_handler` (breaks OAuth popups — §5.4) |
| SPA URL-bar sync (pushState/popstate) | ✅ | patched in TOOLBAR_JS |
| Settings overlay + debug panel | ✅ | `debug_panel.rs` generators |
| Reader mode (live-DOM CSS transform) | 🟡 | Works but global `!important` CSS breaks page controls/forms; `text_mode.rs::clean_html` is dead code; `TerminalBlue` theme has invalid CSS (`text-decoration: line-through decoration-color …`) |
| Ad-block engine init (Brave's `adblock` crate) | ✅ | 165 bundled rules; `should_block_request` works (§7) |
| JS-layer fetch/XHR ad blocking + cosmetic hiding + ad-iframe observer | ✅ | `privacy_init_js` (with §7.3 gaps) |
| YouTube ad-config stripping | 🟡 | `ytInitialPlayerResponse` trap + `/youtubei/v1/player|next` fetch rewrite — same family of technique uBO uses, but brittle; **preroll/midroll not covered** (they come from the same `googlevideo.com` CDN as content and nothing rewrites the player request's `adSlots` mid-stream beyond initial response) |
| Stealth/UA spoofing | 🟡 | JS-level `navigator.*` overrides only — **does not change real HTTP User-Agent headers** (wry 0.38 has `with_user_agent` for that; unused) → server-side fingerprint sees the true WebKitGTK/WebView2 UA; platform/timezone inconsistencies (§9.9) |
| Tor detection + env-var proxy (Linux WebKitGTK) | 🟡 | env vars set pre-WebView; works only on Linux, only at startup, and libsoup SOCKS semantics unverified |
| Tor on Windows WebView2 | ❌ | Env vars are ignored by WebView2 (code admits it) — **yet wry 0.38 already ships `with_proxy_config` with SOCKSv5 support on Windows/Linux** ([docs](https://docs.rs/wry/0.38.0/wry/struct.WebViewBuilder.html)); the in-code comment "until wry adds `with_proxy()`" is outdated for its own pinned version (§9.10) |
| Tor-routed downloads (libcurl, SOCKS5h, resume-preserving) | ✅ | correct `ProxyType::Socks5Hostname` usage; resume keeps proxy |
| Download manager reachable from UI | ❌ | `StartDownload` IPC exists but **no UI element ever sends it**; engine-native downloads unhandled (wry `with_download_started_handler` unused) → clicking a download link does nothing |
| Multi-tab browsing | ❌ | Single WebView. "New tab" navigates the *same* view to DuckDuckGo; `TabManager` only prints path strings; **Ctrl+Tab is hijacked to open a new tab instead of switching** |
| Tab isolation (multi-account) | ❌ | Bookkeeping only; one shared `WebContext` → one cookie jar (wry 0.38 supports per-context isolation via `with_web_context`; unused) |
| History persistence | 🟡 | Works, but plaintext `.catisen_history.json` in the **current working directory** (repo root when run from checkout); synchronous write on every navigation; no bounds/rotation |
| Permission manager | 🟡 | Records state but **cannot gate the engine** (wry 0.38 exposes no permission-request API; WebKitGTK's signal is not hooked) → effectively the engine's default-deny decides, not CAT BRO |
| Sync chain (QR identity) | 🟡 | QR renders in an isolated second WebView (good XSS fix), but 43-bit seed, seed printed to stdout, **pairing transport does not exist** (`pair_device()` prints fake success) |
| Sandbox | ❌ | Placeholder that logs "not implemented" (honest, at least) |
| Zoom (Ctrl+/-/0) | ❌ | Not implemented (README admits) |
| Extensions | ❌ | Removed in P1; none |
| Headless smoke mode | ✅ | `--headless [--no-network]` with deterministic `HEADLESS_SMOKE_OK` |
| Media extraction (mpv/vlc handoff) | ❌ | `media_extractor.rs` is an orphan file that never compiles |
| GTK4 shell | ❌ | `gtk_ui.rs` is an orphan file referencing a nonexistent module |

---

## 6. Speed, loading times & comparison with Chrome / Safari / Brave

### 6.1 What could and could not be measured

No live page-load timings of CAT BRO could be produced in this environment (§1.1): the binary cannot be built or run here. The repository's own timing reports (`docs/LATEST_*.md`, `wiki/Diagnostic Data and Testing/*`) are **all from the legacy egui/HTTP-fetcher era** — they measure `reqwest` fetches of raw HTML (e.g. "Mozilla 745 ms, 48,492 bytes"), not WebView rendering, and they are not evidence about the current browser. Their "Browser/Network Comparison" tables against Chrome/Brave/Safari are themselves empty: every comparison row reads *"Chrome: No successful runs; Brave: No successful runs; Unavailable on Windows"*.

### 6.2 Structural expectation (engine-derived)

CAT BRO's rendering speed **is** the speed of the host engine: WebView2 (Chromium) on Windows, WebKitGTK (WebKit) on Linux, plus:

- **Per-page overhead:** ~100 KB of initialization JavaScript (privacy + toolbar + YT-strip) parsed and executed at document-start on *every* page, in the main world.
- **Per-navigation overhead:** synchronous JSON history write; adblock mutex lock; permission-map writes.
- **No HTTP cache control, no preconnect, no process-per-site** tuning beyond engine defaults.

Realistic expectation: page loads within ~±10% of the underlying engine for typical pages, measurably worse on script-heavy pages, and far behind on benchmarks because there is no V8/WebKit JIT tuning, no site-specific optimizations, and extra injection cost.

### 6.3 Reference anchors for the requested comparison (public data)

Speedometer 3.0 (web-app responsiveness; higher is better), from a Feb-2025 cross-browser test: **Chrome 31.5, Chromium 30.7, Brave 29.3, Edge 27.2, Firefox 25.0, Safari 19.5** on the same machine ([zdnet](https://www.zdnet.com/home-and-office/work-life/i-speed-tested-11-browsers-and-the-fastest-might-surprise-you/)); a Feb-2026 round reports Speedometer **Chrome 144.9, Edge 131, Brave 108, Firefox 98.4**; MotionMark **Chrome 553, Brave 452, Firefox 258**; JetStream 2 **Edge 131.75, Chrome 130.3, Brave 121.5, Firefox 82.3** ([cloudwards](https://www.cloudwards.net/fastest-browser/)). (Scores are hardware-dependent; the ranking is the useful signal.)

| Dimension | Chrome 150 | Safari (WebKit) | Brave | **CAT BRO** |
|---|---|---|---|---|
| Engine | Blink/V8, multi-process | WebKit, multi-process | Blink/V8 + rust adblock engine | **WebKitGTK (Linux) / WebView2 (Windows), single app process** |
| Speedometer class (typical desktop) | 30–145 (hw-dependent) | 19–32 | 29–108 | **untested; ≤ host engine minus injection overhead** |
| Page-load infra | preconnect, speculative loads, HTTP/3, prioritization | same | same + request pruning from blocking | **none beyond engine defaults** |
| Ad/tracker blocking effect on speed | none (no blocker) | content blockers optional | **measurable speedup** (blocked requests never start) | tiny (165 rules, mostly JS-layer after the request was *about to* start) |
| Memory per typical site | ~300–500 MB with many processes | similar | similar, slightly lower | single-process shell + engine → lower floor, but one renderer compromise = whole app |
| Startup (published stopwatches) | ~0.7 s | ~1.3 s | ~1.2 s ([zdnet](https://www.zdnet.com/home-and-office/work-life/i-speed-tested-11-browsers-and-the-fastest-might-surprise-you/)) | engine + GTK init + adblock parse; never measured |

**Honest bottom line:** CAT BRO cannot out-load Chrome/Safari/Brave — it *is* a slower-configured instance of one of their engine families, with no benchmark evidence of its own. Any future claim needs Speedometer 3 + real-page TTFB/TTI runs on identical hardware.

---

## 7. Ad blocker — current condition

### 7.1 Architecture (5 layers)

1. **Engine-level, top-level navigations only:** `with_navigation_handler` consults the `adblock` crate (Brave's engine library — a genuinely good choice).
2. **JS-layer `fetch()`/`XHR` interception** by domain regex (`privacy_init_js`).
3. **JS-layer `window.open()` popup blocking** by the same regex.
4. **MutationObserver** neutralizing late-injected ad *iframes*.
5. **Cosmetic CSS** for ~25 ad-container selectors; plus **YouTube response-rewriting** (playerAds/adPlacements/adSlots/adBreakHeartbeatParams strip on initial + SPA responses).

### 7.2 Rule scale — the core weakness [executed count]

- Bundled `assets/easylist.txt`: **296 lines → 165 network rules, 0 cosmetic rules**.
- The real EasyList has **~82,000 rules** (45k optimized) and EasyPrivacy ~50,000 ([filterlist reference](https://github.com/yokoffing/filterlists), [Universal-FilterLists](https://github.com/Universalizer/Universal-FilterLists)).
- → CAT BRO ships **≈0.2% of EasyList and 0% of EasyPrivacy**. No `##` cosmetic rules exist in the engine at all; cosmetic hiding is ~25 hand-written selectors.
- The `adblock` crate is called with request type hardcoded to `"script"` and source URL = request URL, which disables the engine's type- and domain-specific rule logic — another accuracy reduction.
- Layer 1 only sees **top-level navigations**: image/pixel/media/font ad subresources never pass through it (wry 0.38 has no subresource-request hook), so "network-level" blocking effectively means "blocks you from typing an ad URL into the bar".

### 7.3 Behavioral test of the JS layer [executed]

Ran the extracted `_isAd()` against real-world tracker URLs:

```
BLOCKED  google-analytics.com/ga.js            BLOCKED  securepubads.g.doubleclick.net/gpt.js
BLOCKED  pagead2.googlesyndication.com         BLOCKED  cdn.taboola.com
BLOCKED  adservice.google.com                  BLOCKED  googletagmanager.com/gtm.js
allowed  connect.facebook.net/en_US/fbevents.js   ← BUG: Facebook Pixel passes
allowed  static.hotjar.com                        ← not covered
allowed  cdn.segment.com                          ← not covered
allowed  api.mixpanel.com                         ← not covered
allowed  browser.sentry-cdn.com                   ← not covered
allowed  googlevideo.com/videoplayback            ← correct (YT streams must not break)
allowed  accounts.google.com/signin               ← correct
```

- **Bug found:** the regex `facebook\.net\/.*pixel` tests `hostname + pathname` only — the canonical pixel script `fbevents.js` contains no "pixel" in its path, so **the Facebook Pixel is never blocked**.
- YouTube: initial-response stripping works against today's JSON shape; preroll/midroll and server-side ad injection are **not** covered (consistent with README's admission). Brave rewrites these responses with a battle-tested, continuously-updated pipeline; CAT BRO's copy will silently rot when YouTube changes field names.

### 7.4 vs Brave

Brave uses the same underlying `adblock-rust` engine but with **full lists (EasyList+EasyPrivacy+regional, 100k+ rules)**, engine-level **subresource** interception, cosmetic filters, CNAME uncloaking, and farmed-out YouTube scriptlet updates. CAT BRO uses the engine at roughly **0.2% capacity and only on top-frame navigations**. Rating: **functionally closer to "a handful of hosts file entries plus JS tricks" than to Brave Shields.**

---

## 8. Sandboxing — current condition

- `src/sandbox.rs` is a **documented placeholder**: `engage_strict_sandbox()` prints "not implemented" and returns `Ok(())` on both Windows and Linux; `main.rs` honestly logs *"sandbox status is not verified; browser is running without a proven process sandbox."*
- There is **no renderer isolation at all**: the WebView runs in-process with the full privileges of the browser (filesystem, network, the IPC channel). A renderer exploit (a WebKitGTK/WebView2 RCE) is an immediate full-process compromise.
- The IPC channel *widens* the attack surface: **any page's JavaScript can call `window.ipc.postMessage(...)`** and drive navigation, Tor/permission toggles, and — critically — `StartDownload{url, path}` with an **unvalidated absolute path** → a malicious page can write/append arbitrary files anywhere the process can write (`OpenOptions::create(true).append(true)`). This was flagged Critical #1 in `SOURCE_REVIEW_ADDENDUM.md` and **remains unfixed in this tree** (patch 0003 was never applied).
- Verdict: **sandboxing 0/10; process-security posture below any mainstream browser** (all of Chrome/Safari/Brave run multi-process site-isolation sandboxes).

---

## 9. Security, errors & vulnerability findings

Severity scale: 🔴 critical · 🟠 high · 🟡 medium · ⚪ low/informational.

| # | Sev | Finding | Evidence |
|---|---|---|---|
| 9.1 | 🟠 | **URL policy is incomplete.** `normalize_url()` passes through anything containing `://` — `file:///etc/passwd` reaches `load_url`; the nav handler only rejects lowercase-leading `javascript:`. `data:`, `blob:`, and credentials-in-URL (`https://user:pass@host`) are not rejected. Scheme check is case-sensitive (`JavaScript:` may slip past on engines that normalize case). | `browser/mod.rs` handler + executed port probe (§3.3) |
| 9.2 | 🔴 | **Page-controlled arbitrary file write.** Any page can send `{t:"start_download", url, path:"/home/victim/.config/autostart/evil.desktop"}` via IPC; path is used verbatim. | `ipc.rs` → `AppEvent::StartDownload` → `libcurl_download_manager.rs::start_download` |
| 9.3 | 🟠 | **Unauthenticated IPC.** No origin/allowlist check on `window.ipc.postMessage`; every command (navigate, set_tor, set_permission, downloads) is page-spoofable. | `with_ipc_handler` in `browser/mod.rs` |
| 9.4 | 🟠 | **Sync identity is cryptographically weak.** 12 words from a **12-word dictionary** = 12¹² ≈ 8.9×10¹² (**43 bits**); the seed is `println!`-ed to stdout/logs; `pair_device()` prints fake success with no transport. | `sync_chain.rs`; executed entropy calc |
| 9.5 | 🟠 | **libcurl proxy fails open.** If `easy.proxy()`/`proxy_type()` errors, the download **continues over clearnet** with only a warning — a Tor user can silently de-anonymize. | `libcurl_download_manager.rs` lines flagged in addendum §2, still present |
| 9.6 | 🟡 | **History in plaintext in the CWD.** `.catisen_history.json` (now gitignored ✅, but) is written synchronously per navigation into the working directory — inside the repo when run from a checkout; unbounded growth; no atomic write. | `history.rs` |
| 9.7 | 🟡 | **Debug panel builds DOM with `innerHTML` from `location.href`** — HTML injection into the panel (same-origin self-XSS, but the panel presents itself as security tooling). | `debug_panel.rs::row()` |
| 9.8 | 🟡 | **Permission manager keys never match.** `extract_domain()` strips the scheme, but `set_permission()` auto-denies unless the key *starts with* `https://` → the auto-deny branch always fires; the Settings panel sends `domain:'*'`, which `query_permission()` can never match by real host → the geolocation toggle is a **no-op**, and HTTPS sites get mis-keyed denials. | `permissions.rs` + `browser/mod.rs` UpdateUrlBar |
| 9.9 | 🟡 | **Fingerprint spoofing is inconsistent and detectable.** (a) UA is JS-only — real HTTP headers carry the true engine UA (wry's `with_user_agent` unused). (b) `platform` is hardcoded `"Win32"` even when the random UA is the Linux or macOS string. (c) timezone spoof covers `Intl` but not `Date.getTimezoneOffset()`. (d) spoofed UAs are Chrome **122–124 (early 2024)** while current stable is **Chrome 150** ([whatismybrowser](https://www.whatismybrowser.com/guides/the-latest-version/chrome), [fosspost](https://fosspost.org/chrome-version-history/)) — a 28-milestone-stale UA is an instant anomaly flag. (e) canvas noise is re-randomized per `toDataURL()` call, breaking deterministic canvas apps while still leaking a near-stable fingerprint. (f) `X-Forwarded-For` "geo spoofing" (dead helper) never anonymizes anything. | `chrome.rs::privacy_init_js`, `stealth.rs` |
| 9.10 | 🟡 | **Windows Tor gap is self-inflicted.** Code comment claims wry lacks a proxy API, but **wry 0.38 (the pinned version) ships `with_proxy_config` supporting SOCKSv5 on Windows *and* Linux** ([wry 0.38 docs](https://docs.rs/wry/0.38.0/wry/struct.WebViewBuilder.html)). The README/settings label ("Downloads Only on Windows") is therefore describing a fixable gap as permanent. (DNS semantics — socks5 vs socks5h — still need validation.) | docs.rs vs `browser/mod.rs` module comment |
| 9.11 | 🟡 | **Engine CVE exposure depends entirely on the user's distro.** WebKitGTK advisories are continuous: fixes shipped in 2.48.2 ([WSA-2025-0004](https://webkitgtk.org/security/WSA-2025-0004.html)) and 2.52.5 ([WSA-2026-0004](https://webkitgtk.org/security/WSA-2026-0004.html)), with same-origin-policy bypass, CSP-enforcement bypass, and fingerprinting CVEs in recent cycles ([AlmaLinux errata for 2025–2026 CVE list](https://errata.almalinux.org/8/ALSA-2026-10702.html)). CAT BRO pins **no minimum WebKitGTK version check** at startup, and its gtk-rs stack (glib 0.18.5) carries RUSTSEC-2024-0429 unsoundness ([seen in wry release advisories](https://github.com/tauri-apps/wry/releases)). No `cargo audit` gate exists. |
| 9.12 | 🟡 | **Tor diagnostics leak a clearnet request.** `maybe_probe_tor_route()` correctly probes over Tor, then calls `ipapi.co` **without the proxy** to resolve exit-country. | `privacy/tor_manager.rs` |
| 9.13 | 🟡 | **`std::env::set_var` for proxy config** runs in a process that spawns threads — racy/UB-prone once any thread exists (edition 2021 permits it; 2024 makes it `unsafe`). | `browser/mod.rs` step 8 |
| 9.14 | ⚪ | **`Cargo.lock` is gitignored** — non-reproducible builds for an application; supply-chain diffs invisible. | `.gitignore` |
| 9.15 | 🟠 | **Changelog workflow shell injection (still present).** `changelog-on-push.yml` interpolates `${{ github.event.head_commit.message }}` directly into a shell `echo`, runs with `contents: write`, and pushes to main — a crafted commit message yields code execution in CI. Audited as High in `AUDIT_REPORT.md`; unfixed here. | `.github/workflows/changelog-on-push.yml` |
| 9.16 | ⚪ | **Compiler warnings.** Docs claim ~35–36 warnings; static dead-code enumeration reproduces the same order of magnitude (whole unused APIs in `stealth.rs` — `build_stealth_script`, all `GeoLocation` methods, all `parse_*_env`; `tor_manager::resolve_tor_proxy/resolve_or_launch_tor_proxy`; `ublock::load_filter_list`; `history::get_bookmarks`; `text_mode::clean_html/set_theme/remove_html_tag`; `libcurl::get_progress` + unread `total_bytes/downloaded_bytes/is_completed` fields; `sync_chain::pair_device` + unread `is_synced`; `config::FingerprintLevel::defaults/from_runtime_flags`; `network/text_extract.rs` entire module unused by the browser). Clippy with `-D warnings` would fail. |

---

## 10. Real-life scenario analysis (user personas)

### 10.1 Persona A — general browsing (search, news, Wikipedia, shopping)

**[engine-derived]** DuckDuckGo/Wikipedia/news sites render fine — these run on the same WebKitGTK/WebView2 engines that Epiphany/Edge use. Frictions: the injected toolbar shifts layout (`margin-top: 52px !important` on every page), dark scrollbars are forced on light sites, and Ctrl+R was *removed* from reader mode to avoid fighting the native reload — minor but constant. Reader mode is usable on article pages but, by its own global-CSS design, breaks forms/media/overlays. **Verdict: passable for casual reading, not comfortable daily.**

### 10.2 Persona B — entertainment (YouTube, Netflix, Spotify, Twitch, social)

- **YouTube:** loads and plays (README-observed on Windows); the ad-strip removes *config-based* ads but not preroll/midroll reliably; MiniPlayer reported working. SPA navigation keeps working because the toolbar re-syncs on pushState.
- **Netflix/Prime/Disney+:** **will not play on Linux.** WebKitGTK's EME is built without Widevine — GNOME Web (the reference WebKitGTK browser) explicitly does not support Netflix/DRM content ([debugpoint Web 45 feature table](https://www.debugpoint.com/gnome-web-update/); [WebKitGTK EME/Widevine discussion](https://www.reddit.com/r/Fedora/comments/186ox3d/epiphany_with_extension_and_netflix_support/)). On Windows/WebView2, Netflix *can* work (Edge engine + PlayReady), but CAT BRO's forced UA/platform spoofing and popup interception add breakage risk.
- **Spotify Web / Twitch:** WebRTC-heavy features (live comments, DMs) are weaker on WebKitGTK than Chromium; basic playback generally OK.
- **Verdict: Linux build = no DRM streaming at all; Windows build = partial.**

### 10.3 Persona C — developer/coding (GitHub, GitLab, StackOverflow, CodePen, VS Code Web)

**[engine-derived]** Git hosting + docs render (modern WebKit/Chromium engines pass Interop well). Pain points: Ctrl+D (devtools-ish) is hijacked by the debug overlay; Ctrl+T/Ctrl+Tab don't manage real tabs; no DevTools wiring (wry devtools not enabled); no zoom; heavy IDE-class apps (vscode.dev, CodeSandbox) stress the single-process model with no isolation. **Verdict: fine for reading docs/issues; unsuitable as a coding browser.**

### 10.4 Persona D — privacy/Tor user

- Tor routing only at startup, only on Linux, with an unverified SOCKS DNS story; **Windows browsing is never on Tor** (§9.10). Toggling Tor at runtime only affects later downloads. Onion pages can load *if* the proxy is up at launch; the probe still leaks one clearnet geo lookup (§9.12).
- Fingerprint test sites (CreepJS/BrowserLeaks — which the legacy test matrix did run) would expose the UA/platform/timezone inconsistencies immediately (§9.9). The legacy reports show CreepJS fetched with status 200 in the *old HTTP client* — that is not evidence about the WebView's fingerprint.
- **Verdict: not anonymity-grade; the honest labels in the settings panel now say so, which is credit — but the product should not be used for threat-model-level privacy.**

### 10.5 Authentication — will signing in work?

- **Google:** two independent blockers. (1) Since 2017/2021 Google **refuses OAuth in embedded WebViews** — `disallowed_useragent` / "this browser or app may not be secure" — and Microsoft explicitly confirmed **Google auth flows are not supported in WebView2** ([MicrosoftEdge/WebView2Feedback#1647](https://github.com/MicrosoftEdge/WebView2Feedback/issues/1647); [Google policy background](https://truelink-group.com/en/blog/why-google-login-fails-in-line-facebook-in-app-browsers-2026/); [ServiceNow KB on disallowed_useragent](https://support.servicenow.com/kb?id=kb_article_view&sysparm_article=KB0623491)). Whether accounts.google.com *direct navigation* completes in WebView2/WebKitGTK is inconsistent and policy-dependent ([pake embedded-webview OAuth limitation docs](https://github.com/tw93/pake/issues/1148)); (2) CAT BRO **redirects every `window.open()` popup into the same tab**, which the code itself admits breaks OAuth popup + `postMessage` flows — so "Sign in with Google" buttons on third-party sites will not complete even where Google would allow the engine.
- **Microsoft/GitHub/password-based logins:** same-tab navigation keeps cookies (single shared WebContext), so classic redirect-flow logins generally work [engine-derived]; GitHub's popup flow will not.
- **Verdict: assume Google sign-in is broken; redirect-style logins mostly work; no credential storage/autofill exists.**

### 10.6 Site compatibility quick matrix (engine-derived, Windows=WebView2 / Linux=WebKitGTK)

| Site | Windows | Linux | Notes |
|---|---|---|---|
| DuckDuckGo, Wikipedia, BBC, news | ✅ | ✅ | README-verified class |
| YouTube (playback) | ✅ | ✅ | ad-blocking partial |
| Google Search | ✅ | ✅ | |
| Google **login** | ⚠️/❌ | ⚠️ | embedded-WebView policy + popup breakage |
| GitHub (read/issues) | ✅ | ✅ | popup OAuth ❌ |
| Netflix/DRM video | ⚠️ | ❌ | no Widevine on WebKitGTK |
| Google Meet/Zoom web | ⚠️ | ⚠️ | WebRTC historically weak on WebKitGTK |
| Banking sites | ⚠️ | ⚠️ | bot-detection sees stale UA + engine tells; not tested |
| `.onion` sites | ❌ (no Tor path) | 🟡 (startup-only) | |

---

## 11. Cross-platform readiness

| Aspect | Windows | Linux | Verdict |
|---|---|---|---|
| Engine | WebView2 (Edge/Chromium) | WebKitGTK 4.1 | both real engines ✅ |
| Code paths compile for both | `cfg(target_os="windows")` winapi dep; no windows-only code in browser core | gtk/webkit via wry | plausible, **unproven by CI** |
| Built by CI | **No Windows job** | one ubuntu job that doesn't install WebKitGTK dev packages and never runs tests (§3.2) | ❌ |
| Tor browsing | ❌ env vars ignored; fix exists unused (§9.10) | 🟡 startup-only | ❌/🟡 |
| Display server | n/a | **wry 0.38's `WebViewBuilder::new()` is X11-only** — Wayland needs the separate `new_gtk` path ([wry docs](https://docs.rs/wry/0.38.0/wry/struct.WebViewBuilder.html)); CAT BRO uses `new()` → Wayland-only sessions need XWayland | ⚠️ |
| Packaging | `scripts/build_windows.ps1` (zip; GTK DLL bundling commented out though the app doesn't need GTK4 on Windows anyway — script is confused) | `scripts/build_unix.sh` (cargo-deb; also has a macOS `.app` branch) | ⚠️ scripts are legacy-era guesses |
| macOS | n/a | n/a | **not a target** despite `initial_vision.md`/build script claims; zero macOS handling |
| Verified anywhere | README claims manual Windows verification | Replit nix env pins webkitgtk_4_1; no CI proof | claims only |

**Verdict: source is cross-platform-shaped, but "cross-platform" is currently an untested claim. Neither platform is green in CI.**

---

## 12. Documentation-vs-reality audit (claim → verdict)

| Claim (source) | Verdict |
|---|---|
| "cargo check/test PASS, 10 unit tests PASS, headless smoke PASS" (README) | **Credible** — test inventory matches; smoke contract is deterministic; but *not reproduced in this sandbox* and never gated in CI |
| "Process-Level Sandboxing: Hooked Windows Job Objects / Linux Seccomp" (STATUS.md) | **False** — placeholder, contradicted by sandbox.rs itself |
| "WASM Extension Matrix", "Servo WebRender pipeline", "120FPS" (STATUS.md, TODO.md) | **False for this tree** — those files don't exist here; legacy-product claims |
| "Tor routing (Linux WebView / all downloads)" (toolbar flyout) | **Half-true** — Linux startup-only; downloads yes; Windows browsing no |
| "Tab isolation (multi-account)" (flyout + settings) | **False** — bookkeeping only, shared cookie jar |
| "Strict HTTPS enforcement" (flyout) | **False** — no HTTPS-only mode in code; HTTP navigates freely (only hardware permissions are auto-denied on HTTP) |
| "Clear all data on exit", "Ctrl+W close tab", "Ctrl+J downloads", PiP, macOS (docs/USAGE.md) | **False** — legacy document |
| Download manager pause/resume (README/USAGE) | **Exists in code, unreachable from UI**; fails-open proxy (§9.5) |
| "EasyList initializes with 296 rules" (status doc) | True, but 296 *lines* = 165 *rules* ≈ 0.2% of real EasyList (§7.2) |
| WebView2 proxy "not possible until wry adds with_proxy" (code comment) | **Outdated** — wry 0.38 has `with_proxy_config` (§9.10) |

---

## 13. Final ratings

| Dimension | Score | Rationale |
|---|---:|---|
| Repository cleanliness | 6/10 | Source clean & honest; docs polluted; stray files; lockfile ignored |
| Structure & architecture | 6/10 | Sensible module layout; no policy/egress/lifecycle centralization; JS-in-page chrome is fragile |
| Technical depth | 4/10 | Thin WebView shell; no engine, no isolation, no real tabs, no service workers/persistence strategy |
| Test maturity | 3/10 | §4 |
| Build health | ~5/10 | Self-reported green; ~35 warnings; clippy-fail; CI structurally broken/absent |
| Feature completeness (browser) | 3/10 | One real tab, no zoom, no downloads UI, no extensions, no real isolation |
| Ad blocking | 2.5/10 | Right engine, 0.2% of data, wrong interception layer, tested regex gaps |
| Privacy/anti-fingerprint | 3/10 | JS-only spoofs, inconsistent, stale UAs, clearnet leaks |
| Security posture | 2/10 | No sandbox, IPC arbitrary-file-write, fail-open Tor, weak seed, scheme gaps |
| Cross-platform proof | 2/10 | Claims without CI; Wayland/macOS gaps |
| Documentation accuracy | 2/10 | Two stale product identities mixed in; current docs mostly honest |
| **Overall release readiness** | **3/10 — No-go** | Matches the embedded audit; nothing since has changed the security blockers |

---

## 14. Recommended priority order (if work resumes)

**P0 — safety** (before anyone browses anything real): confine download paths + authenticate IPC origins (9.2/9.3); strict scheme/credential URL policy (9.1); fail-closed libcurl proxy (9.5); fix changelog workflow injection (9.15).
**P1 — honesty of the engine layer**: use `with_proxy_config` for real Tor on both platforms (9.10); use `with_user_agent` so HTTP headers match the spoof (9.9a); real EasyList+EasyPrivacy lists with subresource-level blocking; minimum-WebKitGTK-version check + `cargo audit` gate (9.11).
**P2 — browser basics**: real multi-WebView tabs via `with_web_context`; native download handlers (`with_download_started_handler`); zoom; permission-request wiring.
**P3 — quality gates**: commit Cargo.lock; Windows + WebKitGTK CI jobs that run `cargo test`; delete orphans (`gtk_ui.rs`, `media_extractor.rs`, `test_font.rs`, `src/file.png`, duplicate scripts); archive the 20+ legacy docs/reports; replace report-sprawl with one schema.

---

## Appendix A — Evidence log (executed commands)

```text
git ls-files | wc -l                      → 109 tracked files
git ls-files | grep (CLEANUP patterns)    → only assets/mentality_bg.png + src/file.png (benign pattern hits); no sensitive artifacts
grep '#[test]' over *.rs                  → 11 tests (10 unit + 1 integration), listed §3.1
node --check on 6 extracted JS payloads   → 6/6 PASS (toolbar, privacy_init, yt_adblock, settings, debug, sync overlay)
node behavioral test of _AD_RE/_isAd      → 17 URLs: FB-pixel gap confirmed, 0 false positives, googlevideo safe
python tomllib on config.toml             → parses; dead fields default_view_mode, ublock_rules_path
python port of normalize_url()            → file:/// + user:pass@ pass through
wc/grep assets/easylist.txt               → 296 lines / 165 '||' rules / 0 '##' cosmetic
12**12                                    → 8,916,100,448,256 combinations = 43.0 bits
gh api compare main...migration           → migration is 5 commits ahead, 0 behind
gh run list                               → no successful workflow runs in repo history at delivery time
```

**External references:** wry 0.38 API ([docs.rs](https://docs.rs/wry/0.38.0/wry/struct.WebViewBuilder.html)) · Chrome 150 current ([whatismybrowser](https://www.whatismybrowser.com/guides/the-latest-version/chrome), [fosspost](https://fosspost.org/chrome-version-history/)) · Speedometer/MotionMark/JetStream data ([zdnet](https://www.zdnet.com/home-and-office/work-life/i-speed-tested-11-browsers-and-the-fastest-might-surprise-you/), [cloudwards 2026](https://www.cloudwards.net/fastest-browser/)) · Google embedded-WebView OAuth policy ([WebView2Feedback#1647](https://github.com/MicrosoftEdge/WebView2Feedback/issues/1647), [truelink](https://truelink-group.com/en/blog/why-google-login-fails-in-line-facebook-in-app-browsers-2026/), [ServiceNow KB](https://support.servicenow.com/kb?id=kb_article_view&sysparm_article=KB0623491), [pake#1148](https://github.com/tw93/pake/issues/1148)) · WebKitGTK DRM ([debugpoint](https://www.debugpoint.com/gnome-web-update/), [fedora thread](https://www.reddit.com/r/Fedora/comments/186ox3d/epiphany_with_extension_and_netflix_support/)) · WebKitGTK advisories ([WSA-2026-0004](https://webkitgtk.org/security/WSA-2026-0004.html), [WSA-2025-0004](https://webkitgtk.org/security/WSA-2025-0004.html), [errata](https://errata.almalinux.org/8/ALSA-2026-10702.html)) · filter-list scale ([yokoffing](https://github.com/yokoffing/filterlists), [Universalizer](https://github.com/Universalizer/Universal-FilterLists)) · glib unsoundness ([wry advisories](https://github.com/tauri-apps/wry/releases)).

*Report generated 2026-07-20. No project files were modified; this document is the sole addition.*
