#!/usr/bin/env bash
# Catisen Browser manual/integration test runner.
#
# Run from Git Bash on Windows or a normal Bash shell:
#   bash scripts/test-browser.sh
#
# To launch the GUI and record observations interactively:
#   RUN_BROWSER=1 bash scripts/test-browser.sh
#
# To use a custom output path:
#   bash scripts/test-browser.sh catisen-browser-test.txt

set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
STAMP="$(date +%Y%m%d-%H%M%S)"
LOG="${1:-$ROOT/catisen-browser-test-$STAMP.txt}"
mkdir -p "$(dirname "$LOG")"

# UTF-8/LF source files are intentional. This avoids the $'\377\376[[' error
# that occurs when a UTF-16 PowerShell file is pasted into Git Bash.
exec > >(tee -a "$LOG") 2>&1

PASS_COUNT=0
FAIL_COUNT=0
NOTE_COUNT=0
BROWSER_PID=""
FIXTURE_DIR=""
FIXTURE_PORT=""
SERVER_PID=""
PYTHON_BIN=""
if command -v python3 >/dev/null 2>&1; then
    PYTHON_BIN="$(command -v python3)"
elif command -v python >/dev/null 2>&1; then
    PYTHON_BIN="$(command -v python)"
fi

section() {
    printf '\n============================================================\n'
    printf '%s\n' "$1"
    printf '============================================================\n'
}

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

run_cmd() {
    printf '\n$'
    printf ' %q' "$@"
    printf '\n'
    "$@"
    local rc=$?
    printf '[exit=%s]\n' "$rc"
    return "$rc"
}

cleanup() {
    if [[ -n "$BROWSER_PID" ]] && kill -0 "$BROWSER_PID" 2>/dev/null; then
        kill "$BROWSER_PID" 2>/dev/null || true
        sleep 1
        kill -9 "$BROWSER_PID" 2>/dev/null || true
    fi
    if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
        kill "$SERVER_PID" 2>/dev/null || true
    fi
    if [[ -n "$FIXTURE_DIR" && -d "$FIXTURE_DIR" ]]; then
        rm -rf "$FIXTURE_DIR"
    fi
}
trap cleanup EXIT INT TERM

cd "$ROOT" || exit 1

section "Catisen Browser test run"
printf 'Started: %s\n' "$(date -u '+%Y-%m-%d %H:%M:%S UTC')"
printf 'Root: %s\n' "$ROOT"
printf 'Log: %s\n' "$LOG"
printf 'Branch: '; git branch --show-current 2>/dev/null || true
printf 'Commit: '; git rev-parse --short HEAD 2>/dev/null || true
printf 'Remote: '; git remote get-url origin 2>/dev/null || true

CURRENT_BRANCH="$(git branch --show-current 2>/dev/null || true)"
if [[ "$CURRENT_BRANCH" == "arena/019fbfaf-catisen-browser" ]]; then
    record PASS "Running on arena/019fbfaf-catisen-browser"
elif git cat-file -e HEAD:scripts/test-browser.sh 2>/dev/null \
    && git cat-file -e HEAD:src/sandbox.rs 2>/dev/null; then
    record PASS "Running from a detached browser-source worktree at $(git rev-parse --short HEAD)"
else
    record NOTE "This is not the fixed browser source worktree. Fetch/check out arena/019fbfaf-catisen-browser before treating results as the remediation build."
fi

section "Environment and repository checks"
for command_name in git cargo rustc python3 python node curl; do
    if command -v "$command_name" >/dev/null 2>&1; then
        printf '%-8s: ' "$command_name"
        "$command_name" --version 2>&1 | head -n 1 || true
    else
        record NOTE "$command_name is not installed"
    fi
done

if [[ -f Cargo.lock ]]; then
    record PASS "Cargo.lock exists"
else
    record NOTE "Cargo.lock is absent; generate and commit it on a machine with Cargo"
fi

if [[ -f .catisen_history.json ]]; then
    record FAIL ".catisen_history.json exists in the repository root; do not commit or paste it"
else
    record PASS "No repository-root .catisen_history.json"
fi

for forbidden in .cache attached_assets; do
    if [[ -e "$forbidden" ]]; then
        record FAIL "Runtime/generated directory exists: $forbidden"
    else
        record PASS "No $forbidden directory"
    fi
done
if [[ -e target ]]; then
    record NOTE "target/ exists because Cargo generated build artifacts during this test; it is ignored and must not be committed"
else
    record PASS "No target/ directory before Cargo runs"
fi

