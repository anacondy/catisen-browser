# Catisen two-project migration kit

**Document version:** 1.0  
**Prepared:** 2026-07-19  
**Purpose:** split the mixed Catisen repository into two independent GitHub repositories without copying the wrong architecture, runtime secrets, screenshots, generated output, or dead legacy files.

---

## Chapter 1 — What is happening now

The current repository contains two different products:

### Product A — historical implementation

The historical implementation is an egui/eframe privacy HTTP client and static text browser. Its data flow is approximately:

```text
URL input
  -> reqwest/rustls request
  -> HTML string
  -> source view / reader text / heuristic visual output
```

It does not provide the same browser engine behavior as Chrome, Firefox, WebKit, or WebView2.

### Product B — current intended implementation

The current implementation is a wry/tao embedded browser:

```text
native window
  -> wry WebView
  -> WebKitGTK on Linux / WebView2 on Windows
  -> document-start privacy and toolbar scripts
  -> Rust IPC and network services
```

These products must not continue sharing one repository identity, one README, one Cargo manifest, or one feature claim.

---

## Chapter 2 — What was verified

The supplied commit history identifies these split points:

| Product | Commit | Evidence |
|---|---|---|
| Historical egui/HTTP client | `ea726eb` | Last pre-wry implementation in supplied commit history, dated 2026-05-28. |
| Current wry browser | `2e03418` | Latest supplied commit, dated 2026-07-18, with wry/tao/browser-module changes. |

The supplied Rust source attachment contains the current wry implementation and its manifest. It does not contain the old `src/ui/app.rs` historical source. Therefore the historical files are extracted from Git history rather than guessed or copied from the current tree.

The split script verifies the Cargo manifest before copying:

- the historical commit must contain `eframe` or `egui`;
- the current commit must contain `wry` or `tao`.

If those checks fail, stop. Do not continue with an architecture guess.

---

## Chapter 3 — What is inside this ZIP

### Primary operation files

| File | Use |
|---|---|
| `scripts/split-catisen-projects.ps1` | Extracts both commits into two destination repositories, removes clutter, writes architecture files, and can apply the current patches. |
| `scripts/validate-split.ps1` | Confirms that the two repositories have different manifests and no forbidden artifacts. |
| `scripts/cleanup-repository-artifacts.ps1` | Standalone cleanup for an existing repository. Use it when you are not yet splitting. |

### Patches

| File | Use |
|---|---|
| `patches/0001-workflow-hardening.patch` | Fixes unsafe commit-message interpolation in the changelog workflow and adds CI format/check/test/clippy gates. |
| `patches/0003-current-browser-security.patch` | Applies source-level fixes to the supplied current wry source. |
| `patches/0002-policy-modules-reference.patch` | Reference-only policy patch. Do not apply during the normal migration; patch 0003 already includes the current policy module. |

### Analysis and guidance

| File | Use |
|---|---|
| `AUDIT_REPORT.md` | Full architectural, security, dead-code, clutter, testing, and rating report. |
| `SOURCE_REVIEW_ADDENDUM.md` | Exact findings confirmed after the current Rust source was supplied. |
| `SPLIT_PLAN.md` | Repository ownership, file division, commit selection, and commands. |
| `DOWNLOAD_GUIDE.md` | Short list of what to download and what not to copy. |
| `PATCHSET.md` | Patch application instructions and caveats. |
| `CLEANUP_MANIFEST.md` | Files and patterns that must not remain in either repository. |
| `README.md` | Contents overview. |

---

## Chapter 4 — What is deliberately not inside the ZIP

The ZIP does not contain:

- `target/` build output;
- `.cache/` cookies or profiles;
- `.catisen_history.json` browsing history;
- screenshots or pasted agent prompts;
- generated reports containing raw browsing URLs;
- the source-analysis snapshot directories;
- a fake replacement for the historical source.

Do not copy the analysis directories into either new repository. The historical source must come from Git commit `ea726eb`.

---

## Chapter 5 — Prerequisites

On the Windows machine where you will run the migration, install or verify:

```powershell
git --version
cargo --version
rustc --version
```

You also need:

- access to the original GitHub repository;
- permission to clone the source repository;
- two new empty GitHub repositories;
- PowerShell 5.1 or PowerShell 7;
- WebView2 Runtime and MSVC build tools for the Windows current-browser build;
- GTK/WebKitGTK development packages for the Linux current-browser build.

The analysis environment used to prepare this kit did not have Cargo installed. The patches were checked with `git apply --check`, but no Rust compilation result is claimed here.

---

## Chapter 6 — Create the two GitHub repositories

Create two empty repositories. Recommended names:

