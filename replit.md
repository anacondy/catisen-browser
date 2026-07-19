# Catisen — Privacy Browser

A privacy-focused browser written in Rust.

## Architecture

| Layer | Technology | Notes |
|-------|-----------|-------|
| **Rendering** | wry 0.38 + WebKitGTK (Linux) | Full DOM / BOM / JS / CSS — real engine, not a scraper |
| **Window / event loop** | tao 0.26 | Cross-platform OS window + typed event loop |
| **Browser chrome** | JavaScript (injected) | Fixed toolbar: URL bar, Back/Fwd/Reload, HTTPS lock icon |
| **Privacy JS** | Rust-generated init script | BOM overrides run before any page script |
| **Ad blocking** | adblock crate (uBlock Origin engine) | Bundled EasyList + navigation handler + JS fetch/XHR intercept |
| **Networking** | reqwest + rustls | TLS via rustls (no OpenSSL); Tor via socks5h env vars |
| **Config** | TOML (`config.toml`) | Hot-reloadable; Tor, fingerprinting, home page, etc. |

## Key source files

```
src/
├── main.rs                      — binary entry point
├── browser/
│   ├── mod.rs                   — wry event loop, WebView, IPC routing
│   ├── chrome.rs                — toolbar JS + privacy/BOM init script
│   └── ipc.rs                   — IPC message types (JS → Rust)
├── config.rs                    — CatisenConfig struct + TOML loader
├── privacy/
│   ├── stealth.rs               — user-agent selection, fingerprint options
│   └── tor_manager.rs           — Tor connectivity checks
├── network/                     — HTTP fetch, cookie jars, redirect follow
├── ublock_integration.rs        — adblock engine wrapper, bundled EasyList
├── tab_isolation.rs             — per-tab cookie jar isolation
├── history.rs                   — bookmark & history (flat-file JSON)
└── sync_chain.rs                — QR-code sync
assets/
└── easylist.txt                 — bundled filter list (embedded at compile time)
config.toml                      — user configuration
```

## How privacy works

1. **Before any page script runs**: Rust generates a JS init script from `privacy::stealth` (navigator UA, platform, hardwareConcurrency, canvas noise, WebGL vendor spoof, webdriver removal) and registers it with wry's `with_initialization_script`. It runs at document-start, before the page's own JS.

2. **Navigation-level blocking**: wry's `with_navigation_handler` is called for every load; blocked URLs return `false` (WebKitGTK never fetches them).

3. **Subresource blocking (JS layer)**: The init script wraps `window.fetch` and `XMLHttpRequest.prototype.open` to block calls to known ad domains at the JS level.

4. **Tor proxy**: When `use_tor_by_default = true` in `config.toml`, Catisen sets `http_proxy` / `https_proxy` env vars to the configured SOCKS5 address before creating the WebView. WebKitGTK on Linux respects these automatically.

## Running

```
cargo run
```

Requires `webkit2gtk-4.1` (installed as a Nix system dependency).

## Config reference (`config.toml`)

| Key | Default | Description |
|-----|---------|-------------|
| `home_page` | `https://duckduckgo.com` | Page to open on startup |
| `use_tor_by_default` | `false` | Route all WebKit traffic through Tor |
| `tor_proxy_url` | `socks5h://127.0.0.1:9150` | Tor SOCKS5 address |
| `fingerprint_level` | `strict` | How aggressively to spoof BOM properties |
| `ublock_rules_path` | `assets/easylist.txt` | Extra filter list (merged with bundled list) |

## User preferences

- DuckDuckGo as the default home page (privacy-first)
- Bundled EasyList embedded at compile time — ad blocker works out of the box with no download
- wry + WebKitGTK preferred over egui + external Chrome shell-out (SEC-01 fix)
- No dependency on Servo git repo (removed — unconditional dep blocked compilation)
