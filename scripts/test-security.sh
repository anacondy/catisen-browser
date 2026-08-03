#!/usr/bin/env bash
# Catisen Browser security-focused verification runner.
#
# This complements scripts/test-browser.sh. It separates deterministic Rust
# policy tests and source assertions from manual WebView2 checks that cannot be
# proven by a headless process.
#
# Usage from Git Bash:
#   bash scripts/test-security.sh catisen-security-test.txt
#
# Optional Tor checks (only when Tor is deliberately running):
#   TEST_TOR=1 bash scripts/test-security.sh catisen-security-tor.txt

set -u
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
STAMP="$(date +%Y%m%d-%H%M%S)"
LOG="${1:-$ROOT/catisen-security-test-$STAMP.txt}"
mkdir -p "$(dirname "$LOG")"
exec > >(tee -a "$LOG") 2>&1

PASS_COUNT=0
FAIL_COUNT=0
NOTE_COUNT=0

record() {
    local kind="$1"
    shift
    printf '[%s] %s\n' "$kind" "$*"
    case "$kind" in
        PASS) PASS_COUNT=$((PASS_COUNT + 1)) ;;
        FAIL) FAIL_COUNT=$((FAIL_COUNT + 1)) ;;
        NOTE) NOTE_COUNT=$((NOTE_COUNT + 1)) ;;
    esac
}

section() {
    printf '\n============================================================\n%s\n============================================================\n' "$1"
}

run_check() {
    printf '\n$'
    printf ' %q' "$@"
    printf '\n'
    "$@"
    local rc=$?
    printf '[exit=%s]\n' "$rc"
    return "$rc"
}

cd "$ROOT" || exit 1

section "Security test run"
printf 'Started: %s\n' "$(date -u '+%Y-%m-%d %H:%M:%S UTC')"
printf 'Commit: '; git rev-parse --short HEAD 2>/dev/null || true
printf 'Branch: '; git branch --show-current 2>/dev/null || printf '(detached)\n'
printf 'Log: %s\n' "$LOG"

if git cat-file -e HEAD:src/browser/mod.rs 2>/dev/null \
    && git cat-file -e HEAD:src/security_policy.rs 2>/dev/null; then
    record PASS 'Browser source is present in this worktree'
else
    record FAIL 'Browser source is missing; do not run this as the remediation build'
fi

section "Repository privacy checks"
if [[ -f .catisen_history.json ]]; then
    record FAIL 'Repository-root history file exists; do not commit/share it'
else
    record PASS 'No repository-root history file'
fi
for path in attached_assets .cache; do
    if [[ -e "$path" ]]; then
        record FAIL "Sensitive/runtime path exists: $path"
    else
        record PASS "No $path directory"
    fi
done
if [[ -e target ]]; then
    record NOTE 'target/ exists as generated Cargo output; it must remain ignored and uncommitted'
fi

section "Static security assertions"
assert_present() {
    local needle="$1"
    local file="$2"
    if grep -Fq "$needle" "$file"; then
        record PASS "$file: $needle"
    else
        record FAIL "$file is missing: $needle"
    fi
}
assert_present 'serde_json::to_string(ua)' src/browser/chrome.rs
assert_present 'const _origSend = XMLHttpRequest.prototype.send;' src/browser/chrome.rs
assert_present 'set_global_default' src/permissions.rs
assert_present 'is_insecure_http_origin' src/permissions.rs
assert_present 'remove_dir_all' src/tab_isolation.rs
assert_present 'PR_SET_NO_NEW_PRIVS' src/sandbox.rs
assert_present 'CreateJobObjectW' src/sandbox.rs
assert_present 'Some(&proxy_addr)' src/privacy/tor_manager.rs
assert_present 'max_attempts' src/network/fetch.rs
assert_present 'should_block_resource' src/ublock_integration.rs
assert_present 'sanitize_download_path' src/security_policy.rs
assert_present 'CATISEN_LOG_FILE' src/main.rs
if grep -Fq 'pub fn unwrap_or_default' src/config.rs; then
    record FAIL 'Dead CatisenConfig::unwrap_or_default remains'
else
    record PASS 'Dead CatisenConfig::unwrap_or_default is absent'
fi
if [[ -e src/network/text_extract.rs ]]; then
    record FAIL 'Inactive network/text_extract.rs remains in the product tree'
else
    record PASS 'Inactive text-extraction module is removed'
fi