section "Static/source assertions"
assert_source() {
    local needle="$1"
    local file="$2"
    if grep -Fq "$needle" "$file"; then
        record PASS "$file contains: $needle"
    else
        record FAIL "$file is missing: $needle"
    fi
}
assert_source 'serde_json::to_string(ua)' src/browser/chrome.rs
assert_source 'const _origSend = XMLHttpRequest.prototype.send;' src/browser/chrome.rs
assert_source 'remove_dir_all' src/tab_isolation.rs
assert_source 'PR_SET_NO_NEW_PRIVS' src/sandbox.rs
assert_source 'CreateJobObjectW' src/sandbox.rs
assert_source 'set_global_default' src/permissions.rs
assert_source 'should_block_resource' src/ublock_integration.rs
assert_source 'config.use_tor_by_default = enabled' src/browser/mod.rs
assert_source 'CATISEN_LOG_FILE' src/main.rs
if grep -Fq 'pub fn unwrap_or_default' src/config.rs; then
    record FAIL 'Dead CatisenConfig::unwrap_or_default method still exists'
else
    record PASS 'Dead CatisenConfig::unwrap_or_default method removed'
fi

section "Cargo checks (run when Cargo is installed)"
if command -v cargo >/dev/null 2>&1; then
    run_cmd cargo fmt --all -- --check || record FAIL 'cargo fmt check failed'
    run_cmd cargo check --workspace --all-targets || record FAIL 'cargo check failed'
    run_cmd cargo test --workspace --all-targets -- --nocapture || record FAIL 'cargo test failed'
    run_cmd cargo clippy --workspace --all-targets -- -D warnings || record FAIL 'cargo clippy failed'
    run_cmd cargo run -- --headless --no-network || record FAIL 'headless no-network smoke failed'
else
    record NOTE 'Cargo checks skipped because cargo is unavailable'
fi

section "External network smoke checks"
check_url() {
    local label="$1"
    local url="$2"
    if ! command -v curl >/dev/null 2>&1; then
        record NOTE "curl unavailable; skipped $label ($url)"
        return
    fi
    local result
    result="$(curl -L --max-time 20 -sS -o /dev/null -w 'status=%{http_code} redirects=%{num_redirects} bytes=%{size_download} final=%{url_effective}' "$url" 2>&1)"
    local rc=$?
    if [[ $rc -eq 0 ]]; then
        record PASS "$label: $result"
    else
        record NOTE "$label unavailable (network/site issue): $result"
    fi
}
check_url 'HTTPS baseline' 'https://example.com/'
check_url 'IANA links/content' 'https://www.iana.org/domains/reserved'
check_url 'Redirect handling' 'https://httpbin.org/redirect/1'
check_url 'Non-2xx status handling' 'https://httpbin.org/status/404'
check_url 'HTTP warning target' 'http://neverssl.com/'
check_url 'Wikipedia/SPA-scale page' 'https://www.wikipedia.org/'

if [[ "${TEST_TOR:-0}" == "1" ]]; then
    section "Optional Tor route check"
    TOR_PROXY="${CATISEN_TOR_PROXY:-socks5h://127.0.0.1:9150}"
    printf 'Tor proxy configured for this test: %s\n' "$TOR_PROXY"
    check_url 'Tor check page' 'https://check.torproject.org/api/ip'
    if command -v curl >/dev/null 2>&1; then
        curl --proxy "$TOR_PROXY" --max-time 30 -sS 'https://check.torproject.org/api/ip' -o "$ROOT/tor-check-response.json"
        rc=$?
        if [[ $rc -eq 0 ]]; then
            record PASS "Tor-proxied check completed; inspect tor-check-response.json"
        else
            record NOTE "Tor-proxied check failed; no clearnet fallback was attempted"
        fi
    fi
else
    record NOTE 'Tor checks skipped. Run TEST_TOR=1 only with a local Tor daemon and a deliberate privacy test.'
fi

section "Local browser fixture"
if [[ -n "$PYTHON_BIN" ]]; then
    FIXTURE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/catisen-browser-fixture.XXXXXX")"
    FIXTURE_PORT="$("$PYTHON_BIN" - <<'PY'
import socket
sock = socket.socket()
sock.bind(('127.0.0.1', 0))
print(sock.getsockname()[1])
sock.close()
PY
)"
    cat > "$FIXTURE_DIR/reader.html" <<'HTML'
