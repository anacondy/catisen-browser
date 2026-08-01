# Catisen Browser — Current Repository Audit

**Audit date:** 2026-08-01 (UTC)  
**Audit type:** read-only code, security, privacy, architecture, and repository-state review  
**Requested scope:** determine which findings from the supplied earlier-agent analysis belong to this repository and which do not  
**No production code was changed.** This deliverable adds only this Markdown report.

---

## 1. Straight answer first

The earlier analysis was **not performed against the checked-out revision of this repository**. That distinction is material:

- The checked-out branch, `arena/019fbfaf-catisen-browser`, is at commit `7bbd9ac` (the same commit as `origin/main`).
- That commit contains only `README.md` and `LICENSE`.
- It contains no `Cargo.toml`, no `src/`, no `tests/`, no workflows, and no browser implementation to compile or audit.
- Therefore, none of the earlier source-level findings can honestly be called findings in the current default branch: the implementation is not present there yet.

The repository does, however, contain remote non-default branches with the split browser source. I reviewed those without switching this working branch:

| Reference | Commit | Role | Result |
|---|---|---|---|
| `main` / current checkout | `7bbd9ac` | Default branch | Metadata-only repository shell; not a buildable browser. |
| `migration/browser-2e03418` | `d2ef623` | Current-browser migration baseline and the base of open PR #3 | Full wry/tao browser source; audited as the split-point implementation. |
| `arena/019f819c-catisen-browser` | `3489fcf` | Latest visible unmerged development/audit branch | Full source plus several P0/P2 patches; audited as the most advanced source snapshot available in this repository. |

**Conclusion:** the earlier analysis is directionally relevant to the wry/tao browser source in the migration/development branches, but it is not evidence about `main` as currently checked out. Several of the earlier agent's claimed fixes are also **not present** in the latest source snapshot I inspected. They must not be applied or accepted based only on that prior transcript.

For the WebView2/Edge product specifically, the latest source is a promising embedded-browser baseline, but it is **not release-ready as a privacy/security product**. The most important unresolved issue is that page JavaScript and privileged toolbar IPC share the same WebView trust boundary without a real authenticated capability mechanism.

---

## 2. Scope, provenance, and limitations

### Sources inspected

1. The checked-out repository and branch shown above.
2. The remote GitHub tree for `migration/browser-2e03418` at `d2ef6239a3386e232c812092e554243f5d1a1084`.
3. The remote GitHub tree for `arena/019f819c-catisen-browser` at `3489fcffd3a338d4ae5d5bf42582cc5b4dafb0fa`.
4. The repository's open pull-request metadata and GitHub Actions run metadata.
5. The supplied earlier-agent analysis in the request, used as a comparison checklist—not as proof that its edits exist here.

The remote source branches were extracted to a temporary directory outside the working tree for inspection. This did not switch, merge, or modify the working branch.

### Verification constraints

