# Catisen Browser — Engineering Audit

**Audit date:** 2026-07-18  
**Review basis:** `catisen-folder-tree.txt`, `catisen-commits.txt`, `catisen-codebase.txt`, and the embedded prior audit `attached_assets/Catisen25e7c8f_1784053298493.md`.

## Important scope limitation

The supplied `catisen-codebase.txt` does **not contain the Rust source tree**. It contains 1,450 file sections: 1,398 generated `target/**/*.json` files and documentation/configuration artifacts. The folder tree lists `src/`, but `src/*.rs` files are absent from the codebase export. Therefore:

- I could not compile, run tests, inspect the current Rust implementation, or safely edit the production source.
- Findings explicitly marked **verified** come from supplied workflow/config/history artifacts.
- Findings marked **carry-forward** come from the embedded July 2 audit and must be rechecked against the current July 18 source before closing them.
- The included patch bundle contains executable repository/workflow cleanup patches and source-level reference patches. It does not claim that absent Rust-source changes were applied.

This was a data-integrity problem in the first handoff. A later source attachment supplied the current wry Rust tree; see `SOURCE_REVIEW_ADDENDUM.md` and `patches/0003-current-browser-security.patch` for the source-level findings and patch. The historical egui source is still absent from the attachments and must be extracted from Git commit `ea726eb`.

---

## Executive assessment

Catisen currently has two conflicting identities:

1. **Actual historical implementation:** a Rust/egui privacy-oriented HTTP fetcher with source/text views, diagnostics, cookie jars, Tor-aware `reqwest`, and a heuristic/external-browser visual path.
2. **Current intended implementation:** a wry/tao embedded WebView browser with document-start privacy scripts, IPC, navigation interception, and real DOM/BOM/JavaScript rendering.

The July 15–18 commits indicate a significant browser-module refactor, including new-window interception, top-frame guarding, reader-mode shortcuts, YouTube configuration stripping, and preservation of `window.chrome.webview`. Those changes are promising, but they cannot be verified from the supplied export.

### Ratings

| Area | Rating | Assessment |
|---|---:|---|
| Code quality | **4/10** | Rust gives memory safety, but the project history shows large unverified changes, dead/legacy paths, monolithic UI ownership, and weak gates. |
| Architecture against the browser goal | **3/10** | The documented historical runtime was not a browser engine; the current wry architecture is not verifiable from the handoff. |
| Architecture as a privacy HTTP client | **6/10** | The reqwest/rustls/Tor/text pipeline is a reasonable foundation if all egress paths share one policy. |
| Security posture before remediation | **2/10** | History/cookie exposure, egress inconsistency, external renderer risk, and self-modifying CI are release blockers. |
| Test maturity | **3/10** | Many scripts and reports exist, but several reports contain contradictory PASS results, missing telemetry, and zero-byte download artifacts. |
| Documentation accuracy | **2/10** | README, status, TODOs, reports, and architecture notes disagree materially. |
| Repository hygiene | **1/10** | Generated build output, runtime state, screenshots, pasted agent context, backups, and logs are present in the project tree. |
| **Overall release readiness** | **3/10** | Not ready to present as a secure general-purpose browser. |

**Release decision:** **No-go** until the egress policy is unified, sensitive state is removed from the repository history, source is compiled in CI, and browser-engine behavior is tested end to end.

---

## Verified repository and handoff problems

### 1. Sensitive browsing history is present in the supplied project data — critical

**Verified from:** root `.catisen_history.json` in the codebase export.

The file contains a large browsing history, including search queries, test URLs, onion URLs, fingerprint-test URLs, and long Google sign-in URLs with sensitive query material. Even if those URLs are expired, committing or handing them around violates the stated privacy goal and creates avoidable disclosure risk.

**Impact:** privacy breach, possible session/link-token disclosure, user profiling, and permanent exposure if the file is present in Git history.

**Required fix:**

- Remove `.catisen_history.json` from the working tree and Git index.
- Add it to `.gitignore`.
- Rewrite Git history if it was ever pushed.
- Rotate/revoke any credentials or link tokens that may have appeared in URLs.
- Store history under an OS user-data directory, not the repository.

### 2. Runtime cookie/profile state is in the project tree — critical

**Verified from:** folder tree `.cache/catisen/global_profile` and numerous isolated tab profiles.

The project contains runtime profile directories. The prior audit also identified plaintext cookie jars. A local `.gitignore` entry is not sufficient if the files were previously committed or copied into an archive.

**Required fix:** purge `.cache/` from the index and history, use a per-user application-data directory, set restrictive filesystem permissions, and encrypt or protect cookies using the platform credential store where feasible.

