# CAT BRO — Complete status, timeline, structure, findings, and next plan

**Repository:** `catisen-browser`  
**Local path:** `C:\Users\iassh\catisen-browser`  
**Active branch:** `migration/browser-2e03418`  
**Product:** current Catisen wry/tao browser  
**Platform verified:** Windows/WebView2  
**Status date:** 2026-07-20

---

## 1. Product identity

CAT BRO is the current browser product. It must remain separate from:

```text
CAT     = original mixed development repository
CAT BRO = current wry/tao browser
CAT LEG = historical egui/HTTP privacy tool
```

CAT BRO is not the historical scraper/text client. Its intended runtime is:

```text
native tao window
  -> wry WebView
  -> WebView2 on Windows / WebKitGTK on Linux
  -> document-start toolbar and privacy scripts
  -> Rust IPC and browser services
```

---

## 2. CAT BRO timeline

### Base migration

```text
2e03418
```

This was selected as the current wry/tao browser baseline from the supplied history.

### P0 — integration-test contract repair

```text
23f12a3
```

Changes:

- added `--no-network` headless mode;
- added deterministic `HEADLESS_SMOKE_OK` output;
- replaced stale Mozilla/Wikipedia `[REQ]` telemetry integration test;
- removed the false dependency on unsupported `--url`, `--tor`, `CATISEN_LOG_FILE`, and `CATISEN_VIEW_MODE` behavior.

### P1 — dead-code cleanup

```text
5c59e85
```

Changes:

- removed deprecated `src/tor_proxy.rs` from the active browser;
- removed the extension stub from the active crate;
- removed the fake Servo compilation test;
- removed unused `UpdateTitle` event wiring;
- removed unused network re-exports;
- kept the actual browser/runtime modules intact.

### P2 — truthful sandbox status

```text
e6da91e
```

Changes:

- stopped claiming that the Windows Job Object/Linux Seccomp sandbox was active;
- replaced the false success message with an explicit “sandbox not implemented” warning;
- kept the browser running while accurately reporting the security limitation.

---

## 3. Verified build and test status

### CAT BRO compilation

```text
cargo check --workspace --all-targets
```

Result:

```text
PASS
```

### CAT BRO tests

Current result after P0/P1:

```text
10 unit tests: PASS
1 headless smoke integration test: PASS
```

The original external network integration test was removed from the default test path because it waited for telemetry the current WebView runtime never produced.

### CAT BRO runtime

Manual Windows runtime has been observed:

- WebView2 browser launches;
- DuckDuckGo loads;
- Wikipedia loads;
- YouTube loads and plays video;
- YouTube MiniPlayer works;
- MotionMark has run successfully;
- settings overlay opens;
- debug panel opens;
- same-tab new-window navigation is logged;
- EasyList initializes with 296 rules.

These are runtime observations, not a complete security certification.

### Remaining build quality

The project still emits roughly 35–36 warnings. They are not build failures, but they identify unfinished paths:

- unused config helpers;
- unused bookmark methods;
- unused download progress/pause/resume methods;
- unused network text extraction;
- unused fingerprint/geolocation helpers;
- unused Tor helpers;
- unused sync pairing argument/state;
- unused reader themes/raw cleaner;
- unused EasyList file loader;
- unused tab close method.

Strict Clippy remains a separate cleanup gate.

---

## 4. Current CAT BRO tree

### Product files

```text
Cargo.toml
Cargo.lock
config.toml
assets/
.github/workflows/
src/main.rs
src/config.rs
src/browser/
  chrome.rs
  debug_panel.rs
  ipc.rs
  mod.rs
src/network/
  client.rs
  error.rs
  fetch.rs
  mod.rs
  text_extract.rs
src/privacy/
  mod.rs
  stealth.rs
  tor_manager.rs
src/history.rs
src/libcurl_download_manager.rs
src/permissions.rs
src/sandbox.rs
src/sync_chain.rs
src/tab_isolation.rs
src/text_mode.rs
src/ublock_integration.rs
tests/integration_test.rs
```

### Current tree interpretation

