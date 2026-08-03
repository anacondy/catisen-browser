# Catisen Browser (CAT BRO)

A Rust desktop browser built with `wry`/`tao`, using Microsoft Edge WebView2 on Windows and WebKitGTK on Linux.

## Current repository status

The browser source was imported from the repository's latest visible browser development snapshot and is being hardened on the Arena review branch. This is not yet a release build.

Local validation in the current analysis environment:

```text
Rust toolchain: unavailable in the sandbox
JavaScript syntax checks: passing for generated toolbar, settings, privacy, and reader scripts
Rust compile/test verification: pending CI or a local Windows/Linux toolchain
```

Do not interpret the existence of a UI toggle or a passing pure-function test as proof of sandboxing, anonymity, Tor routing, complete ad blocking, cookie isolation, or secure downloads.

## Product scope

```text
native tao window
  -> wry WebView
  -> WebView2 on Windows / WebKitGTK on Linux
  -> document-start toolbar and privacy scripts
  -> Rust navigation policy, IPC, history, downloads, and settings services
```

Windows WebView2 is the primary target. A Windows build requires the WebView2 Runtime and MSVC build tools. Linux requires the corresponding GTK/WebKitGTK development packages.

## Current hardening work

- HTTP/HTTPS-only navigation policy with credential rejection;
- explicit `.onion` route checks and no silent Tor-to-clearnet fallback in headless mode;
- JSON-safe JavaScript value serialization;
- document-start insecure-origin hardware API denial as defense-in-depth;
- bounded, proxy-aware download workers with stable IDs and non-panicking setup paths;
- persistent history opt-out, onion filtering, bounded snapshots, and background visit writes;
- exact permission-origin keys including scheme and port;
- 256-bit sync identity generation without seed logging;
- actual isolated-profile directory cleanup when a tab context is closed;
- CI format, check, test, Clippy, and Windows WebView2 build gates.

## Known limitations / remaining blockers

- browser chrome and page JavaScript still share a WebView context; privileged IPC needs a native or separately authenticated trust boundary;
- the application hardening controls are not a complete Chromium-style renderer sandbox;
- tab bookkeeping exists, but separate WebView cookie/storage contexts are not active yet;
- WebView permission callbacks for camera, microphone, and notifications require platform-specific integration;
- EasyList filtering remains best effort until resource interception supplies actual initiator and resource type for all subresources;
- sync pairing transport and key exchange are not implemented;
- a local Cargo/Rust verification pass is still required before release;
- F11/Fn+F11 uses title-bar-only borderless mode; resizing is intentionally disabled while decorations are hidden and restored on exit;
- video players retain their own responsive aspect/layout; Catisen does not force a global video height because that can clip portrait/4:3 controls.

## Build and test

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo run
```

For a deterministic headless smoke test:

```powershell
cargo run -- --headless --no-network
```

For one-shot text fetching, use `--url <URL>` with `CATISEN_VIEW_MODE=TextOnly`; add `--tor` only when a reachable SOCKS endpoint is configured. Tor-required paths fail instead of silently falling back to clearnet.