```text
catisen-legacy
catisen-browser
```

Do not initialize them with README, license, or `.gitignore` if possible. Empty repositories make the first push and history review cleaner. If they already contain a README, the migration script removes the destination working-tree files other than `.git` before copying the selected commit.

You will need three URLs:

```powershell
$SourceRepo  = 'https://github.com/YOUR_ACCOUNT/catisen.git'
$LegacyRepo  = 'https://github.com/YOUR_ACCOUNT/catisen-legacy.git'
$CurrentRepo = 'https://github.com/YOUR_ACCOUNT/catisen-browser.git'
```

Replace `YOUR_ACCOUNT` before running anything.

---

## Chapter 7 — Run the split

Run from the directory containing this kit:

```powershell
$SourceRepo  = 'https://github.com/YOUR_ACCOUNT/catisen.git'
$LegacyRepo  = 'https://github.com/YOUR_ACCOUNT/catisen-legacy.git'
$CurrentRepo = 'https://github.com/YOUR_ACCOUNT/catisen-browser.git'

powershell -ExecutionPolicy Bypass -File `
  .\scripts\split-catisen-projects.ps1 `
  -SourceRepo $SourceRepo `
  -LegacyRepo $LegacyRepo `
  -CurrentRepo $CurrentRepo `
  -ApplyCurrentPatches
```

The script:

1. clones the source repository into a temporary work directory;
2. fetches the source history;
3. checks `ea726eb` for the egui identity;
4. checks `2e03418` for the wry identity;
5. exports each commit independently;
6. prepares the two destination clones;
7. copies each commit into its matching destination;
8. removes generated output, runtime data, screenshots, pasted prompts, backups, and local agent files;
9. writes an `ARCHITECTURE.md` into each destination;
10. applies the workflow and current-browser security patches if `-ApplyCurrentPatches` was supplied.

The script does not force-push and does not automatically commit the results unless you add `-Commit`.

### Important rerun caveat

Do not rerun `-ApplyCurrentPatches` against a current repository that already has the patches. The second application will normally fail because the changes are already present. Start with fresh destination clones or omit `-ApplyCurrentPatches` after the first successful run.

---

## Chapter 8 — Review the resulting projects

The script prints the temporary work directory. Open the two repositories from that location.

Check the identities:

```powershell
powershell -ExecutionPolicy Bypass -File `
  .\scripts\validate-split.ps1 `
  -LegacyRoot 'C:\path\to\legacy-repo' `
  -CurrentRoot 'C:\path\to\current-repo'
```

The validator checks:

- legacy contains egui/eframe;
- legacy does not contain wry/tao;
- current contains wry/tao;
- neither contains `target`, `.cache`, `.continue`, `.agents`, history, screenshots, pasted prompts, or backup files;
- current `main.rs` activates the browser/security module.

Also inspect manually:

```powershell
git -C 'C:\path\to\legacy-repo' status --short
git -C 'C:\path\to\current-repo' status --short

git -C 'C:\path\to\legacy-repo' ls-files
git -C 'C:\path\to\current-repo' ls-files
```

---

## Chapter 9 — Historical repository: what it should become

Repository name:

```text
catisen-legacy
```

README identity:

```text
Privacy-focused HTTP client and static text browser.
```

It should retain the historical egui/network files from `ea726eb`. It should not receive the current `src/browser/*` files.

### Historical build goals

- source view works;
- reader/text view works;
- static HTML fetch works;
- ad-blocking behavior is tested honestly;
- Tor-aware HTTP requests are explicit;
- no external browser snapshot is advertised as a secure Tor renderer;
- history and cookies are outside the repository;
- no Servo dependency is compiled unless it is genuinely imported and tested.

### Historical caveat

The historical source was not attached directly. It is extracted from Git by the script. After extraction, inspect `Cargo.toml` and run the build. If the old manifest contains an unused Servo dependency, remove it or put it behind an explicitly tested feature before calling the project stable.

---

## Chapter 10 — Current browser repository: what it should become

Repository name:

```text
catisen-browser
```

README identity:

```text
Embedded WebView browser using wry and tao.
```

### Current security patch fixes

The current patch addresses:

- strict `http`/`https` navigation;
- credential-bearing URL rejection;
- Tor requirement for `.onion` URLs;
- safe download path confinement;
- libcurl proxy fail-closed behavior;
- redirect limit;
- zero-byte download failure detection;
- configuration proxy selection;
- Tor-routed exit-country lookup;
- HTTPS permission-key mismatch;
- history moved outside the repository;
- atomic and bounded history writes;
- stronger sync identity entropy;
- removal of secret logging;
- explicit unimplemented pairing result;
- isolated-profile cleanup;
- top-frame-only privacy/YouTube hooks;
- runtime download proxy updates when Tor is toggled.

