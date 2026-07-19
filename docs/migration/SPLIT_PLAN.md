# Catisen split plan: two coherent repositories

## Decision

Create two repositories from Git history. Do **not** copy the current working tree into both repositories. The current tree mixes two different products.

| Repository | Source of truth | Product identity | Supported scope |
|---|---|---|---|
| `catisen-legacy` | Commit `ea726eb` — 2026-05-28 | Historical egui/HTTP privacy client | reqwest/rustls fetches, static HTML source/reader views, diagnostics and optional Tor-aware HTTP. |
| `catisen-browser` | Commit `2e03418` — 2026-07-18 | Current wry/tao embedded browser | WebView DOM/BOM/JS, injected toolbar/privacy scripts, IPC, reader mode, EasyList, downloads, sync overlay and browser policy. |

The commit selection is checked by the supplied history and by `Cargo.toml` validation in `scripts/split-catisen-projects.ps1`:

- `ea726eb` must contain `eframe` or `egui`.
- `2e03418` must contain `wry` or `tao`.

If either check fails, stop. Do not force the split using guessed files.

## Important accuracy point

The newly supplied Rust source export contains only the **current wry implementation**. It contains no old `src/ui/app.rs`, old egui `main.rs`, or old `servo_renderer.rs` implementation. The historical implementation can therefore only be extracted accurately from its Git commit, not reconstructed by copying current files.

## Current repository file ownership

The current wry repository should retain these active files:

```text
Cargo.toml
Cargo.lock
assets/easylist.txt
src/main.rs
src/config.rs
src/browser/chrome.rs
src/browser/debug_panel.rs
src/browser/ipc.rs
src/browser/mod.rs
src/network/client.rs
src/network/error.rs
src/network/fetch.rs
src/network/mod.rs
src/network/text_extract.rs
src/privacy/mod.rs
src/privacy/stealth.rs
src/privacy/tor_manager.rs
src/history.rs
src/libcurl_download_manager.rs
src/permissions.rs
src/sandbox.rs
src/sync_chain.rs
src/tab_isolation.rs
src/text_mode.rs
src/ublock_integration.rs
src/extensions.rs
src/media_extractor.rs
tests/integration_test.rs
tests/servo_integration_test.rs
```

`src/tor_proxy.rs` is deprecated and should be removed from `main.rs` and then deleted after a clean build proves there are no external callers. `test_font.rs` and `test_dom_sanitization.rs` are root-level ad hoc programs, not product tests.

## Historical repository file ownership

The legacy repository should be exported from `ea726eb` as a complete historical snapshot, then cleaned. Do not manually mix in current `src/browser/*` files. The old implementation is expected to contain the earlier egui/network path described in the prior audit, including some or all of:

```text
src/main.rs
src/ui/app.rs
src/ui/mod.rs
src/network/*
src/config.rs
src/history.rs
src/tab_isolation.rs
src/text_mode.rs
src/tor_proxy.rs
src/ublock_integration.rs
src/libcurl_download_manager.rs
src/sync_chain.rs
src/servo_renderer.rs
src/debug_panel.rs
assets/*
scripts/*
tests/*
```

The exact list must come from `git ls-tree -r --name-only ea726eb`; it must not be inferred from the current tree.

## Copy/paste PowerShell command

Create two empty GitHub repositories first. Replace the three URLs with the actual URLs. Run the command from the directory containing this deliverables folder:

```powershell
$SourceRepo  = 'https://github.com/YOUR_ACCOUNT/catisen.git'
$LegacyRepo  = 'https://github.com/YOUR_ACCOUNT/catisen-legacy.git'
$CurrentRepo = 'https://github.com/YOUR_ACCOUNT/catisen-browser.git'

powershell -ExecutionPolicy Bypass -File `
  .\Catisen-Audit-2026-07-18\scripts\split-catisen-projects.ps1 `
  -SourceRepo $SourceRepo `
  -LegacyRepo $LegacyRepo `
  -CurrentRepo $CurrentRepo `
  -ApplyCurrentPatches
```

The script:

1. clones the source repository;
2. verifies the two commits identify the expected architectures;
3. exports each commit independently;
4. copies each export into its own new repository clone;
5. removes `target`, `.cache`, history, screenshots, pasted prompts, backups, logs, and local agent state;
6. writes an `ARCHITECTURE.md` file into each repository;
7. optionally applies the current-browser security/workflow patches;
8. does not push automatically.

Review before committing:

```powershell
Set-Location $env:TEMP # replace with the WorkDir printed by the script
# or open the two paths printed by the script

git -C .\legacy-repo status --short
git -C .\current-repo status --short