| Area | State |
|---|---|
| `browser/` | Active current browser runtime. |
| `chrome.rs` | Toolbar, privacy initialization, popup/ad JS, reader button, YouTube hooks. |
| `browser/mod.rs` | WebView construction, IPC routing, navigation, managers, event loop. |
| `ipc.rs` | Toolbar-to-Rust messages and event types. |
| `network/client.rs` | Reqwest/rustls client construction. |
| `network/text_extract.rs` | Currently unused by the live WebView browser. |
| `privacy/stealth.rs` | Contains multiple unused profile/geo helpers; current browser uses only a subset. |
| `privacy/tor_manager.rs` | Canonical Tor manager; several helper APIs are unused. |
| `history.rs` | Active visit persistence, but repository-local storage remains a privacy problem until separately patched. |
| `libcurl_download_manager.rs` | Constructed and callable, but UI progress/pause/resume paths remain incomplete. |
| `permissions.rs` | Constructed and called, but requires origin-key and IPC review. |
| `sandbox.rs` | Honest status after P2; actual sandbox still not implemented. |
| `sync_chain.rs` | QR/identity code exists; actual pairing transport is not implemented. |
| `tab_isolation.rs` | Creates logical paths; does not yet create separate WebView storage contexts. |
| `text_mode.rs` | Reader mode is partly wired through live DOM JS; raw HTML helpers remain unused. |
| `ublock_integration.rs` | EasyList engine initializes; full subresource/ad behavior needs tests. |

---

## 5. What is wired up

Verified or observed:

- wry/tao window and WebView launch;
- document-start toolbar injection;
- navigation handler;
- same-tab new-window handler;
- IPC parsing;
- back/forward/reload controls;
- settings overlay;
- debug panel;
- reader-mode event path;
- adblock initialization;
- permission manager construction;
- history recording on page load;
- download manager construction and IPC entry point;
- sync overlay construction and QR generation path;
- Tor startup detection path;
- deterministic headless smoke path;
- WebView2 runtime on Windows.

“Wired” means a call path exists. It does not mean the feature is complete, secure, or fully tested.

---

## 6. What remains foggy, partial, or unfinished

### Toolbar lifecycle

Observed issue:

- toolbar can disappear during YouTube/SPA/embedded transitions;
- duplicate toolbars have appeared in embedded frames and popup/new-window contexts.

Required work:

- top-frame guard before any toolbar/Trusted Types setup;
- exactly one toolbar per top-level document;
- robust remount after SPA DOM replacement;
- native popup/new-window lifecycle test;
- iframe test.

### Trusted Types

The pass-through `default` policy helped YouTube/Google login compatibility, but it is too broad for the final security design.

Required work:

- keep YouTube/login compatibility;
- use a Catisen-specific policy;
- avoid page-derived `innerHTML`;
- use `textContent` and DOM construction;
- test login and toolbar injection under strict Trusted Types CSP.

### YouTube ad blocking

Ads are still observed. The current implementation is custom YouTube response/fetch interception, not proven Brave code.

Required tests:

- preroll;
- midroll;
- related-video SPA navigation;
- MiniPlayer;
- signed-out flow;
- no false blocking of video streams.

### Reader mode

The global reader CSS approach can break page controls, media, forms, and overlays. Scope it to reader content.

### Zoom

Ctrl+Plus, Ctrl+Minus, and Ctrl+0 remain unwired.

### Sandbox

The status is now honest, but CAT BRO remains unsandboxed until real platform enforcement is implemented.

### Tab isolation

Filesystem path bookkeeping is not equivalent to separate WebView cookie/storage contexts.

### History

History still requires application-data storage, atomic writes, and repository exclusion in the actual CAT BRO branch.

### Downloads

Download proxy behavior, safe destination paths, partial-file handling, and UI progress require source-level patching against the actual CAT BRO tree.

---

## 7. CAT BRO next plan

### P3 — browser behavior

1. Fix toolbar duplication/disappearance.
2. Replace global Trusted Types default policy.
3. Add toolbar/frame lifecycle instrumentation.
4. Fix reader-mode scoping.
5. Add native WebView zoom controls.
6. Test YouTube MiniPlayer after every change.

### P4 — privacy/security

1. Add strict navigation policy.
2. Constrain download destinations.
3. Prevent proxy fail-open behavior.
4. Move history outside repository.
5. Correct permission origin matching.
6. Validate UA/profile consistency.
7. Remove or implement real sandboxing.
8. Review page-to-Rust IPC capabilities.

### P5 — quality gates

1. Resolve remaining warnings by wiring/removing/feature-gating.
2. Remove fake Servo test/config references.
3. Run strict Clippy.
4. Add Windows and Linux CI.
5. Add WebView smoke tests.
6. Add network tests separately from browser UI tests.

### P6 — CAT LEG

Only after CAT BRO has a stable browser baseline:

1. inspect CAT LEG’s Servo/mozjs dependency;
2. remove or feature-gate unused Servo if it is not part of the supported tool;
3. build CAT LEG independently;
4. preserve CAT LEG as an HTTP/text research tool.

---

## 8. Current decision

CAT BRO is now a valid, buildable, testable browser baseline with known limitations.

The next work is not another architecture split. It is targeted browser behavior and security hardening.

CAT and CAT LEG remain separate.
