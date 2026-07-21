# Catisen source review addendum — 2026-07-18

The additional `catisen-rust-source.txt` supplied after the first audit contains 32 source/manifest/test files. This addendum supersedes the earlier “Rust source absent” limitation for the current wry snapshot.

## Confirmed current architecture

The supplied current source is the **wry/tao implementation**, not the historical egui implementation:

- `Cargo.toml`: `wry 0.38`, `tao 0.26`, `reqwest`, `curl`, `adblock`.
- `src/browser/mod.rs`: native tao window, wry WebView, toolbar initialization script, IPC, new-window interception.
- `src/browser/chrome.rs`: injected toolbar, privacy/BOM scripts, ad/fetch/XHR wrappers, reader shortcut, YouTube ad configuration stripping.
- `src/network/*`: headless reqwest/rustls fetch path.
- `src/privacy/*`: stealth script and Tor detection.

The source does not contain the historical `src/ui/app.rs`/egui implementation. That implementation must be extracted from Git commit `ea726eb`, not copied from this snapshot.

## Confirmed current source defects

### Critical

1. **Download IPC accepts a filesystem path from WebView JavaScript.** `IpcMsg::StartDownload { url, path }` reaches `DownloadManager::start_download()` without path confinement. A page can send an IPC message and attempt arbitrary file overwrite. The current security patch confines output to `downloads/<safe-name>`.
2. **libcurl proxy setup fails open.** If `easy.proxy()` or `easy.proxy_type()` fails, the code logs a warning and continues. Tor-enabled downloads can become direct downloads. The current security patch returns instead of falling back.
3. **The process “sandbox” is a placeholder.** `src/sandbox.rs` prints that Windows Job Objects/Linux Seccomp are engaged but returns `Ok(())` without making those OS calls. `main.rs` therefore reports a sandbox that does not exist.
4. **Sync identity security is inadequate.** The old implementation chooses 12 words from a 12-word dictionary, prints the seed and QR payload, and claims pairing success although transport is not implemented. The current patch changes identity generation to 256-bit entropy, removes secret logging, and returns an explicit unimplemented error for pairing.

### High

5. **Navigation policy is incomplete.** The navigation handler only blocks strings beginning with `javascript:`. `file://`, `data://`, credentials in URLs, malformed schemes, and new-window paths were not subject to one strict URL policy.
6. **`config.toml` Tor proxy is ignored.** `resolve_tor_for_session()` calls `detect_tor_proxy_status()`, which reads the environment/default instead of `config.tor_proxy_url` when no environment override exists.
7. **Tor toggle is mostly cosmetic.** The WebView proxy is fixed before WebView construction. The old `SetTor` handler only logs a note; it does not update the proxy used by subsequent downloads. The patch updates download state and explicitly requires restart for WebView routing.
8. **Exit-country lookup leaves Tor.** `maybe_probe_tor_route()` probes Tor correctly, then calls `ipapi.co` with no proxy. The patch routes that lookup through the Tor proxy.
9. **Permission keys are inconsistent.** `extract_domain()` returns a bare hostname, while `PermissionManager::set_permission()` checks whether the key starts with `https://`. HTTPS permission grants are therefore denied or stored under mismatched keys. The patch uses normalized origins.
10. **Tab isolation is not connected to the WebView.** `TabManager` creates path strings and logs them, but the single wry WebView is reused. This is not per-tab cookie/storage isolation. The patch adds actual cleanup but does not claim full isolation.
11. **History is plaintext in the repository working directory.** `HistoryEngine::file_path()` returns `.catisen_history.json`; it saves synchronously on each visit. The patch moves it to an OS user-data directory, uses an atomic write, and bounds history size.
12. **Current integration tests target a different runtime contract.** `tests/integration_test.rs` invokes `--url`, `--tor`, `CATISEN_LOG_FILE`, and `CATISEN_VIEW_MODE`, but current `main.rs` does not parse those flags and the current browser path does not implement the expected telemetry file contract.

### Medium

13. `src/tor_proxy.rs` is deprecated but still included by `main.rs`, so the old duplicate proxy system still compiles.
14. `ReaderMode::clean_html()` is now unused by the current live WebView path; reader mode uses a separate injected-DOM function. Remove the unused raw-HTML path or test it as a separate product mode.
15. `extensions.rs` is an API stub: it marks a WASM extension loaded without compiling or instantiating WASM.
16. `media_extractor.rs` launches external `mpv`/`vlc` with an unvalidated URL. It is not an embedded media subsystem and is not connected to browser navigation in the supplied source.
17. `debug_panel.rs` builds page DOM with `innerHTML` for URL/title values. Use `textContent` for diagnostic values.
18. `normalize_url()` treats any string containing `://` as loadable. The new policy layer now validates after normalization, but this helper should be documented as formatting only, never authorization.

## Confirmed working current wiring

These are present and connected in the supplied current source:

- adblocker initialization and navigation filtering;
- document-start toolbar script;
- top-frame toolbar guard;
- preservation of `window.chrome.webview` before replacing `window.chrome`;
- same-tab new-window request handler;
- reader-mode IPC and live CSS/DOM transformation;
- sync overlay WebView creation and QR image display;
- download manager invocation from IPC;
- Tor route probe invocation;
- permission manager construction and navigation-time updates.

“Wired” does not mean secure or complete; the defects above apply to several of these paths.

## Patch status

`patches/0003-current-browser-security.patch` is generated against the supplied current source snapshot and passed `git apply --check` in the analysis workspace. It changes nine source files and adds `src/security_policy.rs`.

It still requires:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Cargo was unavailable in the analysis environment, so no claim of a passing Rust build is made.

## Final split conclusion

- The **legacy project** must be extracted from the pre-wry Git commit. Current files cannot recreate it accurately.
- The **current project** should be extracted from `2e03418`, then patched with `0001-workflow-hardening.patch` and `0003-current-browser-security.patch`.
- The split script refuses to proceed if the selected commits do not contain the expected `egui`/`wry` manifests. This prevents silently creating two repositories with the same conflicting identity.
