# Catisen remediation patch set

## What is included

| File | Status | Purpose |
|---|---|---|
| `patches/0001-workflow-hardening.patch` | **Ready to apply** | Fixes shell interpolation in changelog workflow; adds native WebView dependencies, format/test/clippy gates, permissions, and concurrency. |
| `patches/0003-current-browser-security.patch` | **Ready for supplied wry source** | Adds strict URL/Tor policy, confines downloads, prevents direct proxy fallback, fixes history storage, permissions, sync entropy, and Tor configuration. |
| `scripts/cleanup-repository-artifacts.ps1` | **Ready to run** | Removes target/runtime state, screenshots, pasted agent captures, source backups, local agent configuration, and sensitive history from the working tree/index; hardens `.gitignore`. |
| `patches/0002-policy-modules-reference.patch` | Reference only | Standalone policy modules for projects that need them; do not apply without wiring them into the active crate. |
| `reference/security_policy.rs` | Reference source | Readable copy of the policy module added by patch 0002. |
| `reference/proxy_policy.rs` | Reference source | Readable copy of the proxy module added by patch 0002. |

## Apply the repository patch

From the actual repository root:

```powershell
git checkout -b fix/catisen-audit-2026-07-18
git apply Catisen-Audit-2026-07-18/patches/0001-workflow-hardening.patch
```

The CI patch assumes the current Linux build uses GTK/WebKitGTK. If the final backend is wry with different system packages, keep the Rust checks and adjust only the apt package list.

## Clean the repository

Run the cleanup script with preview first:

```powershell
powershell -ExecutionPolicy Bypass -File .\Catisen-Audit-2026-07-18\scripts\cleanup-repository-artifacts.ps1 -WhatIf
powershell -ExecutionPolicy Bypass -File .\Catisen-Audit-2026-07-18\scripts\cleanup-repository-artifacts.ps1

git status --short
git ls-files target .cache .catisen_history.json attached_assets
```

The second command is destructive in the working tree. Review `git status` before committing.

The cleanup deliberately removes:

- `target/`, `.cache/`, `.continue/`, `.agents/`;
- `.catisen_history.json`, logs, error captures, `test.bin`;
- `Screenshot_*.png`, `image_*.png`, and `Pasted-*.txt` under `attached_assets/`;
- `*.bak-*` and `*.bak2-*` under source directories.

It preserves the prior audit by moving it to `docs/reference/prior-audit-2026-07-02.md`.

## Purge sensitive history if it was pushed

Removing a file from the current commit does not remove old Git blobs. First rotate any credentials or link tokens that appeared in history. Then, after coordinating with all repository users:

```powershell
git filter-repo --force `
  --path .catisen_history.json `
  --path .cache `
  --path target `
  --path-glob 'attached_assets/Screenshot_*.png' `
  --path-glob 'attached_assets/image_*.png' `
  --path-glob 'attached_assets/Pasted-*.txt' `
  --path-glob '*.bak-*' `
  --path-glob '*.bak2-*' `
  --invert-paths

git push --force-with-lease --all origin
git push --force-with-lease --tags origin
```

Use `git log --all -- .catisen_history.json .cache` before and after the rewrite to verify removal. Do not force-push a shared repository without coordination.

## Apply the current wry source patch

After exporting commit `2e03418` into the current-browser repository, apply:

```powershell
git apply Catisen-Audit-2026-07-18/patches/0001-workflow-hardening.patch
git apply Catisen-Audit-2026-07-18/patches/0003-current-browser-security.patch
```

`0003` was generated against the supplied `catisen-rust-source.txt` and passed `git apply --check`. Cargo was unavailable in the analysis environment, so run the full Rust gates after applying it.

## Apply the standalone policy reference only when needed

`0002-policy-modules-reference.patch` is a valid addition patch, but adding a module without wiring it would create more dead code. Apply it only with these integration changes in the same commit:

1. Add a direct dependency to `Cargo.toml`:

   ```toml
   url = "2.5"
   ```

2. Add the modules to the active crate module tree. Use the project’s actual layout, for example:

   ```rust
   mod proxy_policy;
   mod security_policy;
   ```

3. Validate the initial URL before constructing the WebView:

   ```rust
   let initial_url = security_policy::validate_navigation(
       &configured_url,
       config.use_tor_by_default,
   )?;
   ```

4. Use the same validation in the wry navigation handler. The handler must return `false` for rejected URLs and must not write rejected URLs to history:

   ```rust
   .with_navigation_handler({
       let tor_enabled = config.use_tor_by_default;
       move |raw_url| {
           match security_policy::validate_navigation(&raw_url, tor_enabled) {
               Ok(url) => {
                   // Record only security_policy::redact_url(&url) in diagnostics.
                   true
               }
               Err(error) => {
                   eprintln!("navigation blocked: {error:?}");
                   false
               }
           }
       }
   })
   ```

5. Apply exactly the same policy to `with_new_window_req_handler`. A new-window request is navigation, not a trusted exception. Preserve the latest top-frame guard and `window.chrome.webview` compatibility from commits `2e03418`, `79c1686`, and `262ee48` while adding this check.

6. Make visual helpers fail closed under Tor:

   ```rust
   if security_policy::allow_external_renderer(tor_enabled).is_err() {
       // Do not spawn Chrome/Edge/Firefox. Use embedded WebView content or
       // return an explicit “visual helper disabled in Tor mode” state.
   }
   ```

7. Configure libcurl from the validated proxy. Never silently fall back to direct networking:

   ```rust
   let proxy = proxy_policy::validate_tor_proxy(&proxy_url)?;
   easy.proxy(proxy.as_str())?;
   if proxy.scheme() == "socks5h" {
       easy.proxy_type(curl::easy::ProxyType::Socks5Hostname)?;
   }
   ```

8. Route diagnostics through the same policy or remove third-party exit-country lookups. Do not log full query strings.

9. Add tests for the actual callback paths, not only the policy functions:

   - top-level `https://example.com` allowed;
   - `javascript:`, `file:`, `data:`, `blob:` rejected;
   - credentials in URL rejected;
   - `.onion` rejected when Tor is off and allowed when Tor is on;
   - new-window request receives the same result;
   - external renderer is not spawned with Tor enabled;
   - download uses SOCKS5-hostname proxy;
   - rejected navigation is absent from history.

## Download correctness patch requirements

The supplied reports show HTTP 200 responses with zero-byte output files. The active download manager must:

- write to a random temporary file in the destination directory;
- check every libcurl return value and transfer result;
- verify the received byte count against `Content-Length` when present;
- atomically rename only after success;
- delete partial files on failure/cancel;
- enforce a maximum file size and redirect limit;
- expose failure state to the UI;
- use the same Tor/proxy policy as page navigation.

A zero-byte file must never be reported as a successful download.

## Verification commands

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings

git diff --check
git ls-files | Select-String '(^target/|^\.cache/|\.catisen_history\.json|Screenshot_|Pasted-|\.bak-)'
```

The last command must return no product files. Run the browser’s real headless test matrix only after these gates pass; do not accept a report generated by a script that does not fail on missing telemetry or zero-byte output.