### 3. Build output is mixed with source/documentation — high

**Verified from:** 1,398 `target/**/*.json` sections in the codebase export and the folder tree.

`target/` must never be part of a source handoff or commit. It adds noise, increases review time, leaks build paths and dependency details, and can conceal the absence of real source files.

**Required fix:** delete `target/`, ignore it, and verify with `git ls-files target` that it is not tracked.

### 4. Agent screenshots, pasted prompts, and local agent configuration are repository clutter — medium/high

**Verified from:** `attached_assets/` and `.continue/` in the folder tree.

The tree contains:

- seven `Screenshot_*.png` files;
- multiple `image_*.png` files;
- `Pasted-*.txt` prompt/context captures;
- a local MCP configuration with a Windows filesystem path;
- `.agents/memory` and `.continue` local-agent state.

These are not product source. The MCP configuration also invokes floating `npx -y` packages and grants filesystem access, which is a supply-chain and developer-environment risk.

**Required fix:** remove screenshots/pasted captures from the product repository; keep only intentional design assets under `assets/`; keep agent/MCP state outside Git; pin tool versions if such tooling must be retained.

### 5. Changelog workflow interpolates an untrusted commit message into a shell command — high

**Verified from:** `.github/workflows/changelog-on-push.yml`.

The workflow directly embeds `${{ github.event.head_commit.message }}` inside a shell `echo` command. A crafted commit message can alter the generated shell script after GitHub expression substitution. The workflow also has write permission and pushes back to `main`.

**Required fix:** pass the commit message through `env:` and print it as a data argument with `printf`; avoid workflow self-triggering; restrict permissions to the job that writes the changelog; add CI validation.

The supplied patch `patches/0001-workflow-hardening.patch` replaces this workflow and improves CI.

---

## Carry-forward security findings from the embedded prior audit

These findings were reported against the earlier source snapshot and are not independently verifiable without `src/`.

### SEC-01 — external visual renderer can bypass Tor — critical

The prior audit reports that visual mode can spawn Chrome/Edge/Firefox through a snapshot bridge. If that process is not explicitly configured with the same SOCKS5 hostname proxy, it performs a direct connection while the main `reqwest` request uses Tor.

**Failure mode:** the user sees a Tor-fetched page or enables Tor, but the visual renderer performs a second direct fetch. This is a privacy leak and a false security guarantee.

**Safe default:** when Tor is enabled, external snapshot rendering must be disabled. The only acceptable alternative is an engine/process whose every network request is proven to use the same proxy and DNS policy.

### SEC-02 — libcurl download path can bypass Tor — critical

The prior audit reports that the libcurl download manager did not configure a proxy, while `reqwest` did. A binary download therefore has a different egress path from navigation.

**Safe default:** use one shared proxy policy and configure libcurl with `CURLOPT_PROXY` plus SOCKS5-hostname mode, or route downloads through the same engine/network client. Do not silently fall back to direct networking when Tor is requested.

### SEC-03 — URL scheme validation is insufficient — high

The prior audit reports that `javascript:`, `file:`, and other non-web schemes could reach visual/external paths. A browser navigation policy must allow only `http` and `https` unless a scheme is deliberately implemented and isolated.

**Required behavior:** reject `javascript:`, `file:`, `data:`, `blob:`, `about:`, shell commands, credentials embedded in URLs, and malformed/oversized URLs before any WebView, external process, download manager, or history write receives them.

### SEC-04 — two Tor systems create policy drift — high

The earlier implementation had `privacy/tor_manager.rs` and a separate `tor_proxy.rs` with different defaults and responsibilities. There must be one `ProxyPolicy` object used by navigation, subresources, downloads, diagnostics, and any external helper.

### SEC-05 — plaintext cookie/history storage — high

Per-tab separation is not equivalent to confidentiality. Plain JSON cookie jars and repository-local history expose state to other local users, backups, crash uploads, and accidental commits.

### SEC-06 — auto-delete settings were UI-only — high

A checkbox is not a control. The application must register an exit/shutdown cleanup path, close all stores, delete session cookies and temporary profiles, and report cleanup failures. It must also clean a tab profile when a tab is closed.

### SEC-07 — sync identity/seed handling is weak — high

The earlier audit reports a weak 12-word seed and stdout exposure. A sync secret must be generated from a cryptographically secure source, never logged, encrypted at rest, rate-limited, and transferred over authenticated encrypted transport. A display-friendly QR payload must not itself be the long-term secret.

### SEC-08 — ad-blocking claims exceed implementation — medium/high