<!doctype html><meta charset="utf-8"><title>Catisen Reader Fixture</title>
<h1 id="heading">Reader mode fixture</h1>
<p>This paragraph should remain visible when Reader Mode is enabled.</p>
<script>document.body.dataset.pageScript = 'loaded';</script>
<aside id="fixture-sidebar">Sidebar should be removed by Reader Mode.</aside>
<iframe id="fixture-iframe" src="https://doubleclick.net/ads/test"></iframe>
<video id="fixture-video" controls></video>
<a href="/download.bin" download="catisen-test.bin">Download fixture</a>
HTML
    cat > "$FIXTURE_DIR/permissions.html" <<'HTML'
<!doctype html><meta charset="utf-8"><title>Catisen Permission Fixture</title>
<h1>Permission fixture</h1><pre id="result">checking...</pre>
<script>
(async function () {
  const out = document.getElementById('result');
  const results = [];
  try { navigator.geolocation.getCurrentPosition(() => results.push('geo-granted'), e => results.push('geo-denied:' + e.code)); }
  catch (e) { results.push('geo-threw:' + e.name); }
  try {
    if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
      await navigator.mediaDevices.getUserMedia({audio:true, video:true});
      results.push('media-granted');
    } else results.push('media-api-absent');
  } catch (e) { results.push('media-denied:' + e.name); }
  out.textContent = results.join('\n');
})();
</script>
HTML
    cat > "$FIXTURE_DIR/ipc.html" <<'HTML'
<!doctype html><meta charset="utf-8"><title>Catisen IPC Boundary Probe</title>
<h1>IPC boundary probe</h1><pre id="result">checking...</pre>
<script>
try {
  if (window.ipc && window.ipc.postMessage) {
    window.ipc.postMessage(JSON.stringify({t:'set_tor', enabled:true}));
    document.getElementById('result').textContent = 'PAGE COULD CALL window.ipc; inspect settings/logs (known remaining blocker)';
  } else document.getElementById('result').textContent = 'window.ipc unavailable';
} catch (e) { document.getElementById('result').textContent = 'IPC call threw: ' + e; }
</script>
HTML
    "$PYTHON_BIN" - "$FIXTURE_DIR" "$FIXTURE_PORT" >"$FIXTURE_DIR/server.log" 2>&1 <<'PY' &
import http.server
import os
import sys
root = sys.argv[1]
port = int(sys.argv[2])
os.chdir(root)
server = http.server.ThreadingHTTPServer(('127.0.0.1', port), http.server.SimpleHTTPRequestHandler)
server.serve_forever()
PY
    SERVER_PID=$!
    "$PYTHON_BIN" - "$FIXTURE_DIR/download.bin" <<'PY'
import sys
from pathlib import Path
Path(sys.argv[1]).write_bytes(bytes(range(256)) * 32)
PY
    sleep 1
    if kill -0 "$SERVER_PID" 2>/dev/null; then
        record PASS "Local fixture server: http://127.0.0.1:$FIXTURE_PORT/reader.html"
        printf 'Fixture URLs for manual browser testing:\n'
        printf '  reader      http://127.0.0.1:%s/reader.html\n' "$FIXTURE_PORT"
        printf '  permissions http://127.0.0.1:%s/permissions.html\n' "$FIXTURE_PORT"
        printf '  IPC probe   http://127.0.0.1:%s/ipc.html\n' "$FIXTURE_PORT"
        printf '  download    http://127.0.0.1:%s/download.bin\n' "$FIXTURE_PORT"
    else
        record NOTE 'Could not start local Python fixture server'
    fi
else
    record NOTE 'Python 3 is unavailable; local fixture pages were skipped'
fi

if [[ "${RUN_BROWSER:-0}" == "1" ]]; then
    section "Launching GUI browser"
    if command -v cargo >/dev/null 2>&1; then
        RUNTIME_LOG="$ROOT/catisen-browser-runtime-$STAMP.txt"
        printf 'Browser runtime log: %s\n' "$RUNTIME_LOG"
        cargo run >"$RUNTIME_LOG" 2>&1 &
        BROWSER_PID=$!
        sleep 8
        if kill -0 "$BROWSER_PID" 2>/dev/null; then
            record PASS "Browser process started (pid $BROWSER_PID)"
        else
            record FAIL "Browser process exited during startup; inspect $RUNTIME_LOG"
            BROWSER_PID=""
        fi
    else
        record FAIL 'RUN_BROWSER=1 requested but cargo is unavailable'
    fi
fi

section "Manual WebView2/browser test matrix"
printf '%s\n' 'The following require the GUI browser. Set RUN_BROWSER=1 to launch it and record PASS/FAIL/NOTE answers.'
printf '%s\n' 'If RUN_BROWSER is not set, this section is a checklist to execute manually.'