git -C .\legacy-repo ls-files | Select-String '(^target/|^\.cache/|\.catisen_history|Screenshot_|Pasted-|\.bak-)'
git -C .\current-repo ls-files | Select-String '(^target/|^\.cache/|\.catisen_history|Screenshot_|Pasted-|\.bak-)'
```

The `Select-String` commands must return no lines.

After the split and before committing, validate the identities:

```powershell
powershell -ExecutionPolicy Bypass -File `
  .\Catisen-Audit-2026-07-18\scripts\validate-split.ps1 `
  -LegacyRoot $WorkDir\legacy-repo `
  -CurrentRoot $WorkDir\current-repo
```

Use the actual `WorkDir` printed by the split script. After review:

```powershell
git -C .\legacy-repo add -A
git -C .\legacy-repo commit -m 'chore: split legacy privacy HTTP client'
git -C .\legacy-repo push -u origin main

git -C .\current-repo add -A
git -C .\current-repo commit -m 'chore: split current wry browser'
git -C .\current-repo push -u origin main
```

## Current-browser patch applied by the script

`patches/0003-current-browser-security.patch` fixes the source that was supplied:

- validates every top-level and new-window URL as `http`/`https`;
- rejects credentials and `.onion` navigation without Tor;
- protects download destinations from arbitrary IPC paths;
- blocks direct fallback if libcurl proxy setup fails;
- limits redirects;
- rejects zero-byte successful downloads;
- makes `config.toml` proxy settings effective;
- routes exit-country diagnostics through Tor instead of direct clearnet;
- prevents privacy/YouTube scripts from installing inside frames;
- fixes HTTPS permission state matching;
- stores history outside the repository with atomic writes and a bounded size;
- removes secret logging and the weak 12-word sync seed;
- performs actual isolated-profile cleanup;
- makes the Tor toggle update new-download routing while clearly requiring restart for WebView proxy changes;
- fails closed instead of falling back to clearnet in the headless pipeline when Tor is explicitly enabled.

This patch was checked with `git apply --check`. Cargo could not be run in the analysis environment because Rust/Cargo is not installed here. Run the full build gates in VS Code after applying it.

## Current-browser issues that remain intentionally visible

The split does not magically make these complete:

1. **Windows WebView2 Tor routing:** the source itself documents that wry 0.38 does not expose a WebView2 proxy API. Environment variables are not enough. The Windows repository must either add supported WebView2 proxy configuration, use a newer supported wrapper, or clearly ship Tor as download-only on Windows.
2. **Tab isolation:** the current `TabManager` creates filesystem paths, but the single WebView is not replaced by per-tab WebView contexts. This is not full cookie isolation yet.
3. **Sandbox:** `sandbox.rs` contains placeholder functions that only print success. It is not a real sandbox. Remove the claim or implement platform enforcement before release.
4. **IPC trust boundary:** page JavaScript and the injected toolbar share a WebView context. Privileged IPC messages must eventually move to a native UI channel or use a properly authenticated capability. Download path traversal is patched, but IPC should not be treated as a security boundary.
5. **Integration tests:** `tests/integration_test.rs` invokes `--url` and `--tor`, but current `main.rs` does not parse those arguments and the browser path does not honor `CATISEN_LOG_FILE` telemetry. These tests are not valid current-browser tests and must be rewritten.
6. **Sync transport:** identity generation is hardened, but actual device pairing remains intentionally unimplemented and must not be advertised as working.

## Legacy repository stabilization

After the legacy export is created:

```powershell
Set-Location C:\path\to\catisen-legacy

git grep -n -E 'servo|SnapshotBridge|Chrome|Firefox|browser executable' -- Cargo.toml src tests || $true
git grep -n -E 'http_proxy|https_proxy|libcurl|start_download|HistoryEngine' -- src
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
```

For the legacy project, remove the unused Servo dependency and external visual renderer from the supported build unless that exact commit actually imports them. Rename the project description to “privacy HTTP client / text browser” so it no longer promises a full browser engine.

The legacy project must also receive the same history/cache cleanup and a strict URL policy. It must not retain the old external visual snapshot path under a privacy/Tor claim.

## Do not share these files between the repositories

- Do not copy current `src/browser/*` into the legacy repository.
- Do not copy old egui `src/ui/*`, old Servo spike files, or old `servo_renderer.rs` into the current browser repository.
- Do not share `config.toml` or runtime profiles between them.
- Do not share `.catisen_history.json`, `.cache`, `target`, screenshots, reports with raw URLs, or agent state.
- If common code is later needed, extract a small separately tested crate with a stable API. Do not duplicate source files by hand.