- `cargo` and `rustc` are not installed in this analysis environment. No local Rust compilation, `cargo test`, or `cargo clippy` result is claimed.
- The latest source branch has no recorded CI run in the repository's visible Actions history. The available CI runs are for the earlier audit/probe branch and all ended in failure:
  - [run 29773551224](https://github.com/anacondy/catisen-browser/actions/runs/29773551224)
  - [run 29773745448](https://github.com/anacondy/catisen-browser/actions/runs/29773745448)
  - [run 29790487820](https://github.com/anacondy/catisen-browser/actions/runs/29790487820)
- Node.js was available. Four extractable JavaScript raw strings from the latest source (`TOOLBAR_JS`, `YT_ADBLOCK_JS`, and the two debug/settings panel scripts) passed `node --check`.
- An instantiated `privacy_init_js` string with ordinary profile values passed syntax checking. A hostile value containing the quote/backslash sequence targeted by the manual escaping pattern produced invalid JavaScript, confirming that the escaping implementation is not robust. The current call site supplies static profile literals, so this is currently a latent/API-level risk rather than a demonstrated remote-page-to-parameter exploit.
- The generated reader-mode JavaScript failed syntax checking for all three themes because the inserted font values contain single quotes inside single-quoted JavaScript strings. This is a directly reachable functional defect in the latest source.

---

## 3. Repository-state findings

### R-01 — The checked-out/default branch is not a browser implementation — **Critical release/blocker**

**Evidence:** `git ls-files` on the current branch returns only:

```text
LICENSE
README.md
```

`README.md` describes a wry/tao embedded WebView browser, but the manifest and source tree are absent. The default branch cannot be built, tested, or launched as a browser from this checkout.

**Meaning:** this is not a source vulnerability; it is a repository/product-state failure. Any review that reports the Rust implementation as part of `main` is reviewing a different ref.

**Required decision:** before calling the default branch the current browser product, decide which migration/development branch is the source of truth and merge or otherwise publish the browser source through the normal review process. Do not silently treat a remote feature branch as `main`.

### R-02 — The earlier report and its proposed patch list are not a reliable current-state manifest — **High process risk**

The supplied transcript says that many files were rewritten or patched. The latest source snapshot does not match those claims in several important places:

- `src/sandbox.rs` is still a no-op on Windows and Linux.
- `src/browser/chrome.rs` still uses quote-only escaping and aborts XHR in `open()`.
- `src/history.rs` still persists plaintext history synchronously and has no onion exclusion or opt-out.
- `src/tab_isolation.rs` still only tracks path strings and does not delete profile data.
- `src/config.rs` still has the dead `unwrap_or_default` method.
- `src/network/fetch.rs` still performs one request, returns no status code, and does not use the retry/error infrastructure.
- `src/ublock_integration.rs` still hardcodes request type `script` and uses the request URL as the source URL.
- `src/browser/mod.rs` still logs the Tor toggle without applying or persisting it.
- `src/main.rs` still lacks the claimed `--url`, `--tor`, `CATISEN_VIEW_MODE`, and `CATISEN_LOG_FILE` support.

The transcript should therefore be treated as an unmerged/partially applied work log, not as the state of this repository.

### R-03 — Documentation, manifest, and workflow drift — **Medium**

The latest source branch has several inconsistent claims:

- `README.md` reports `cargo check: PASS`, `cargo test: PASS`, and Windows WebView2 launch success, but the current default branch has no manifest/source and CI has no Windows job.
- `Cargo.toml` sets `license = "MIT"`, while the tracked `LICENSE` is Apache License 2.0.
- `.gitignore` ignores `Cargo.lock`, although this is a binary application where a committed lockfile is important for reproducible builds.
- Documentation describes `--workspace` commands although the manifest is a single package, not a workspace.
- Several feature comments call the system “strict,” “isolated,” or “Tor routed” while the implementation remains partial.

These discrepancies are security-relevant because privacy/security claims are part of the user-facing threat model.

---

## 4. Executive assessment of the actual wry/WebView2 source

This rating applies to the latest source snapshot at `3489fcf`, not to the empty default branch.

| Area | Rating | Assessment |
|---|---:|---|
| Embedded-browser direction | 6/10 | wry/tao with WebView2 on Windows and WebKitGTK on Linux is a coherent product direction. |
| Runtime architecture | 5/10 | Navigation, toolbar JS, IPC, history, downloads, Tor helpers, and policy helpers exist, but ownership and trust boundaries are not mature. |
| Security posture | 2/10 | No application sandbox and no authenticated privileged IPC are release blockers. |
| Privacy/Tor correctness | 3/10 | Some download and proxy plumbing exists, but toggles, headless routing, malformed proxies, local bypasses, and subresource coverage remain unsafe or misleading. |
| Functional completeness | 4/10 | Browser navigation and UI scaffolding are present; tabs, permissions, reader mode, sync pairing, and ad blocking are incomplete. |
| Test/CI confidence | 2/10 | No local Rust toolchain here; CI only checks/builds on Ubuntu and does not run tests/clippy or Windows WebView2 builds. |
| Documentation/repository hygiene | 3/10 | The split documentation is useful, but claims and branch state are inconsistent and the source tree retains orphaned legacy files. |
| **Release readiness** | **2/10** | **No-go** for a product marketed as a secure/private browser. |

---

## 5. Confirmed findings in the latest browser source

Line references below refer to `arena/019f819c-catisen-browser` at `3489fcf`, unless a different ref is explicitly named.

### C-01 — Privileged IPC is not authenticated to a trusted browser UI — **Critical**

**Evidence:**

- `src/browser/chrome.rs:207-210, 398-410, 449-486` exposes the toolbar and `window.__cat` inside the same page context as arbitrary website JavaScript.
- `src/browser/debug_panel.rs:165` sends commands using `window.ipc.postMessage(...)`.
- `src/browser/mod.rs:411-446` accepts and dispatches all parsed IPC messages.
- `src/security_policy.rs:121-142` only rejects a small scheme blocklist and explicitly allows ordinary `https`, `http`, `ipc`, and unparseable values.
- `src/browser/ipc.rs:42-60` includes privileged actions such as starting downloads, changing Tor/isolation/adblock settings, changing permissions, and navigating.

This is not an origin allowlist. A normal HTTPS page is allowed by the policy, and the browser intentionally exposes the same IPC mechanism to the page context where the toolbar runs. A malicious page can therefore attempt to post the same JSON messages as the toolbar.

The path-confinement patch reduces one impact, but does not authenticate the caller. A page can still trigger downloads into the browser's download directory, fill disk, change privacy settings, alter permission records, and cause navigation or other user-visible actions. If any future IPC command accepts a broader path or privileged value, the same missing trust boundary becomes a direct local compromise path.

**Required fix:** move browser chrome to a native UI or a separately trusted internal WebView/origin; authenticate IPC with an unguessable per-window capability established outside page JavaScript; validate the actual source origin against an exact allowlist; and make all privileged commands fail closed. A blocklist of `javascript:`, `data:`, and `file:` is not sufficient.

### C-02 — The declared process sandbox is still a no-op — **Critical**

**Evidence:** `src/sandbox.rs:5-25` prints “not implemented” and returns `Ok(())` on both Windows and Linux. `src/main.rs:29-47` calls it, but an `Ok(())` result only produces a warning and continues.

There is no `PR_SET_NO_NEW_PRIVS`, seccomp filter, Windows Job Object, AppContainer, restricted token, or equivalent application-level enforcement in this source. WebView2 has its own engine architecture, but that does not make the Catisen host process an implemented application sandbox.

**Required fix:** either implement and test a platform-specific containment design, or remove “strict sandbox” language and explicitly document the trust boundary. Do not accept a successful no-op as a security control.

### C-03 — Tor mode can silently become clearnet or ignore runtime changes — **Critical privacy finding**

There are several distinct failures:

1. **Runtime toggle is inert:** `src/browser/mod.rs:686-699` only logs `SetTor`; it does not update `active_tor_proxy`, `config.use_tor_by_default`, or the download manager. The settings UI presents a control that does not perform the promised runtime action.
2. **Headless mode is fail-open:** `src/main.rs:65-109` uses `probe_tor()` and then selects `None` (direct clearnet) whenever the probe fails. It does this even when a Tor route was explicitly configured; it does not fail closed.
3. **Headless mode ignores the configured enable flag:** it uses a reachable local proxy if one is found, regardless of whether `config.use_tor_by_default` is enabled.
4. **Hostname proxies are tested incorrectly:** `src/main.rs:149-162` attempts `addr.parse::<SocketAddr>()` and silently falls back to `127.0.0.1:9150`. A configured hostname proxy is not necessarily the address that was tested.
5. **Malformed proxy can skip WebView proxy configuration:** `src/browser/mod.rs:327-341` logs a parse failure and continues building the WebView without `with_proxy_config`. If Tor was requested, that is a potential direct-network fallback.
6. **Proxy reachability is only a TCP check:** `src/privacy/tor_manager.rs:65-97` does not perform a SOCKS handshake or prove that the endpoint is a Tor relay/proxy. A listening non-Tor service can be treated as reachable.

The country lookup was correctly changed to use the proxy at `src/privacy/tor_manager.rs:187-195`; that particular clearnet leak is fixed in this source. It does not solve the broader route-selection failures above.

**Required fix:** represent egress as one typed policy (`Disabled`, `Required(proxy)`, `BestEffort`) and pass it to every network path. If `Required(Tor)` cannot be established, abort the request/browser path instead of selecting clearnet. Use DNS-aware endpoint resolution and a SOCKS handshake/known Tor check. Make the runtime toggle update persisted state and the download policy, while clearly requiring restart for a WebView proxy that cannot be changed live.

### C-04 — Hardware permissions are recorded, not reliably enforced — **High**

**Evidence:**

- `src/permissions.rs:75-114` stores normalized host keys but has no connection to a WebView permission callback.
- `src/browser/mod.rs:355-367` auto-denies only when the URL literally begins with lowercase `http://` and does not contain the substring `.onion`.
- `src/browser/mod.rs:511-539` installs a JavaScript geolocation override only after page-load completion.
- `src/browser/mod.rs:701-724` accepts `SetPermission` from IPC without proving the caller or origin and without a secure-origin parameter.
- Camera, microphone, and notification behavior is not blocked by corresponding native WebView policy code.

Consequences include early page scripts getting a chance to access geolocation before the post-load override, camera/microphone requests reaching the engine's default behavior, and a page being able to submit permission mutations through the unauthenticated IPC channel.

The normalization also collapses scheme and port: `https://example.com:8443` and `http://example.com:80` are keyed as `example.com`. Permission decisions should be based on an exact origin (scheme, host, and effective port), not only a hostname.

**Required fix:** enforce permissions before page JavaScript runs using the platform/WebView permission mechanism or a document-start policy; use exact origin keys; authenticate settings commands; and fail closed for unknown hardware requests.

### C-05 — Navigation policy allows local/private destinations and does not require Tor for `.onion` — **High privacy finding**

**Evidence:** `src/security_policy.rs:43-118` permits any parsed HTTP/HTTPS host, including `localhost`, loopback, RFC1918, link-local, and other private addresses. It does not require `.onion` navigation to have an active Tor route. `src/browser/mod.rs:358-364` also treats any URL containing the string `.onion` as an exception, even when the substring is in a path or a larger hostname.

This is not necessarily a normal-browser bug—browsers commonly permit localhost—but it is a serious mismatch for a Tor/privacy product. A web page can navigate the browser to local admin panels or private network services, and an `.onion` URL can be attempted without a verified Tor route. Linux also explicitly sets `no_proxy` for localhost/127.0.0.1 at `src/browser/mod.rs:145-153`.

**Required fix:** define the intended local-network policy. In strict privacy mode, block private/loopback/link-local targets unless the user explicitly allows them. Require a verified Tor route for `.onion` hosts, and parse the URL's hostname rather than using a substring test.

### C-06 — Downloads are lexically confined but not safe against symlinks, overwrite, size, or IPC abuse — **High**

**Evidence:**

- `src/security_policy.rs:88-115` strips directory components and checks `starts_with(base)`, which is a useful traversal mitigation.
- `src/browser/mod.rs:647-669` creates a relative `downloads` directory and accepts the requested basename from IPC.
- `src/libcurl_download_manager.rs:132-153` opens the path with `create(true).append(true)` and follows filesystem links.
- The same file uses multiple `unwrap()` calls in a detached thread (`:85-87, 113, 130, 136-153`).
- `follow_location(true)` is enabled (`:86-87`) without redirect policy, maximum size, status validation, or atomic temporary-file handling.

A pre-existing symlink under the downloads directory can redirect the write outside the intended directory. Existing files are appended to rather than safely replaced or atomically committed. A page that can reach IPC can trigger large downloads and disk exhaustion. A server that returns a full body when a resume was requested can corrupt the result by appending the full response to the partial file.

**Required fix:** open files with a symlink-safe strategy appropriate to Windows and Unix, use a per-download temporary file, use `create_new`/atomic rename semantics, reject device names and Windows ADS syntax, enforce maximum bytes and redirect limits, validate HTTP status/content policy, and make download errors update structured state rather than panic a worker thread. IPC authentication is required in addition to path validation.

### C-07 — History remains plaintext, persistent, and synchronous; onion visits are not excluded — **High privacy finding**

**Evidence:** `src/history.rs:31-145`.

The latest branch improved the location from the repository CWD to a hand-built OS data directory and added a 5,000-visit bound plus a temporary file/rename write. It did **not** add:

- an opt-out or private-browsing mode;
- onion filtering;
- encryption or platform credential-store protection;
- restrictive file permissions;
- asynchronous writes;
- a robust Windows replace/rename strategy.

`record_visit()` calls `self.save()` on every navigation (`:115-132`) from the browser event loop. On Windows, `std::fs::rename` generally cannot replace an existing destination file, so the fixed `.tmp` rename strategy needs a Windows-specific replacement path or subsequent writes may fail. The source also persists `.onion` URLs as ordinary visits.

**Required fix:** define private mode and history retention semantics, keep onion/private visits out of persistent history by policy, write under the platform data directory with restrictive permissions, use an atomic cross-platform replacement strategy, and move serialization/I/O off the UI thread. Treat history as sensitive even when it is not a cryptographic secret.

### C-08 — EasyList integration is not a complete subresource blocker — **High feature/privacy finding**

**Evidence:**

- `src/ublock_integration.rs:95-110` calls `AdRequest::new(url, url, "script")` for every request.
- `src/browser/mod.rs:369-378` invokes that method only from the navigation handler.
- `src/browser/chrome.rs:521-568` adds a DOM iframe observer and cosmetic filters; `:761-780` intercepts only fetch/XHR in the page context.
- `assets/easylist.txt` has 296 lines, of which approximately 179 are non-comment rules, and is a deliberately trimmed list.

The engine receives the wrong resource type and same URL as both request/source URL, so type-specific and third-party rules are evaluated incorrectly. The navigation handler is not a universal subresource interception point. Images, stylesheets, scripts loaded by the DOM, WebSockets, service workers, and browser-managed requests are not all covered by the JS wrappers. The page can also replace or evade the injected observers.

**Required fix:** use a WebView/network interception API that supplies actual resource type and initiator/origin, or rename the feature to best-effort cosmetic/JS filtering. Do not advertise the result as full uBlock/EasyList protection. Mutex poisoning should also fail closed rather than `.unwrap_or(true)` allowing the request.

### C-09 — Reader mode generates invalid JavaScript — **High functional defect**

**Evidence:** `src/browser/mod.rs:773-813` constructs CSS as single-quoted JavaScript string fragments, while the `font` values at `:775-781` themselves contain single quotes, for example:

```javascript
'font-family: 'Fira Code', sans-serif !important;'
```

Static instantiation and `node --check` failed for `MentalityDark`, `TechManual`, and `TerminalBlue`.

The Rust `text_mode.rs` implementation also retains a separate raw-HTML `clean_html()` path (`src/text_mode.rs:35-91`) that has no active caller; the live browser uses a different JS implementation. This duplicate path increases drift and is still documented with a stale Servo-era comment.

**Required fix:** serialize CSS values safely or use template literals/JSON encoding, add generated-script syntax tests for every theme, and remove or explicitly quarantine the dead raw-HTML implementation.

### C-10 — XHR ad blocking leaves the XHR object unopened — **High compatibility defect; latent page-script breakage**

**Evidence:** `src/browser/chrome.rs:772-780` returns from `XMLHttpRequest.prototype.open()` when the URL matches an ad domain without calling the original `open()` and without installing a safe `send()` behavior.

Page code that subsequently calls `send()` receives the browser's `InvalidStateError`. This can break unrelated application code on a page, not merely suppress an ad request.

**Required fix:** always call the original `open()` and mark the request as blocked, then make `send()` abort in a defined way; or use a proper network interception layer. Add a browser-level regression test covering a blocked XHR followed by unrelated page activity.

### C-11 — Privacy-script string escaping is incomplete — **Medium now; High if inputs become configurable**

**Evidence:** `src/browser/chrome.rs:661-679, 711-715` escapes only single quotes using `.replace('\'', "\\'")` and interpolates into JavaScript string literals. It does not robustly encode backslashes, line terminators, U+2028/U+2029, or other JavaScript-string hazards.

A hostile parameter can change the generated script's parse or execution. The current `browser::run()` call uses `pick_profile()` static literals, which limits reachability today, but `privacy_init_js()` is public and its API accepts arbitrary strings. The safe pattern is to serialize each value using `serde_json::to_string` or equivalent JavaScript-string serialization and splice the resulting quoted literal without adding another layer of quotes.

The same general rule applies to the URL-bar update at `src/browser/mod.rs:541-544`, which performs only backslash and single-quote replacement before building an evaluated script. URL parsing limits practical control, but this should still use a serializer.

### C-12 — Network retry/error/status infrastructure is dead or bypassed — **Medium/High reliability and policy defect**

`src/network/error.rs` contains rich timeout, retry, Tor, and client-init variants, but `src/network/fetch.rs:4-9` performs one `send()` and returns only `(HeaderMap, String)`. It discards the HTTP status code and never consults `CatisenConfig.request_retries` or `request_timeout_secs` (the client also hardcodes 30 seconds in `src/network/client.rs:18-29`).

This means network failures do not receive the intended retry behavior or structured user-facing errors. More importantly, a caller cannot distinguish a successful HTTP response from a 404/500 based on the fetch API. The earlier transcript's claimed retry/status fix is not present in the latest source.

**Required fix:** implement one typed request function that carries route policy, timeout, retry budget, status, headers, and body; make every caller use it; test timeout, proxy failure, redirect, non-2xx, and cancellation cases.

### C-13 — Tab isolation is cosmetic, and cleanup still deletes nothing — **High product/privacy defect**

**Evidence:** `src/tab_isolation.rs:40-73` creates path strings but does not create/configure WebView storage contexts. `src/browser/mod.rs:499-504` creates a `TabManager` record and then navigates the same single `webview`.

Closing a tab only removes the map entry and prints a cleanup message. It never deletes the profile directory. As a result:

- there is one active WebView/cookie context, not one per tab;
- “multi-account” isolation is not implemented;
- stale profile data can accumulate if directories are ever created.

**Required fix:** model tabs as owned WebViews with explicit storage/data directories, track the active tab/window, close and securely remove the correct profile, and test cookie separation across two tabs. Until then, change the UI copy to say “planned” rather than “Strict Tab Isolation.”

### C-14 — Sync identity generation is improved, but pairing is not implemented — **Medium product/security completeness**

**Fixed relative to the old source:** `src/sync_chain.rs:22-35` now generates 32 random bytes (256 bits), does not print the seed, and `pair_device()` fails with `Err("not implemented")`. This is materially better than the earlier 12-word/approximately-43-bit seed and fake pairing message.

**Still unresolved:** `src/browser/debug_panel.rs:229-239` exposes a “Generate New Sync Identity & QR Code” control, and `src/browser/chrome.rs:250-252` still labels the flyout “Device sync chain,” while there is no transport, key exchange, or pairing flow. The QR/seed is a bearer secret shown to the user, so it needs explicit lifecycle, revocation, and secure-display semantics before sync can be marketed as a feature.

### C-15 — Fingerprint “consistency” is only partial — **Medium privacy quality**

`pick_profile()` at `src/privacy/stealth.rs:157-183` selects Windows, Linux, or macOS identity tuples at random. The tuple is internally consistent, which fixes one earlier mismatch, but it is not necessarily consistent with the actual WebView2 host platform, screen, fonts, GPU, input devices, and engine behavior. On Windows/WebView2, randomly claiming Linux or Mac can make the browser more distinctive rather than less.

The page also receives broad synthetic values (`hardwareConcurrency = 8`, `deviceMemory = 8`, fake plugins, fake WebGL vendor/renderer) and the host HTTP UA is separately set through `.with_user_agent(&ua)` at `src/browser/mod.rs:327-330`. This is a best-effort anti-fingerprinting layer, not anonymity. It needs a documented threat model and measured consistency tests rather than “strict” language.

### C-16 — Browser chrome is page DOM, so the page can tamper with the UI — **High design risk**

Even apart from IPC authentication, the toolbar is mounted into `document.documentElement` by `TOOLBAR_JS`. A hostile page can remove, cover, replace, or observe page-DOM UI. The MutationObserver attempts to remount it, but the observer itself is also page JavaScript and can be disconnected or monkey-patched.

This is especially important because the browser uses the same page context for address-bar input, privacy badge, settings, and privileged commands. A native toolbar or an isolated trusted UI surface is the safer design. The current approach can be acceptable as a prototype UI, but not as a security boundary.

---

## 6. Findings from the supplied earlier-agent list: current status

This table directly checks the claims in the supplied transcript against the latest source snapshot.

| Earlier claim | Status in latest source | Evidence / conclusion |
|---|---|---|
| `chrome.rs` now uses `serde_json` escaping | **Not applied** | `src/browser/chrome.rs:661-665` still uses quote-only replacement. |
| XHR ad-block intercept no longer leaves objects unopened | **Not applied** | `src/browser/chrome.rs:772-780` still returns from `open()` without opening. |
| Flyout no longer overstates tab isolation/device sync | **Not applied** | `src/browser/chrome.rs:248-252` still presents “Tab isolation (multi-account)” and “Device sync chain.” |
| Permission scheme-prefix bug fixed and wildcard/global behavior fixed | **Partially applied** | Host normalization and wildcard lookup exist in `permissions.rs`, but the runtime secure-origin checks are case-sensitive, host-only, and not applied to the IPC setter. Exact scheme/port origin semantics are still missing. |
| `sandbox.rs` is now a real Linux/Windows sandbox | **Not applied** | Both platform functions still print “not implemented” and return `Ok(())`. |
| `libc` and Windows Job Object dependency features were added | **Not applied** | Latest `Cargo.toml` has only `winapi = { version = "0.3", features = ["winuser"] }` and no Linux `libc` dependency. |
| History opt-out, onion exclusion, async writes, and permissions fixed | **Not applied / partially diverged** | History moved to an OS-like data path and added a bound/temporary file, but still persists onion URLs, has no opt-out/encryption/permissions, and writes synchronously. |
| Dead `config::unwrap_or_default` removed | **Not applied** | `src/config.rs:76-79` still defines it. |
| Sync seed changed from weak repeated words to stronger entropy | **Applied, but differently** | Latest source uses 256-bit random bytes rendered as 64 hex characters, which is stronger than the proposed 214-word list. Pairing remains unimplemented. |
| `tab_isolation::close_tab` now deletes the directory | **Not applied** | `src/tab_isolation.rs:65-71` only logs a message. |
| Tor country lookup now uses the Tor proxy | **Applied** | `src/privacy/tor_manager.rs:187-195` passes `Some(&proxy_addr)`. |
| Fetch retry/status logic wired | **Not applied** | `src/network/fetch.rs` remains a one-request, headers/body-only function. |
| Adblock checks use real type and initiator/source | **Not applied** | `src/ublock_integration.rs:103-107` still hardcodes `script` and uses the same URL for source. |
| Browser call sites and `SetTor` were fixed/persisted | **Partially applied / mostly not** | Navigation and some managers are wired, but adblock remains old, `SetTor` only logs, and there is no persistence in that event handler. |
| `main.rs` implements `--url`, `--tor`, TextOnly view, log file, and correct Tor probing | **Not applied** | Latest `main.rs` recognizes only `--headless` and `--no-network`; `probe_tor` still has the hardcoded fallback and headless mode can fall back to clearnet. |
| Dead Rust reader HTML functions removed | **Not applied** | `src/text_mode.rs:35-91` still contains `clean_html`/`remove_html_tag`; the live browser uses a separate JS path. |

### What *is* clearly present from the later hardening work

The latest branch does include several meaningful improvements that should be preserved and tested:

- centralized URL normalization and an HTTP/HTTPS navigation check;
- rejection of credential-bearing URLs;
- lexical download-path confinement;
- Tor proxy use in the libcurl download manager, including SOCKS5-hostname mode when configured;
- proxy use for the Tor exit-country lookup;
- a separate native sync-display window rather than putting the seed in the main page DOM;
- 256-bit sync identity generation with no seed logging;
- permission key normalization and wildcard lookup;
- debug/settings panel construction with `textContent` instead of attacker-controlled `innerHTML` for URL/title values;
- consistent internal UA/platform/language/timezone tuples;
- an attempted WebView2/WebKit proxy configuration path through `with_proxy_config`.

These are partial controls, not proof of a secure browser. In particular, URL validation and path confinement do not compensate for an unauthenticated privileged IPC channel.

---

## 7. CI, testing, and build-gate assessment

### Current workflow weaknesses

`.github/workflows/ci.yml` in the source branches:

- runs only on Ubuntu;
- executes `cargo check` and `cargo build`;
- does not run `cargo test`;
- does not run `cargo clippy`;
- does not run `cargo fmt --check`;
- does not build the Windows/WebView2 target;
- does not test runtime IPC, URL policy, downloads, Tor-required behavior, or generated JavaScript.

The repository's README therefore claims more validation than the workflow enforces. The failed historical CI runs listed above are the only visible compile evidence and are not a green gate for the latest branch.

### Test coverage is concentrated in pure helper modules

The latest source has unit tests for policy normalization, permission-map behavior, sync entropy, adblock samples, history serialization, tab-map bookkeeping, and a headless smoke test. It does not have effective end-to-end tests for:

- the WebView2 Windows path;
- actual origin-authenticated IPC;
- new-window requests and OAuth behavior;
- document-start permission enforcement;
- WebView subresource interception;
- Tor-required fail-closed routing;
- malformed/hostname/IPv6 proxies;
- symlink-safe Windows downloads;
- reader-mode generated JS for each theme;
- history replacement on Windows;
- real tab cookie separation.

The `--no-network` smoke test still calls `probe_tor()` before it skips HTTP fetches, so it is not truly network-free. It also exercises a diagnostic pipeline rather than the GUI/WebView2 runtime.

### Workflow shell-injection risk

`.github/workflows/changelog-on-push.yml` directly interpolates `${{ github.event.head_commit.message }}` inside a shell `run` block and has write permission to contents. A commit message containing shell syntax can alter the generated command after expression expansion.

**Required fix:** pass the message through an `env:` mapping and use `printf '%s\n' "$COMMIT_MESSAGE"`; limit permissions, avoid self-trigger loops, and add workflow linting.

---

## 8. Recommended remediation order

### P0 — do before feature work or release claims

1. Decide and document the source-of-truth branch; do not call the metadata-only `main` a working browser.
2. Replace page-context privileged IPC with a trusted/native channel or a real authenticated capability model.
3. Stop treating `sandbox.rs`'s `Ok(())` as sandbox engagement; implement platform enforcement or remove the claim.
4. Make Tor-required mode fail closed across GUI, headless, WebView, downloads, redirects, diagnostics, and every helper. Fix the runtime toggle and hostname proxy probing.
5. Enforce exact origins and real WebView permission callbacks before page JavaScript runs.
6. Fix download symlink/overwrite/size/redirect handling and remove worker-thread `unwrap()` calls.
7. Add CI gates for `cargo fmt --check`, `cargo check --all-targets`, `cargo test --all-targets`, and `cargo clippy -- -D warnings`, plus a Windows/WebView2 build job.

### P1 — next engineering cycle

8. Fix generated reader JS and XHR interception; add Node/browser syntax and behavior regression tests.
9. Replace manual JavaScript interpolation with JSON serialization everywhere evaluated scripts are built.
10. Finish history policy: private mode, onion policy, restrictive permissions/platform protection, asynchronous writes, and cross-platform atomic replacement.
11. Implement actual per-tab WebView/storage ownership and cleanup, or remove the isolation claim.
12. Replace the best-effort adblock path with a real resource-interception path that has resource type and initiator context.
13. Wire the existing typed network error/retry/status policy into every request path.
14. Remove or quarantine orphaned `gtk_ui.rs`, `media_extractor.rs`, duplicate reader code, and stale Servo-era documentation.

### P2 — product hardening

15. Finish sync pairing only with a reviewed transport, key exchange, revocation, and secure seed lifecycle; otherwise keep it clearly marked unimplemented.
16. Align fingerprint policy with the actual Windows WebView2 host rather than random unsupported OS identities.
17. Commit `Cargo.lock`, correct the license metadata, and add dependency/security scanning.
18. Maintain one canonical current report and one machine-readable test result rather than treating status documents as evidence.

---

## 9. Acceptance criteria

The audit should not be closed until all of the following are demonstrable on the source-of-truth branch:

- `main` or the explicitly designated release branch contains the actual wry/WebView2 source and a committed lockfile.
- CI builds the Windows WebView2 target and runs Rust tests/clippy/format checks.
- A normal web page cannot issue privileged IPC commands.
- `javascript:`, `data:`, `blob:`, `file:`, `about:`, credential-bearing, malformed, and policy-disallowed private-network URLs are rejected before side effects.
- Tor-required requests abort when the proxy is absent, malformed, unreachable, or not verified; they never silently become clearnet.
- WebView2 navigation, subresources, downloads, diagnostic calls, redirects, and helper requests share one egress policy.
- Hardware permission decisions use exact origins and are enforced before page script execution.
- Downloads are symlink-safe, bounded, status-checked, temporary/atomic, and robust to cancellation and write errors.
- History has explicit private-mode/onion/retention behavior and is stored with platform-appropriate protection.
- Two real tabs have separate cookie/storage contexts and close-time cleanup is tested.
- Generated JavaScript passes syntax and behavioral tests for all themes and blocked-resource paths.
- UI copy does not claim “strict isolation,” “device sync,” “sandbox,” or “full ad blocking” before those capabilities exist.

---

## 10. Final verdict

The split decision was correct: the wry/tao WebView2 browser belongs in `catisen-browser`, and the older egui/HTTP/Servo-oriented material should not be mixed into it. The current GitHub repository has not yet made that browser source the default branch, however, so the checked-out `main` is currently a repository shell rather than a browser application.

The source on the migration/development branches is a credible prototype and contains several good hardening steps. The earlier agent's findings are substantially relevant to that source, but the transcript overstates how many fixes are present. The latest source still has critical IPC and sandbox gaps, privacy fail-open paths, incomplete permissions, unsafe downloads, persistent plaintext history, incomplete ad blocking, nonfunctional tab isolation, and a reader-mode script defect.

**Release decision: NO-GO for a secure/private browser.** Keep the existing pull requests open for review; do not merge them as evidence of completion. The next work should establish the source-of-truth branch and close the P0 trust-boundary/egress/sandbox issues before additional UI features.

---

## Appendix A — Useful repository references

- Default repository: <https://github.com/anacondy/catisen-browser>
- Open audit PR #3: <https://github.com/anacondy/catisen-browser/pull/3>
- Open temporary CI probe PR #2: <https://github.com/anacondy/catisen-browser/pull/2>
- Open WIP PR #1: <https://github.com/anacondy/catisen-browser/pull/1>
- Migration baseline tree: <https://github.com/anacondy/catisen-browser/tree/migration/browser-2e03418>
- Latest inspected development tree: <https://github.com/anacondy/catisen-browser/tree/arena/019f819c-catisen-browser>

**Working-tree state after report creation:** only the report file is newly added on `arena/019fbfaf-catisen-browser`; no production source, workflow, branch, or pull request was modified.