manual_step() {
    local id="$1"
    local title="$2"
    local url="$3"
    local expected="$4"
    printf '\n[%s] %s\nURL: %s\nExpected: %s\n' "$id" "$title" "$url" "$expected"
    if [[ "${RUN_BROWSER:-0}" == "1" && -t 0 ]]; then
        read -r -p 'Observation (PASS/FAIL/NOTE + short detail): ' answer
        case "${answer^^}" in
            PASS*) record PASS "$id $answer" ;;
            FAIL*) record FAIL "$id $answer" ;;
            *) record NOTE "$id $answer" ;;
        esac
    fi
}

manual_step 'WEB-01' 'HTTPS baseline and URL/title sync' 'https://example.com/' 'Page renders; URL bar and window title match; no duplicate toolbar.'
manual_step 'WEB-02' 'Rich content and links' 'https://www.iana.org/domains/reserved' 'Links work; back/forward/reload work; toolbar remains single.'
manual_step 'WEB-03' 'HTTP warning and permission denial' 'http://neverssl.com/' 'Warning padlock; geolocation/media APIs are denied; no native grant appears.'
manual_step 'WEB-04' 'Redirect and HTTP status behavior' 'https://httpbin.org/redirect/1' 'Final URL is visible; no crash. Use /status/404 to confirm browser remains usable.'
manual_step 'WEB-05' 'Wikipedia/SPA behavior' 'https://www.wikipedia.org/' 'Navigation and URL-bar updates remain usable on a larger page.'
manual_step 'WEB-06' 'YouTube and iframe/ad behavior' 'https://www.youtube.com/' 'Page remains usable; ads may not all be blocked; no duplicate toolbars in iframes.'
if [[ -n "$FIXTURE_PORT" ]]; then
    manual_step 'WEB-07' 'Reader mode DOM cleanup' "http://127.0.0.1:$FIXTURE_PORT/reader.html" 'Toggle Reader Mode: paragraph remains; aside/iframe/video disappear; no JavaScript syntax error.'
    manual_step 'WEB-08' 'HTTP hardware APIs' "http://127.0.0.1:$FIXTURE_PORT/permissions.html" 'Result shows geo-denied and media-denied, not granted.'
    manual_step 'WEB-09' 'Download fixture' "http://127.0.0.1:$FIXTURE_PORT/download.bin" 'Use the supported download UI/path; verify a non-zero file and no path outside downloads.'
    manual_step 'WEB-10' 'Untrusted-page IPC probe' "http://127.0.0.1:$FIXTURE_PORT/ipc.html" 'This is an intentionally expected remaining blocker: record whether the page can call window.ipc and whether Tor/settings change.'
fi
manual_step 'WEB-11' 'URL scheme policy' 'javascript:alert(1)' 'Must not execute JavaScript; it should be searched/blocked.'
manual_step 'WEB-12' 'Local-file policy' 'file:///etc/passwd' 'Must not load a local file.'
manual_step 'WEB-13' 'Credential URL policy' 'https://user:password@example.com/' 'Must be rejected, not loaded.'
manual_step 'WEB-14' 'Onion without Tor' 'http://example.onion/' 'Must be blocked when no Tor route is active.'
manual_step 'WEB-15' 'Settings/history' 'Open Settings with Ctrl+, or the settings button' 'Toggle persistent history off; close/reopen; no visits are written. Toggle Tor; verify preference persists and restart is required for WebView route changes.'
manual_step 'WEB-16' 'Sync identity' 'Open Settings -> Generate New Sync Identity' 'Seed/QR appears in the separate native window; main page cannot query its DOM; pairing is clearly not implemented.'
manual_step 'WEB-17' 'Tabs/cleanup' 'Ctrl+T / New Tab' 'New-tab UI works, but verify and record that real cookie isolation is not yet available.'

if [[ "${RUN_BROWSER:-0}" == "1" && -t 0 && -n "$BROWSER_PID" ]]; then
    read -r -p 'Manual matrix complete. Press Enter to close the browser and finish the log: ' _
fi

section "Summary"
printf 'PASS=%s FAIL=%s NOTE=%s\n' "$PASS_COUNT" "$FAIL_COUNT" "$NOTE_COUNT"
printf 'Full log: %s\n' "$LOG"
printf '%s\n' 'Paste the full log into the review conversation. Redact cookies, tokens, authentication URLs, and personal paths before sharing.'