### Remaining current-browser limitations

These are not silently claimed to be fixed:

1. Windows WebView2 proxy routing still needs a supported WebView2 configuration path. Environment variables alone are not enough.
2. Tab isolation creates and cleans directories but does not yet create independent WebView cookie/storage contexts.
3. `sandbox.rs` is still a placeholder, not a real process sandbox.
4. WebView page JavaScript and injected toolbar JavaScript share a context. Privileged IPC should eventually move to a native UI channel or authenticated capability mechanism.
5. Current integration tests expect command-line and telemetry behavior that `main.rs` does not implement. Rewrite those tests for the current runtime.
6. Device sync transport is not implemented. Do not advertise device pairing as complete.

---

## Chapter 11 — Build and test the historical repository

```powershell
Set-Location 'C:\path\to\catisen-legacy'

cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Do not add current wry files to make the historical project compile. If the historical code does not build, repair that project on its own terms.

---

## Chapter 12 — Build and test the current browser on Windows

```powershell
Set-Location 'C:\path\to\catisen-browser'

cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release
```

Verify that WebView2 Runtime and the Microsoft C++ toolchain are installed.

Do not describe Windows browsing as Tor-routed until WebView2 proxy routing is implemented and tested. The current source explicitly documents that environment variables do not control WebView2 networking.

---

## Chapter 13 — Build and test the current browser on Linux

Install the platform packages required by the selected wry backend, then run:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release
```

The GitHub Actions workflow patch installs GTK/WebKitGTK development packages for Ubuntu. Adjust package names if using another distribution.

---

## Chapter 14 — Pure cross-platform architecture

A single WebView binary with zero OS dependencies is not realistic. WebKitGTK and WebView2 are native platform engines.

The correct cross-platform design is:

```text
catisen-core       pure Rust policy/network/history/download crate
catisen-browser    wry/tao native browser shell
linux adapter      WebKitGTK dependencies
windows adapter    WebView2 dependencies
```

Move into `catisen-core`:

- URL policy;
- proxy/Tor policy;
- reqwest/rustls networking;
- ad-blocking policy;
- history model;
- download policy;
- configuration;
- redacted diagnostics.

Keep out of `catisen-core`:

- GTK;
- WebKitGTK;
- WebView2;
- `winapi`;
- mpv/vlc process launching;
- Servo until genuinely integrated;
- external browser shell-outs.

Use a CI matrix with at least:

```text
ubuntu-latest
windows-latest
```

The source can be shared. The native build dependencies cannot.

---

## Chapter 15 — Commit and push

After review and successful builds:

```powershell
git -C 'C:\path\to\catisen-legacy' add -A
git -C 'C:\path\to\catisen-legacy' commit -m 'chore: split legacy privacy HTTP client'
git -C 'C:\path\to\catisen-legacy' push -u origin main

git -C 'C:\path\to\catisen-browser' add -A
git -C 'C:\path\to\catisen-browser' commit -m 'chore: split current wry browser'
git -C 'C:\path\to\catisen-browser' push -u origin main
```

Before pushing, verify:

```powershell
git -C 'C:\path\to\catisen-legacy' ls-files | Select-String '(^target/|^\.cache/|\.catisen_history|Screenshot_|Pasted-|\.bak-)'
git -C 'C:\path\to\catisen-browser' ls-files | Select-String '(^target/|^\.cache/|\.catisen_history|Screenshot_|Pasted-|\.bak-)'
```

Both commands must return no lines.

---

## Chapter 16 — Security and rollback caveats

### Sensitive history

If `.catisen_history.json`, `.cache`, cookies, or account URLs were ever pushed, deleting them from the new repositories is not enough. Rotate exposed tokens and rewrite the old repository history with `git filter-repo` before archiving or deleting it.

### Force push

Do not use `git push --force` on a shared repository without agreement. Use `--force-with-lease` only after verifying the remote and coordinating with collaborators.

### Rollback

If the split output is wrong:

1. delete the two destination working directories;
2. do not push them;
3. rerun the script with the same commit checks;
4. inspect the generated `ARCHITECTURE.md` files;
5. apply patches only once.

The original source repository is not modified by the split script. The script works through a temporary clone and destination clones.

---

## Final expected result

You should end with:

```text
catisen-legacy/
  one clear historical HTTP/text-client identity

catisen-browser/
  one clear current wry/tao browser identity
```

The legacy project is the more tool-like and portable privacy client. The current project is the browser product aligned with the original Catisen browser goal. Neither project should claim capabilities that its own tests do not demonstrate.