The earlier code reportedly used a small custom substring matcher rather than a complete uBlock-compatible rules engine. This is both a correctness problem and a privacy problem: subresource requests may be allowed while the UI claims protection.

### SEC-09 — no demonstrated renderer sandbox — high

The status documents claim process sandboxing, while the prior audit found no verified multiprocess isolation. A browser engine handling hostile web content should not share the full application privilege boundary. Treat `sandbox.rs` as untrusted until its initialization, failure behavior, and platform coverage are tested.

### SEC-10 — debug telemetry logs full URLs — medium

Request logs contain full URLs and query strings. This can capture search terms, account flows, tokens, and private paths. Redact query strings by default and use an explicit opt-in for full diagnostic URLs.

### SEC-11 — `X-Forwarded-For` is not geolocation or privacy spoofing — medium

Adding a forged forwarding header does not change the network source address and can create misleading server-side behavior. It must not be described as IP anonymization.

### SEC-12 — third-party exit-country lookup is an extra egress — medium

If Tor diagnostics sends the observed exit IP to a separate clearnet geolocation service, it creates an unnecessary external request. Prefer local country mapping or omit the country lookup. If retained, route it through the same declared policy and disclose it in diagnostics.

### SEC-13 — floating dependencies and agent tooling — medium

The `.continue/mcpServers/new-mcp-server.yaml` uses `npx -y` package resolution. This is not part of the browser runtime, but it is unsafe repository tooling. Pin versions, restrict filesystem scope, or remove it from the product repository.

---

## Broken, dead, or misleading areas

The following were reported in the embedded audit and are supported by the folder/commit evidence, but current source wiring needs verification:

| Item | Problem | Action |
|---|---|---|
| Servo dependency | Historically compiled but not imported; large build cost and attack surface | Remove until integrated, or put behind an explicit feature and test that feature. |
| `text_mode.rs` / reader mode | Earlier path was declared but not called | Verify the current `Alt+R` path reaches one implementation; remove the old path if not. |
| `tor_proxy.rs` vs `tor_manager.rs` | Duplicate proxy policy and conflicting defaults | Consolidate into one module and one typed configuration. |
| Visual snapshot bridge | External browser is not a browser-engine integration and may bypass Tor | Disable under Tor; replace with embedded engine or document the limitation. |
| Servo spike renderer | Heuristic paint/manifests are diagnostic output, not DOM/CSS rendering | Keep under `spikes/` or remove from the product path. |
| GTK shell | Earlier audit found a shell without an embedded webview | Do not advertise it as a browser until navigation and WebView rendering are connected. |
| Download handoff | Reports show successful HTTP 200 but zero-byte downloaded files | Test file size, checksum, atomic rename, errors, cancellation, and proxy use. |
| FFmpeg/media code | Earlier audit identified a stub/hardcoded path | Remove or feature-gate until a real media pipeline exists. |
| Sync chain | Earlier audit identified a stub | Do not expose a UI claiming pairing is functional. |
| `*.bak-*`, `*.bak2-*` | Multiple source backups are in `src/` and `src/browser/` | Remove from product tree; use Git branches/tags instead. |
| Root test files and logs | Ad hoc tests/logs are not Cargo tests and may contain private URLs | Move tests into `tests/`; ignore/delete runtime logs. |
| Generated reports | Many near-duplicate reports and contradictory PASS/FAIL outcomes | Keep one canonical report per run plus machine-readable results. |

### Current commit-history warning

The latest commit messages are feature claims, not test evidence. In particular:

- `2e03418`: top-frame guard, reader mode, YouTube stripping;
- `79c1686`: new-window request interception;
- `262ee48`: preserving `window.chrome.webview`;
- `2058980`: security fixes and UI wiring;
- `97c3ffa`: wiring dead privacy/security infrastructure.

These should be treated as **unverified until the actual diff and CI result are reviewed**. The supplied source omission prevents confirming whether the fixes are complete, duplicated, or dead code.

---

## Architecture review

### Historical runtime

The documented prior runtime was effectively:

```text
URL bar -> reqwest/rustls -> HTML string -> source/text/heuristic visual output
```

That architecture is acceptable for a privacy-focused text client. It cannot provide live DOM, CSS layout, JavaScript, media, service workers, WebSockets, or browser-compatible event behavior.

### Intended current runtime

The July documentation describes:

```text
native window -> tao/wry WebView -> document-start policy scripts
             -> IPC -> Rust policy/network/download/history services
```

This is the right direction, but a secure browser requires the following invariant:

> Every network-capable path must use one typed egress policy, and every untrusted navigation must pass one URL policy before it reaches the WebView, IPC, external process, download manager, history, or diagnostics.

