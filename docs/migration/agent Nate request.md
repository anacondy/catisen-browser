# Agent Nate Request — Catisen handoff

**Prepared by:** Arena.ai agent  
**Date:** 2026-07-19  
**Recipient:** Agent Nate, with GitHub-connected tools  
**Repository:** Catisen Rust browser  
**Priority:** Verify the split, reproduce the patches, and run the actual build/tests against GitHub before making further feature changes.

---

## 1. Executive request to Agent Nate

Please take over from this point as a verification and implementation agent.

The repository has two conflicting product identities:

1. A historical egui/eframe + reqwest/rustls static HTTP/text browser.
2. A current wry/tao embedded WebView browser using WebKitGTK on Linux and WebView2 on Windows.

Do not merge these architectures again. Extract them into two separate repositories from Git history:

| Project | Commit | Identity |
|---|---|---|
| `catisen-legacy` | `ea726eb` | Privacy HTTP client / static text browser. |
| `catisen-browser` | `2e03418` | Current wry/tao embedded browser. |

The migration kit already contains the split script, validation script, patches, documentation, and ZIP package. Use GitHub as the authoritative source for the actual commit contents. Do not use generated build artifacts, screenshots, or status documents as implementation evidence.

---

## 2. Evidence supplied for this handoff

The review was based on these user-provided artifacts:

1. `catisen-folder-tree.txt`
   - Windows tree listing.
   - Shows `src/`, `assets/`, `attached_assets/`, `.cache/`, `target/`, backups, screenshots, logs, and agent configuration.
2. `catisen-commits.txt`
   - 73 commit entries.
   - Shows the pre-wry history through `ea726eb` and the current browser changes through `2e03418`.
3. `catisen-codebase.txt`
   - 1,450 file sections.
   - 1,398 were generated `target/**/*.json` sections.
   - It contained documentation/configuration but omitted the production Rust source.
4. `catisen-rust-source.txt`
   - 32 source/manifest/test sections.
   - Contains the current wry source, not the old egui implementation.
5. The embedded prior audit file:
   - `attached_assets/Catisen25e7c8f_1784053298493.md`
   - Describes the historical egui/HTTP implementation and its prior security findings.

The current source export was reconstructed into an analysis snapshot and inspected file by file. It was not treated as the user's working Git checkout.

---

## 3. Important handoff limitation

The analysis environment did not have `cargo` or `rustc` installed. Therefore:

- no `cargo check` result is claimed;
- no `cargo test` result is claimed;
- no platform build result is claimed;
- no Tor/WebView runtime test was performed;
- no GitHub push or remote history rewrite was performed.

The following validations were performed successfully:

- `git apply --check` for workflow patch `0001`;
- `git apply --check` for reference policy patch `0002`;
- `git apply --check` for current source patch `0003`;
- `git diff --check` after applying patches to a reconstructed current source tree;
- ZIP integrity check using `unzip -t`;
- source/manifest parsing and file reconstruction;
- static source review and targeted line-number inspection.

Run the real Rust and platform tests before trusting the patches.

---

## 4. Deliverables created

All deliverables are in:

```text
Catisen-Audit-2026-07-18/
```

### Main migration package

```text
catisen-migration-kit-2026-07-19.zip
```

SHA-256:

```text
c9a658393f4f51a98350ad941e70e1048440df3df08be7ff96737e4d988043c1
```

### Important files

| File | Purpose |
|---|---|
| `MIGRATION_GUIDE.md` | Full chaptered migration guide from current state through split, build, test, cross-platform work, and rollback. |
| `SPLIT_PLAN.md` | Shorter architecture split plan and file ownership. |
| `DOWNLOAD_GUIDE.md` | Which files the user needs to download and which are analysis-only. |
| `AUDIT_REPORT.md` | Whole-project architecture, security, dead-code, testing, and rating report. |
| `SOURCE_REVIEW_ADDENDUM.md` | Findings confirmed against the current supplied Rust source. |
| `PATCHSET.md` | Patch usage and caveats. |
| `CLEANUP_MANIFEST.md` | Repository clutter and sensitive files to remove. |
| `README.md` | Package overview. |

### Scripts