section "Deterministic Rust security tests"
if command -v cargo >/dev/null 2>&1; then
    run_check cargo test --workspace --all-targets -- security_policy --nocapture \
        || record FAIL 'security_policy tests failed'
    run_check cargo test --workspace --all-targets -- permissions --nocapture \
        || record FAIL 'permission tests failed'
    run_check cargo test --workspace --all-targets -- history --nocapture \
        || record FAIL 'history tests failed'
    run_check cargo test --workspace --all-targets -- sync_chain --nocapture \
        || record FAIL 'sync identity tests failed'
    run_check cargo test --workspace --all-targets -- tab_isolation --nocapture \
        || record FAIL 'tab cleanup tests failed'
    run_check cargo test --workspace --all-targets -- ublock_integration --nocapture \
        || record FAIL 'adblock tests failed'
else
    record NOTE 'Cargo is unavailable; deterministic Rust tests were skipped'
fi

section "Build and lint status"
if command -v cargo >/dev/null 2>&1; then
    run_check cargo check --workspace --all-targets \
        || record FAIL 'cargo check failed'
    run_check cargo clippy --workspace --all-targets -- -D warnings \
        || record FAIL 'clippy -D warnings failed'
    run_check cargo fmt --all -- --check \
        || record FAIL 'cargo fmt check failed'
else
    record NOTE 'Cargo is unavailable; build/lint checks were skipped'
fi

section "Headless fail-closed check"
if command -v cargo >/dev/null 2>&1; then
    run_check cargo run -- --headless --no-network \
        || record FAIL 'headless no-network check failed'
else
    record NOTE 'Cargo is unavailable; headless check was skipped'
fi

section "Optional Tor route check"
if [[ "${TEST_TOR:-0}" == "1" ]]; then
    if command -v curl >/dev/null 2>&1; then
        TOR_PROXY="${CATISEN_TOR_PROXY:-socks5h://127.0.0.1:9150}"
        printf 'Proxy: %s\n' "$TOR_PROXY"
        if curl --proxy "$TOR_PROXY" --max-time 30 -fsS \
            'https://check.torproject.org/api/ip' \
            -o "$ROOT/tor-security-check.json"; then
            record PASS 'Tor-proxied check completed; inspect tor-security-check.json'
        else
            record FAIL 'Tor was requested but the proxy check failed; no clearnet fallback was attempted'
        fi
    else
        record NOTE 'curl is unavailable; Tor check skipped'
    fi
else
    record NOTE 'Tor check skipped; use TEST_TOR=1 only with a deliberate Tor test'
fi

section "Manual WebView2 security matrix"
printf '%s\n' 'Run these manually in the GUI browser and record the observed result in the final log.'
printf '%s\n' 'The browser should be closed before sharing runtime logs.'
cat <<'MATRIX'
SEC-01  Navigate to javascript:alert(1)
        Expected: no JavaScript executes; it becomes a search or is rejected.
SEC-02  Navigate to file:///C:/Windows/win.ini
        Expected: local file is not loaded.
SEC-03  Navigate to https://user:password@example.com/
        Expected: credential-bearing URL is rejected.
SEC-04  Navigate to http://example.onion/ without Tor
        Expected: navigation is rejected; no clearnet request is attempted.
SEC-05  Open http://neverssl.com/
        Expected: HTTP warning is visible and insecure-origin hardware APIs are denied.
SEC-06  Open the local permissions fixture from test-browser.sh
        Expected: geolocation and media calls are denied, not granted.
SEC-07  Open the local IPC probe from test-browser.sh
        Expected/current limitation: ordinary page JavaScript may still see the shared IPC bridge.
        Record whether it can change Tor/settings; this is a known remaining architecture blocker.
SEC-08  Turn Persistent Visit History off, visit a page, close/reopen
        Expected: the visit is not persisted; onion visits are never persisted.
SEC-09  Turn Tor on with no reachable proxy
        Expected: required Tor paths fail; they do not silently use clearnet.
SEC-10  Open Sync Identity
        Expected: seed/QR appears in a separate native window; pairing remains explicitly unimplemented.
SEC-11  Press F11/Fn+F11, resize, then press Escape
        Expected: decorations hide/show; resizing is disabled only while borderless and returns afterward.
SEC-12  Play portrait/4:3 and 16:9 videos
        Expected: site/player controls remain visible; Catisen does not force player height.
MATRIX

section "Remaining architecture blockers"
cat <<'REMAINING'
- Page JavaScript and browser chrome still share a WebView IPC context.
- Native WebView2 permission callbacks are not fully integrated.
- Tab records do not yet create separate WebView cookie/storage contexts.
- EasyList coverage is best effort until WebView2 resource interception supplies initiator/type data.
- PR_SET_NO_NEW_PRIVS/Job Object hardening is not a full renderer sandbox.
- Sync pairing transport and key exchange are not implemented.
REMAINING

section "Summary"
printf 'PASS=%s FAIL=%s NOTE=%s\n' "$PASS_COUNT" "$FAIL_COUNT" "$NOTE_COUNT"
printf 'Full log: %s\n' "$LOG"
printf '%s\n' 'Redact cookies, authentication URLs, tokens, and personal paths before sharing.'
