# Catisen Browser (CAT BRO)

Current Windows WebView2 browser extracted from the Catisen project.

## Current branch

```text
migration/browser-2e03418
```

## Current status

```text
cargo check: PASS
cargo test: PASS
10 unit tests: PASS
headless smoke test: PASS
Windows WebView2 launch: PASS
```

The branch is a working browser baseline, not a finished privacy/security product.

## Product scope

CAT BRO is the current wry/tao browser implementation:

```text
native tao window
  -> wry WebView
  -> WebView2 on Windows
  -> document-start toolbar/privacy scripts
  -> Rust IPC and browser services
```

Linux support requires the corresponding WebKitGTK development/runtime dependencies.

## Verified functionality

- embedded WebView2 browser window;
- DuckDuckGo, Wikipedia, and YouTube rendering;
- YouTube playback and MiniPlayer observed;
- browser navigation controls;
- settings overlay;
- debug panel;
- same-tab new-window handling;
- EasyList initialization;
- deterministic headless smoke test;
- build and unit/integration smoke tests.

## Known limitations

- approximately 35–36 compiler warnings remain;
- strict Clippy is not clean;
- toolbar can disappear or duplicate during iframe/popup/SPA transitions;
- Trusted Types compatibility currently uses a broad workaround requiring redesign;
- YouTube ads are not fully blocked;
- Ctrl+Plus, Ctrl+Minus, and Ctrl+0 are not complete;
- reader mode CSS requires scoping and regression tests;
- sandbox is not implemented;
- tab isolation does not yet create separate WebView storage contexts;
- history storage requires application-data migration;
- download safety/proxy behavior requires final patching;
- Tor was not validated in the current baseline;
- sync pairing transport is not implemented;
- CAT BRO is not a verified release build yet.

## Build

```powershell
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo run
```

## Testing policy

The default integration test is a local headless smoke test and does not depend on external websites or Tor. Network tests must be opt-in and must use the current runtime’s actual contract.

Do not treat a passing smoke test as proof of:

- sandboxing;
- anonymity;
- Tor routing;
- complete ad blocking;
- cookie isolation;
- secure download behavior.

## Project separation

- `catisen` — original preserved reference/workbench;
- `catisen-browser` — this current browser product;
- `catisen-legacy` — historical egui/HTTP privacy tool.

Do not copy legacy egui/Servo files into CAT BRO. Do not apply CAT patches to CAT LEG.