| File | Purpose |
|---|---|
| `scripts/split-catisen-projects.ps1` | Extracts `ea726eb` and `2e03418` into two destination repositories. |
| `scripts/validate-split.ps1` | Validates different architecture identities and absence of forbidden files. |
| `scripts/cleanup-repository-artifacts.ps1` | Cleans an existing checkout without splitting. |

### Patches

| File | SHA-256 | Purpose |
|---|---|---|
| `patches/0001-workflow-hardening.patch` | `ce61e740a2d128e4d4dcb1ae73909a567ab19c2ea2070190268313ddc6c3e55a` | GitHub Actions safety and CI gates. |
| `patches/0003-current-browser-security.patch` | `0040a76679474b563937241d2e25ee5e51818d66f8af5c54819ff4f53da8a3ed` | Patch against supplied current wry source. |
| `patches/0002-policy-modules-reference.patch` | Reference only. Do not apply in normal flow. | Standalone policy example. |

Split script SHA-256:

```text
6fb967ffe938b5dd4eb5f7245420e806d558c29dd5d4ccdb560cbdcda2344abe
```

---

## 5. Current source architecture confirmed

The supplied current `Cargo.toml` contains:

```toml
wry  = "0.38"
tao  = "0.26"
reqwest = { version = "0.12", ... features = ["rustls-tls", "cookies", "socks"] }
curl = "0.4"
adblock = "0.12"
```

Relevant current source files:

```text
src/main.rs
src/browser/chrome.rs
src/browser/debug_panel.rs
src/browser/ipc.rs
src/browser/mod.rs
src/network/client.rs
src/network/fetch.rs
src/network/error.rs
src/network/text_extract.rs
src/privacy/stealth.rs
src/privacy/tor_manager.rs
src/libcurl_download_manager.rs
src/history.rs
src/permissions.rs
src/sandbox.rs
src/sync_chain.rs
src/tab_isolation.rs
src/text_mode.rs
src/ublock_integration.rs
```

The browser module is activated from `src/main.rs:1`. The source comments describe the current runtime as wry + tao, with WebKitGTK on Linux and WebView2 constraints on Windows.

---

## 6. Confirmed current-source findings

### 6.1 Download path is controlled by IPC input — critical

Evidence:

- `src/browser/ipc.rs` defines `StartDownload { url: String, path: String }`.
- `src/browser/mod.rs:360` maps the IPC message to `AppEvent::StartDownload`.
- `src/browser/mod.rs:579-581` passes the untrusted URL and path directly to `DownloadManager::start_download()`.

Risk:

- A page script can potentially submit an arbitrary filesystem destination.
- The old code had no path confinement.

Patch 0003 introduces a safe filename policy and places output under `downloads/`.

Nate should add a runtime IPC test proving that absolute paths and `..` paths are rejected.

### 6.2 libcurl proxy setup fails open — critical

Evidence from `src/libcurl_download_manager.rs`:

- `easy.proxy(proxy_addr)` is attempted around line 98.
- `easy.proxy_type(ProxyType::Socks5Hostname)` is attempted around line 100.
- Errors were only printed; the transfer continued.

Risk:

A Tor-enabled download could proceed directly if proxy setup failed.

Patch 0003 returns from the download thread on proxy setup failure and limits redirects.

### 6.3 Sandbox is a placeholder — critical

Evidence:

- `src/main.rs:41` calls `SandboxManager::lockdown_current_process()`.
- `src/sandbox.rs:13` labels the implementation `MVP Placeholder`.
- Linux and Windows functions only print messages and return `Ok(())`.

Conclusion:

The process is not actually sandboxed by the supplied source. Documentation claiming active Job Object or Seccomp enforcement is inaccurate.

Nate must either:

1. implement and test the platform sandbox; or
2. remove the security claim and fail closed for a release/security build.

### 6.4 Sync identity is too weak and secret is logged — critical

Evidence from `src/sync_chain.rs`:

- line 23: only 12 words exist in the dictionary;
- lines 29-30: 12 random selections are made;
- line 37: the generated seed is printed;
- line 71: `pair_device()` only prints success and has no transport implementation.

The entropy is approximately `12^12`, around 43 bits, with repeated words allowed. It is not a robust device identity.

Patch 0003 changes identity generation to 256-bit random material, removes secret logging, and makes pairing return an explicit unimplemented error.

### 6.5 Navigation policy is incomplete — high

Evidence:

- `src/browser/mod.rs:281` only explicitly rejects `javascript:`.
- `src/browser/mod.rs:330` handles new-window requests separately.
- `src/browser/mod.rs:739` normalizes any string containing `://` without authorization.

Risk:

`file:`, `data:`, credentials in URLs, malformed schemes, and unsafe new-window destinations were not governed by one policy.

Patch 0003 adds `src/security_policy.rs` and validates initial navigation, normal navigation, new-window navigation, and downloads.

Nate must add callback-level tests, not only unit tests for the policy function.

### 6.6 Configured Tor proxy was ignored — high

Evidence:

- `src/browser/mod.rs:133` calls `resolve_tor_for_session(&config)`.
- The original resolver calls `detect_tor_proxy_status()`.
- The original `src/privacy/tor_manager.rs:90` reads environment/default settings rather than accepting `config.tor_proxy_url`.

Patch 0003 validates and uses the configured value, while allowing `CATISEN_TOR_PROXY` as an explicit override.

### 6.7 Runtime Tor toggle was incomplete — high

Evidence:

- `src/browser/mod.rs` originally declared `active_tor_proxy` immutable.
- The original `SetTor` event only logged that WebView restart was needed.
- Downloads later used the startup `active_tor_proxy` value.

Patch 0003 updates the proxy used by new downloads and clearly states that WebView routing still requires restart.

### 6.8 Tor diagnostic country lookup left Tor — high

Evidence:

- `src/privacy/tor_manager.rs:175` sends the Tor check through the proxy.
- `src/privacy/tor_manager.rs:188` originally called `ipapi.co` with `None` for the proxy.

Patch 0003 routes the country lookup through the same SOCKS proxy.

Nate should consider removing the third-party lookup entirely for a stricter privacy product.

### 6.9 Permission origin keys were inconsistent — high

Evidence:

- `src/browser/mod.rs:294` obtains a bare domain from `extract_domain()`.
- `src/permissions.rs:45` tests whether the domain starts with `https://`.

An HTTPS origin stored as a bare hostname could be treated as insecure or queried under a different key.

Patch 0003 normalizes origins consistently.

### 6.10 History is stored in the repository directory — high

Evidence:

- `src/history.rs:35` returns `.catisen_history.json`.
- `src/history.rs:54` writes directly to that path.
- The first folder/codebase attachments contained the actual history file with browsing URLs and sensitive query data.

Patch 0003 moves history to an application data directory, uses atomic writes, and bounds the number of visits.

The old repository still requires Git history purging if the history file was ever pushed.

### 6.11 Tab isolation is not actual WebView isolation — high

Evidence:

- `src/tab_isolation.rs:42` creates a path per tab.
- `src/browser/mod.rs:426` creates a new logical tab but reuses the same `webview`.
- `src/tab_isolation.rs:65` originally removed the in-memory record but did not delete filesystem state.

Conclusion:

The old code provided path bookkeeping, not true per-tab WebView cookie/storage isolation.

Patch 0003 adds cleanup but deliberately does not claim full isolation.

### 6.12 Current integration tests target a different runtime — high

Evidence:

`tests/integration_test.rs`:

- line 119: invokes `--url`;
- line 121: sets `CATISEN_LOG_FILE`;
- line 123: sets `CATISEN_VIEW_MODE`;
- line 127: invokes `--tor`.

Current `main.rs` only explicitly handles `--headless` and does not implement the expected telemetry-file contract. These tests are not reliable tests of the current browser.

Nate should rewrite them around a real headless command contract or separate core/network tests from GUI tests.

### 6.13 Duplicate deprecated Tor implementation — medium

`src/tor_proxy.rs` explicitly says it is deprecated, but `main.rs` still declares `mod tor_proxy;`. Remove it from the current active module tree after verifying no external callers.

### 6.14 Stubs remain in extensions and media — medium

- `src/extensions.rs` marks a WASM extension loaded without instantiating WebAssembly.
- `src/media_extractor.rs` launches external `mpv`/`vlc`; it is not a browser media implementation.

Either implement, feature-gate, or remove these features from release claims.

---

## 7. Historical implementation status

The old egui implementation source was not included in the source attachment. Do not reconstruct it from current files.

Use the actual Git commit:

```text
ea726eb
```

The split script exports the full commit, then removes generated/runtime clutter. After extraction, inspect:

```powershell
git ls-tree -r --name-only ea726eb
Get-Content .\Cargo.toml
```