The current project history shows that this invariant was not established early. Multiple engines, UI paths, Tor mechanisms, and one-off patches were added incrementally. That explains why features exist but do not fire: ownership and lifecycle are not centralized.

### Main architectural weaknesses

1. **No single source of truth for runtime mode.** Default egui, optional GTK, historical Servo, snapshot rendering, and current wry notes coexist.
2. **UI owns too much.** Navigation, networking, settings, diagnostics, downloads, and rendering decisions were historically concentrated in `ui/app.rs`.
3. **Side effects are not capability-scoped.** A function can fetch, log, render, or download without receiving an explicit proxy/privacy policy.
4. **Feature flags do not define a tested product matrix.** A feature can compile without being used at runtime.
5. **No lifecycle model.** Tabs, cookie stores, renderer processes, downloads, and shutdown cleanup do not have one owner.
6. **Testing is report-driven rather than assertion-driven.** A report can say PASS while a download is zero bytes or telemetry is missing.
7. **Documentation is treated as completion evidence.** Status files use “complete” language for components that the audit calls stubs or unwired.

### Recommended target architecture

```text
Application
├── policy/          URL, scheme, origin, Tor, download and permission policy
├── egress/          one reqwest/curl/WebView proxy configuration
├── browser/         one WebView backend and typed IPC
├── tabs/            tab lifecycle, isolated storage, cleanup
├── content/         reader/source/document presentation
├── downloads/       bounded, resumable, verified downloads
├── privacy/         document-start scripts and capability decisions
├── diagnostics/     redacted structured events
└── ui/              presentation only
```

The browser process should be separated from renderer content when the selected engine supports it. If not, the application must state that it is not a security sandbox.

---

## Required remediation order

### P0 — before any further feature work

1. Remove and rotate exposed history/session URL material.
2. Purge `.cache`, `target`, logs, screenshots, pasted agent captures, and source backups from the Git index and history.
3. Fix the changelog workflow shell injection and self-trigger behavior.
4. Add strict `http`/`https` navigation policy.
5. Make Tor mode fail closed for visual helpers and downloads.
6. Ensure every request path reports its selected egress policy.

### P1 — next engineering cycle

7. Attach the actual `src/`, `Cargo.toml`, and current `Cargo.lock` to the audit input.
8. Run `cargo fmt --check`, `cargo check`, `cargo test`, `cargo clippy`, and platform-specific build checks in CI.
9. Add unit tests for URL policy, proxy policy, redirects, cookie cleanup, new-window requests, top-frame IPC, and download integrity.
10. Remove duplicate/legacy modules or put them behind explicit feature flags.
11. Split UI orchestration from services.
12. Replace ad-blocking substring claims with a tested rules engine or rename the feature honestly.

### P2 — product hardening

13. Use OS-managed application-data directories and encrypted cookie storage.
14. Implement real tab lifecycle and shutdown cleanup.
15. Add renderer isolation/sandboxing appropriate to the chosen engine.
16. Redact diagnostics and make full URL logging opt-in.
17. Replace generated report sprawl with a canonical JSON schema and one summary Markdown report.

---

## Acceptance criteria for closing this audit

- `git ls-files` contains no `target/`, `.cache/`, `.catisen_history.json`, runtime logs, screenshots, pasted prompts, or `*.bak-*` files.
- Git history has been checked and sensitive files have been purged if previously pushed.
- A non-Tor navigation, Tor navigation, WebView subresource, external-helper attempt, and download each show their egress policy in redacted diagnostics.
- Visual mode is blocked or demonstrably proxied when Tor is enabled.
- Downloads cannot silently fall back to direct networking and must produce a non-zero verified file for a known test fixture.
- `javascript:`, `file:`, `data:`, `blob:`, credentials-in-URL, and malformed URLs are rejected before side effects.
- New-window requests are handled by the same policy as top-level navigation.
- Document-start scripts are scoped to the top frame and preserve required platform objects such as `window.chrome.webview`.
- CI builds the actual current source and runs tests; no status document is accepted as a substitute for a passing job.

---

## Final verdict

Catisen has a viable foundation for a privacy-oriented text browser and a promising current wry direction. It is not yet defensible as a secure general-purpose browser because the repository contains sensitive runtime state, the handoff omits the production source, historical egress paths were inconsistent, and the documentation overstates completion.

The previous agent’s screenshot commits are not meaningful progress. They are repository contamination and should be removed. The correct next step is not another UI screenshot or status document; it is a clean source handoff followed by P0 security remediation and reproducible CI evidence.