Expected identity:

```text
eframe/egui + reqwest/rustls + scraper/static text
```

Nate must verify whether the historical manifest still includes an unused Servo dependency or external visual path. If yes, remove it or put it behind a feature before calling the legacy project stable.

---

## 8. Migration instructions for Nate

### Step A — obtain the kit

Download:

```text
catisen-migration-kit-2026-07-19.zip
```

Extract it outside the source repository.

### Step B — set repository URLs

```powershell
$SourceRepo  = 'https://github.com/YOUR_ACCOUNT/catisen.git'
$LegacyRepo  = 'https://github.com/YOUR_ACCOUNT/catisen-legacy.git'
$CurrentRepo = 'https://github.com/YOUR_ACCOUNT/catisen-browser.git'
```

### Step C — run split and patch current source

```powershell
powershell -ExecutionPolicy Bypass -File `
  .\scripts\split-catisen-projects.ps1 `
  -SourceRepo $SourceRepo `
  -LegacyRepo $LegacyRepo `
  -CurrentRepo $CurrentRepo `
  -ApplyCurrentPatches
```

### Step D — validate

```powershell
powershell -ExecutionPolicy Bypass -File `
  .\scripts\validate-split.ps1 `
  -LegacyRoot 'C:\path\to\legacy-repo' `
  -CurrentRoot 'C:\path\to\current-repo'
```

### Step E — build each independently

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Run those commands separately in both repositories.

### Step F — review Git contents

```powershell
git ls-files | Select-String '(^target/|^\.cache/|\.catisen_history|Screenshot_|Pasted-|\.bak-)'
```

This must return no lines.

### Step G — do not push before review

Review the generated diff, build output, patch results, and source ownership first. Push only after the GitHub-connected agent has verified the correct commit and no secrets.

---

## 9. Pure cross-platform recommendation

The current browser should become a Cargo workspace later:

```text
catisen-core       pure Rust policy/network/history/download crate
catisen-browser    wry/tao browser shell
linux adapter      WebKitGTK
windows adapter    WebView2
```

Keep GTK, WebKitGTK, WebView2, winapi, mpv/vlc, and native window code out of `catisen-core`.

The realistic target is shared Rust source plus platform-specific native dependencies. A zero-native-dependency WebView build is not realistic with WebKitGTK/WebView2.

---

## 10. Confidence classification

### Directly verified from source or supplied artifacts

- current manifest uses wry/tao;
- current source contains browser, IPC, Tor, permissions, history, downloads, sync, and sandbox modules;
- current history path is repository-local;
- current download proxy setup logs and continues after errors;
- current sync seed is 12 selections from a 12-word dictionary and is printed;
- current sandbox functions are placeholders;
- current integration tests invoke unsupported CLI/telemetry behavior;
- current tab manager paths are not connected to separate WebViews;
- commit history contains `ea726eb` and `2e03418` at the stated dates/messages;
- ZIP and patches pass the static checks described above.

### Historical findings requiring GitHub confirmation

- exact file list at `ea726eb`;
- whether the historical build currently compiles without removing Servo;
- exact old `src/ui/app.rs` behavior;
- whether old visual/snapshot paths remain in the historical commit;
- whether sensitive files were pushed into remote Git history.

### Not tested here

- Cargo compilation;
- Cargo unit/integration tests;
- Linux WebKitGTK runtime;
- Windows WebView2 runtime;
- Tor routing;
- real downloads;
- browser IPC from hostile pages;
- GitHub remote operations.

---

## 11. Required response from Nate

Please report back with:

1. Source repository URL and exact commit hashes resolved.
2. Confirmation that `ea726eb` is egui/HTTP and `2e03418` is wry/tao.
3. File list for each exported repository.
4. `cargo fmt`, `cargo check`, `cargo test`, and `cargo clippy` results for both.
5. Platform build results for Linux and Windows.
6. Whether any sensitive files exist in current or previous Git history.
7. Which current patch hunks apply cleanly.
8. Any compile errors caused by patch 0003.
9. A separate list of unresolved limitations; do not label stubs as complete.
10. Confirmation that the two repositories no longer share conflicting architecture claims.

Do not report a screenshot, generated status file, or successful `git apply` as proof that the browser works. The required proof is source inspection plus reproducible build/test output.
