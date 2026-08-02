//! Core wry browser — real DOM/BOM/JS rendering via WebKitGTK (Linux).
//!
//! # What was dead, and what is now wired
//!
//! This file is the single call site that activates the following previously-dead modules:
//!
//! | Module                           | What was dead             | Now wired here                          |
//! |----------------------------------|---------------------------|-----------------------------------------|
//! | `privacy/tor_manager.rs`         | All functions             | `resolve_tor_for_session()` calls the configured-proxy detector; `maybe_probe_tor_route()` runs on a background thread |
//! | `libcurl_download_manager.rs`    | `tor_proxy` field missing | `start_download()` now receives `active_tor_proxy` so downloads route through Tor |
//! | `permissions.rs`                 | `PermissionManager` never constructed | Initialised here; `set_permission()` called on HTTP sites in `UpdateUrlBar`; `SetPermission` IPC updates it |
//! | `text_mode.rs`                   | `ReaderMode` never constructed | Initialised here; `ToggleReaderMode` IPC calls `toggle()` and injects reader CSS |
//! | `sync_chain.rs`                  | `SyncChain` never constructed | Initialised here; `ShowSyncChain` IPC calls `generate_new_identity()` + `generate_visual_qr_matrix()` and shows a QR modal |
//!
//! `tor_proxy.rs` has been marked deprecated (see that file); `privacy/tor_manager.rs` is the canonical Tor module.
//!
//! # Tor / WebView proxy note
//!
//! On **Linux with WebKitGTK**, setting `http_proxy` / `https_proxy` environment variables
//! *before* the WebView is constructed causes WebKit's network stack to respect them — so
//! the env-var approach used here is correct and effective.
//!
//! On **Windows with WebView2**, env vars do NOT affect the WebView2 network stack.
//! WebView2 exposes proxy settings via `CoreWebView2EnvironmentOptions::put_AdditionalBrowserArguments`
//! (passing `--proxy-server=socks5://127.0.0.1:9150`) or via the ICoreWebView2Profile proxy API
//! added in WebView2 SDK 1.0.1661+.  wry 0.38 does not expose a Rust wrapper for these APIs.
//! Until wry adds a `with_proxy()` builder method, Windows users who want WebView Tor routing
//! should configure a system-wide proxy or use a newer wry version.  Downloads via libcurl
//! already support Tor on all platforms (see `libcurl_download_manager.rs`).

pub mod chrome;
pub mod debug_panel;
pub mod ipc;

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::{DynamicImage, ImageFormat};

use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::{http::Request, PageLoadEvent, WebViewBuilder};

use crate::browser::chrome::{privacy_init_js, TOOLBAR_JS, YT_ADBLOCK_JS};
use crate::browser::debug_panel::{open_settings_panel_js, toggle_debug_panel_js};
use crate::browser::ipc::{AppEvent, IpcMsg};
use crate::config::CatisenConfig;
use crate::history::HistoryEngine;
use crate::libcurl_download_manager::DownloadManager;
use crate::permissions::{
    is_insecure_http_origin, is_onion_origin, normalize_origin, PermissionManager,
    PermissionState, PermissionType,
};
use crate::privacy::stealth::pick_profile;
use wry::{ProxyConfig, ProxyEndpoint};
// Previously dead: detect_tor_proxy_status and maybe_probe_tor_route were never called.
// Now: the configured-proxy detector validates config.toml plus environment overrides
// so we get actual TCP reachability validation; maybe_probe_tor_route() runs on a
// background thread to log the exit-node IP via the Tor check API.
use crate::privacy::tor_manager::{
    configured_tor_proxy, detect_tor_proxy_status_for, maybe_probe_tor_route,
};
// Previously dead: SyncChain was declared but never constructed.
use crate::sync_chain::SyncChain;
use crate::tab_isolation::TabManager;
// Previously dead: ReaderMode was declared but never constructed or toggled.
use crate::text_mode::{ReaderMode, ReaderTheme};
use crate::ublock_integration::get_adblocker;
use crate::security_policy::{
    downloads_dir, ipc_origin_allowed, is_navigable, normalize_url, sanitize_download_path,
};

/// Entry point for the full GUI browser.
///
/// Initialises all core managers, builds the wry WebView, and runs the tao event loop.
pub fn run(mut config: CatisenConfig) -> Result<(), Box<dyn std::error::Error>> {

    // ── 1. Pre-initialise the global ad blocker ───────────────────────────────
    eprintln!("[Catisen] Initialising ad blocker…");
    let _ = get_adblocker();

    // ── 2. History Engine ─────────────────────────────────────────────────────
    eprintln!("[Catisen] Loading history engine…");
    let mut history = HistoryEngine::new();

    // ── 3. Tab Manager ────────────────────────────────────────────────────────
    eprintln!("[Catisen] Initialising tab manager…");
    let mut tab_manager = TabManager::new();
    let _initial_tab = tab_manager.create_new_tab(); // register tab 1

    // ── 4. Download Manager ───────────────────────────────────────────────────
    // Previously: no tor_proxy parameter existed — downloads always went over clearnet.
    // Now: we pass `active_tor_proxy` to start_download() so downloads route through
    // Tor when it is enabled (via ProxyType::Socks5Hostname in the libcurl Easy handle).
    eprintln!("[Catisen] Initialising download manager…");
    let mut download_manager = DownloadManager::new();

    // ── 5. Permission Manager ─────────────────────────────────────────────────
    // Previously dead: PermissionManager was declared in permissions.rs but never
    // constructed or called from any runtime code.
    //
    // Now: constructed here as Arc<Mutex<>> so it can be shared between:
    //   a) the navigation handler closure (Fn — must be Send + Sync shared ref)
    //   b) the event loop closure (owns the Arc directly)
    // It is used to auto-deny hardware permissions for HTTP sites, and updated
    // via the SetPermission IPC message from the Settings panel.
    eprintln!("[Catisen] Initialising permission manager…");
    let permission_manager = Arc::new(Mutex::new(PermissionManager::new()));
    // Clone for the navigation handler (which runs on a separate thread in wry).
    let pm_for_nav = Arc::clone(&permission_manager);

    // ── 6. Reader Mode ────────────────────────────────────────────────────────
    // Previously dead: ReaderMode was declared in text_mode.rs but never constructed.
    // Now: constructed here, toggled by ToggleReaderMode IPC (Ctrl+R / toolbar button).
    eprintln!("[Catisen] Initialising reader mode…");
    let mut reader_mode = ReaderMode::new();

    // ── 7. Sync Chain ─────────────────────────────────────────────────────────
    // Previously dead: SyncChain was declared in sync_chain.rs but generate_new_identity()
    // and generate_visual_qr_matrix() were never called.
    // Now: constructed here, activated by ShowSyncChain IPC (Settings panel button).
    eprintln!("[Catisen] Initialising sync chain…");
    let mut sync_chain = SyncChain::new();

    // ── 8. Tor proxy — detect and configure ──────────────────────────────────
    // Previously: the browser read config.tor_proxy_url directly and set env vars
    // without verifying the proxy was reachable.
    //
    // Now: resolve_tor_for_session() calls the configured-proxy detector from
    // privacy/tor_manager.rs to:
    //   • Perform a TCP reachability check before setting env vars
    //   • Pick up CATISEN_TOR_PROXY env var overrides
    //   • Log the proxy source (config default vs. env var)
    //
    // Returns None if Tor is disabled, Some(proxy_url) if it's on (reachable or not —
    // we still set env vars if configured so WebKit can fail gracefully).
    let mut active_tor_proxy: Option<String> = resolve_tor_for_session(&config);

    // §9.13: env::set_var is racy after threads exist. We do it once here at startup,
    // before any WebView or background threads, and only on non-Windows where WebKitGTK
    // respects env vars. On Windows, with_proxy_config is the real routing (see below).
    // §9.10 comment is now fixed: wry 0.38 has with_proxy_config with SOCKS5 support.
    let tor_proxy_for_probe: Option<String> = active_tor_proxy.clone();

    #[cfg(not(target_os = "windows"))]
    if let Some(ref proxy) = active_tor_proxy {
        // WebKitGTK path — must be before WebViewBuilder
        std::env::set_var("http_proxy", proxy);
        std::env::set_var("https_proxy", proxy);
        std::env::set_var("HTTP_PROXY", proxy);
        std::env::set_var("HTTPS_PROXY", proxy);
        std::env::set_var("no_proxy", "localhost,127.0.0.1");
        eprintln!("[Catisen] Tor proxy env configured (Linux/WebKitGTK): {}", proxy);
    }
    #[cfg(target_os = "windows")]
    if let Some(ref proxy) = active_tor_proxy {
        eprintln!("[Catisen] Tor proxy (Windows): will use with_proxy_config, not env vars: {}", proxy);
    }

    if let Some(ref proxy) = tor_proxy_for_probe {
        let proxy_for_thread = proxy.clone();
        std::thread::spawn(move || {
            maybe_probe_tor_route(&proxy_for_thread);
        });
    } else {
        eprintln!("[Catisen] Tor routing disabled.");
    }

    // ── 9. Privacy / stealth initialisation script ────────────────────────────
    // §9.9: use pick_profile() for consistent UA/platform/lang/tz tuple, and pass UA via with_user_agent
    // so real HTTP UA matches JS spoof.
    let (ua, platform, language, timezone) = pick_profile();
    let priv_js = privacy_init_js(ua, platform, language, timezone);
    let init_js = format!("{}\n{}\n{}", priv_js, TOOLBAR_JS, YT_ADBLOCK_JS);
    eprintln!("[Catisen] Stealth + toolbar script: {} bytes | UA: {} | platform: {} | lang: {} | tz: {}", init_js.len(), ua, platform, language, timezone);

    // ── 10. Event loop with typed user events ─────────────────────────────────
    let event_loop = EventLoopBuilder::<AppEvent>::with_user_event().build();
    let proxy_load = event_loop.create_proxy();
    let proxy_ipc  = event_loop.create_proxy();
    let proxy_newwin = event_loop.create_proxy();

    // ── 11. OS window ─────────────────────────────────────────────────────────
    let window = WindowBuilder::new()
        .with_title("Catisen — Privacy Browser")
        .with_inner_size(LogicalSize::new(1280.0_f64, 800.0_f64))
        .with_min_inner_size(LogicalSize::new(640.0_f64, 480.0_f64))
        .build(&event_loop)?;

    // ── 11b. Sync Chain native overlay window (XSS fix) ───────────────────────
    //
    // SECURITY: The sync seed phrase is sensitive key material.  The previous
    // implementation injected it into the main WebView DOM as an HTML element.
    // This is an XSS/data-exfiltration vulnerability: any script on the currently
    // loaded page could read it with `document.querySelector(...)` and POST it to
    // an attacker's server.
    //
    // FIX: We now create a completely separate tao `Window` + wry `WebView` for
    // the sync identity display.  The OS enforces the process/widget isolation —
    // the main page's JavaScript context cannot access DOM objects that belong to
    // a different window.  The seed data is passed into this isolated WebView via
    // Rust's `evaluate_script()`, which calls the `window.showSync()` stub defined
    // in the template HTML.
    //
    // The window starts hidden; it is shown and refreshed each time the user clicks
    // "Generate Sync Identity" in the Settings panel.
    let sync_window = WindowBuilder::new()
        .with_title("🔗 Catisen Sync Chain — Keep This Secret")
        .with_inner_size(LogicalSize::new(440.0_f64, 560.0_f64))
        .with_min_inner_size(LogicalSize::new(380.0_f64, 480.0_f64))
        .with_resizable(false)
        .with_visible(false) // hidden until ShowSyncChain fires
        .build(&event_loop)?;

    // Template HTML for the sync overlay.  All visual content lives here; the
    // seed and QR are injected later via `window.showSync(b64, seed)` from Rust.
    let sync_html = r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><style>
*{box-sizing:border-box;margin:0;padding:0}
body{background:#0d1117;color:#c9d1d9;font-family:'Cascadia Code','Fira Mono',monospace;
     display:flex;flex-direction:column;align-items:center;padding:24px;gap:14px;min-height:100vh}
h2{color:#388bfd;font-size:15px;font-weight:700}
.sub{color:#8b949e;font-size:11px}
#loading{color:#8b949e;font-size:13px;padding:20px 0}
#qr_img{width:200px;height:200px;image-rendering:pixelated;border:2px solid #30363d;display:none}
.seed-box{background:#161b22;border:1px solid #30363d;border-radius:4px;padding:12px;
          font-size:10px;word-break:break-all;color:#3fb950;width:100%;max-width:380px;display:none}
.warn{color:#d29922;font-size:11px;text-align:center;max-width:380px;display:none;line-height:1.6}
button{background:#21262d;border:1px solid #30363d;border-radius:4px;padding:8px 24px;
       color:#c9d1d9;font-family:inherit;font-size:12px;cursor:pointer;margin-top:4px}
button:hover{background:#30363d}
</style></head><body>
  <h2>🔗 Catisen Sync Chain</h2>
  <p class="sub" id="subtitle">Generating your identity…</p>
  <div id="loading">⏳</div>
  <img id="qr_img" alt="Sync QR Code" />
  <div id="seed_box" class="seed-box"></div>
  <p id="warn" class="warn">⚠️ Store this seed phrase securely.<br>
     Anyone who has it can access your sync chain.</p>
  <button onclick="window.close()">Close</button>
<script>
// showSync() is called by Rust (evaluate_script) with the base64-encoded QR PNG
// and the plaintext seed phrase.  All seed data stays in this isolated window —
// it never touches the main browsing WebView DOM.
window.showSync = function(b64, seed) {
    document.getElementById('loading').textContent = '';
    document.getElementById('subtitle').textContent = 'Scan on your other device to pair';
    var img = document.getElementById('qr_img');
    img.src = 'data:image/png;base64,' + b64;
    img.style.display = 'block';
    var sb = document.getElementById('seed_box');
    sb.textContent = seed;
    sb.style.display = 'block';
    document.getElementById('warn').style.display = 'block';
};
// Reset to loading state — called before each new identity generation.
window.setLoading = function() {
    document.getElementById('loading').textContent = '⏳';
    document.getElementById('qr_img').style.display = 'none';
    document.getElementById('seed_box').style.display = 'none';
    document.getElementById('warn').style.display = 'none';
    document.getElementById('subtitle').textContent = 'Generating your identity…';
};
</script></body></html>"#;

    let sync_wv = WebViewBuilder::new(&sync_window)
        .with_html(sync_html)
        .build()?;

    // Capture OS-level window IDs so the event loop can distinguish CloseRequested
    // events for the main browser window vs the sync overlay.
    let main_window_id = window.id();
    let sync_window_id = sync_window.id();

    let home = match std::env::var("CATISEN_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ if !config.home_page.trim().is_empty() => config.home_page.clone(),
        _ => "https://duckduckgo.com".to_string(),
    };
    if !is_navigable(&home) {
        return Err(format!("configured home page is not an HTTP(S) URL: {home}").into());
    }

    // ── 12. WebView ───────────────────────────────────────────────────────────
    // §9.9: with_user_agent so HTTP UA matches JS spoof
    // §9.10: with_proxy_config for real Tor routing (Windows + Linux)
    fn parse_proxy_endpoint(proxy_url: &str) -> Option<ProxyEndpoint> {
        // Try url::Url parse (handles socks5h:// etc)
        if let Ok(u) = url::Url::parse(proxy_url) {
            if !matches!(u.scheme(), "socks5" | "socks5h" | "socks4" | "socks4a") {
                return None;
            }
            if let Some(host) = u.host_str() {
                let port = u.port().unwrap_or(9150).to_string();
                return Some(ProxyEndpoint { host: host.to_string(), port });
            }
        }
        // If a URI was supplied but did not parse as an allowed SOCKS URI,
        // fail closed instead of guessing and accidentally launching clearnet.
        if proxy_url.contains("://") {
            return None;
        }
        // Fallback: parse a bare host:port
        let trimmed = proxy_url
            .trim_start_matches("socks5h://")
            .trim_start_matches("socks5://")
            .trim_start_matches("socks4a://")
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let host_port = trimmed.split('/').next().unwrap_or(trimmed);
        let host_port = host_port.split('@').last().unwrap_or(host_port);
        if host_port.is_empty() {
            return None;
        }
        let mut parts = host_port.rsplitn(2, ':');
        let port_str = parts.next()?;
        let host_part = parts.next();
        if let Some(h) = host_part {
            if let Ok(p) = port_str.parse::<u16>() {
                if !h.is_empty() {
                    return Some(ProxyEndpoint { host: h.to_string(), port: p.to_string() });
                }
            }
        } else {
            // No colon, just host
            if !port_str.is_empty() && !port_str.contains('.') && port_str.parse::<u16>().is_ok() {
                // Actually single part is port? treat as 127.0.0.1:port
                if let Ok(p) = port_str.parse::<u16>() {
                    return Some(ProxyEndpoint { host: "127.0.0.1".to_string(), port: p.to_string() });
                }
            } else if !host_port.is_empty() {
                return Some(ProxyEndpoint { host: host_port.to_string(), port: "9150".to_string() });
            }
        }
        None
    }

    let tor_required_for_navigation = active_tor_proxy.is_some();
    let mut webview_builder = WebViewBuilder::new(&window)
        .with_url(&home)
        .with_user_agent(&ua)
        .with_initialization_script(&init_js);

    // §9.10: if Tor enabled, route WebView through SOCKS5 proxy via wry's with_proxy_config
    // This is the actual fix for Windows (env vars ignored by WebView2)
    if let Some(ref proxy_url) = active_tor_proxy {
        if let Some(endpoint) = parse_proxy_endpoint(proxy_url) {
            eprintln!("[Tor] WebView with_proxy_config SOCKS5 {}:{}", endpoint.host, endpoint.port);
            webview_builder = webview_builder.with_proxy_config(ProxyConfig::Socks5(endpoint));
        } else {
            return Err(format!(
                "Tor is enabled but the proxy endpoint is invalid: {proxy_url}"
            )
            .into());
        }
    }

    let webview = webview_builder


        // Native ad/tracker blocking at the navigation level.
        // Also enforces geolocation permission denial for HTTP sites via PermissionManager.
        // P0 9.1: strict scheme + creds check via is_navigable (kills javascript:/data:/blob:/file:/about: + user:pass@)
        .with_navigation_handler(move |url: String| {
            if !is_navigable(&url) {
                eprintln!("[Security] Blocked non-navigable URL: {}", url);
                return false;
            }
            if is_onion_origin(&url) && !tor_required_for_navigation {
                eprintln!("[Security] Blocked .onion navigation without an active Tor route: {}", url);
                return false;
            }

            // ── Permission gate for HTTP sites ────────────────────────────────
            // §9.8: HTTP auto-deny now done here based on parsed scheme, not inside set_permission.
            // set_permission stores under normalized host (normalize_origin) so set/query agree.
            if is_insecure_http_origin(&url) {
                if let Ok(mut pm) = pm_for_nav.lock() {
                    let origin = normalize_origin(&url);
                    if !origin.is_empty() && origin != "*" {
                        pm.set_permission(&origin, PermissionType::Geolocation, PermissionState::Denied);
                        pm.set_permission(&origin, PermissionType::Camera, PermissionState::Denied);
                        pm.set_permission(&origin, PermissionType::Microphone, PermissionState::Denied);
                    }
                }
            }

            // Ad/tracker filter via EasyList engine
            let allowed = get_adblocker()
                .lock()
                .map(|b| !b.should_block_resource(&url, &url, "document"))
                // A poisoned blocker must fail closed, not silently allow the
                // navigation after a security component has failed.
                .unwrap_or(false);
            if !allowed {
                eprintln!("[AdBlock] Blocked: {}", url);
            }
            allowed
        })

        // Sync URL bar after every page finishes loading.
        // New-window request interceptor — previously MISSING entirely.
        // ROOT CAUSE of duplicate-toolbar bug: any window.open() not caught by
        // the ad-domain filter in privacy_init_js fell through to WebView2's
        // default new-window behavior, spawning a full second Catisen window
        // (inheriting the same profile-level injected scripts, hence the
        // cloned toolbar).
        //
        // FIX: intercept every new-window request here and redirect it into
        // the SAME webview via AppEvent::Navigate, instead of letting a
        // second native window spawn at all.
        //
        // KNOWN TRADE-OFF: OAuth flows relying on true popup + postMessage
        // back to the opener will not complete under same-tab redirect.
        // Real borderless popup support is separate future work.
        .with_new_window_req_handler(move |req_url: String| {
            if !is_navigable(&req_url) {
                eprintln!("[Security] Blocked new-window non-navigable URL: {}", req_url);
                return false;
            }
            if is_onion_origin(&req_url) && !tor_required_for_navigation {
                eprintln!("[Security] Blocked .onion new-window without an active Tor route: {}", req_url);
                return false;
            }
            eprintln!("[Catisen] New-window request -> same-tab nav: {}", req_url);
            proxy_newwin.send_event(AppEvent::Navigate(req_url)).ok();
            false
        })

        .with_on_page_load_handler(move |event, url| {
            if matches!(event, PageLoadEvent::Finished) {
                proxy_load.send_event(AppEvent::UpdateUrlBar(url)).ok();
            }
        })

        // IPC: toolbar JS → Rust event loop.
        // P0 9.3: authenticate IPC origin — block dangerous schemes, but allow unparseable platform URIs
        .with_ipc_handler(move |req: Request<String>| {
            if !ipc_origin_allowed(&req.uri().to_string()) {
                eprintln!("[Security] Blocked IPC from disallowed origin: {}", req.uri());
                return;
            }
            let msg = req.into_body();
            match serde_json::from_str::<IpcMsg>(&msg) {
                Ok(parsed) => {
                    let ev = match parsed {
                        IpcMsg::Navigate { url }              => AppEvent::Navigate(url),
                        IpcMsg::Back                          => AppEvent::Back,
                        IpcMsg::Forward                       => AppEvent::Forward,
                        IpcMsg::Reload                        => AppEvent::Reload,
                        IpcMsg::NewTab                        => AppEvent::NewTab,
                        IpcMsg::OpenSettings                  => AppEvent::OpenSettings,
                        IpcMsg::ToggleDebugPanel              => AppEvent::ToggleDebugPanel,
                        // NEW: wires text_mode::ReaderMode
                        IpcMsg::ToggleReaderMode              => AppEvent::ToggleReaderMode,
                        // NEW: wires sync_chain::SyncChain
                        IpcMsg::ShowSyncChain                 => AppEvent::ShowSyncChain,
                        // NEW: wires libcurl_download_manager with Tor proxy
                        IpcMsg::StartDownload { url, path }  => AppEvent::StartDownload { url, path },
                        IpcMsg::SetAdblock { enabled }        => AppEvent::SetAdblock { enabled },
                        IpcMsg::SetIsolation { enabled }      => AppEvent::SetIsolation { enabled },
                        IpcMsg::SetTor { enabled }            => AppEvent::SetTor { enabled },
                        // NEW: wires permissions::PermissionManager
                        IpcMsg::SetPermission { domain, permission, state } =>
                            AppEvent::SetPermission { domain, permission, state },
                        IpcMsg::SetSaveHistory { enabled } => AppEvent::SetSaveHistory { enabled },
                    };
                    proxy_ipc.send_event(ev).ok();
                }
                Err(e) => eprintln!("[Catisen] IPC parse error: {} — raw: {}", e, msg),
            }
        })
        .build()?;

    eprintln!("[Catisen] Browser window ready. Home: {}", home);

    // ── 13. Event loop ────────────────────────────────────────────────────────
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            // ── Window close ──────────────────────────────────────────────────
            // CloseRequested is fired for EVERY tao window.  We check which window
            // it came from so the sync overlay can be hidden without quitting the app.
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested, .. } => {
                if window_id == sync_window_id {
                    // Sync overlay: just hide it — don't destroy the WebView.
                    // The next ShowSyncChain can reuse it without rebuilding.
                    sync_window.set_visible(false);
                } else if window_id == main_window_id {
                    eprintln!("[Catisen] Window closed. Saving session history…");
                    if let Err(e) = history.save() {
                        eprintln!("[Catisen] Warning: could not save history: {}", e);
                    }
                    *control_flow = ControlFlow::Exit;
                }
            }

            // ── Custom events from IPC / page-load ────────────────────────────
            Event::UserEvent(app_event) => match app_event {

                // ── Navigate ──────────────────────────────────────────────────
                // P0 9.1: enforce navigable check even on AppEvent path (defense-in-depth)
                AppEvent::Navigate(raw) => {
                    let url = normalize_url(&raw);
                    if !is_navigable(&url)
                        || (is_onion_origin(&url) && !tor_required_for_navigation)
                    {
                        eprintln!("[Security] Blocked Navigate by URL/route policy: {}", url);
                    } else {
                        eprintln!("[Catisen] Navigating → {}", url);
                        let _ = webview.load_url(&url);
                    }
                }

                // ── Back / Forward / Reload ───────────────────────────────────
                AppEvent::Back => {
                    let _ = webview.evaluate_script("history.back()");
                }
                AppEvent::Forward => {
                    let _ = webview.evaluate_script("history.forward()");
                }
                AppEvent::Reload => {
                    let _ = webview.evaluate_script("location.reload()");
                }

                // ── New Tab ───────────────────────────────────────────────────
                AppEvent::NewTab => {
                    let tab = tab_manager.create_new_tab();
                    eprintln!("[Catisen] New tab ID={} path={}", tab.tab_id, tab.storage_path);
                    let _ = webview.load_url("https://duckduckgo.com");
                }

                // ── UpdateUrlBar ──────────────────────────────────────────────
                // Fires after every PageLoadEvent::Finished.  Records the visit in
                // history, enforces JS-level geolocation denial on HTTP pages
                // (complementing the Rust-side PermissionManager update in the nav handler),
                // and syncs the toolbar URL bar text.
                AppEvent::UpdateUrlBar(url) => {
                    // Record visit in history engine (skips chrome: / about:).
                    history.record_visit(&url, None);

                    // ── JS-level geolocation enforcement for HTTP sites ────────
                    // The nav handler updated the PermissionManager in Rust.  We also
                    // override navigator.geolocation in JS so that page scripts that call
                    // getCurrentPosition() on HTTP sites receive a denied error rather
                    // than a real location — closing the gap between the Rust record and
                    // in-page API access.
                    if is_insecure_http_origin(&url) {
                        let origin = normalize_origin(&url);
                        // query_permission confirms the state in the Rust record
                        let denied = permission_manager.lock()
                            .map(|pm| pm.query_permission(&origin, PermissionType::Geolocation)
                                       == PermissionState::Denied)
                            .unwrap_or(true);
                        if denied {
                            let _ = webview.evaluate_script(
                                "try { \
                                    navigator.geolocation.getCurrentPosition = \
                                        function(s,e){if(e)e({code:1,message:'Denied by Catisen: insecure origin'});}; \
                                    navigator.geolocation.watchPosition = \
                                        function(s,e){if(e)e({code:1,message:'Denied by Catisen: insecure origin'});return 0;}; \
                                } catch(_) {}"
                            );
                            eprintln!("[Permissions] Geolocation blocked via JS override for: {}", origin);
                        }
                    }

                    // Sync the JS toolbar URL bar text.
                    let safe = serde_json::to_string(&url).unwrap_or_else(|_| "\"\"".to_string());
                    let js = format!("if(window.__cat) window.__cat.updateUrl({safe});");
                    let _ = webview.evaluate_script(&js);

                    // Reflect hostname in OS window title bar.
                    let short = url
                        .trim_start_matches("https://")
                        .trim_start_matches("http://")
                        .split('/')
                        .next()
                        .unwrap_or(&url);
                    window.set_title(&format!("{} — Catisen", short));
                }

                // ── Settings overlay ──────────────────────────────────────────
                AppEvent::OpenSettings => {
                    eprintln!("[Catisen] Opening settings overlay");
                    let _ = webview.evaluate_script(&open_settings_panel_js());
                }

                // ── Debug panel overlay ───────────────────────────────────────
                AppEvent::ToggleDebugPanel => {
                    eprintln!("[Catisen] Toggling debug panel");
                    let _ = webview.evaluate_script(&toggle_debug_panel_js());
                }

                // ── Reader Mode (text_mode.rs) ────────────────────────────────
                // ReaderMode state is owned in text_mode.rs and the live DOM
                // transformation is generated here.
                //
                // Now: ToggleReaderMode IPC (Ctrl+R / toolbar button) calls
                // reader_mode.toggle() and either:
                //   ON  → injects CSS + DOM cleanup JS generated from the theme
                //   OFF → reloads the page to restore original styling
                AppEvent::ToggleReaderMode => {
                    reader_mode.toggle(!reader_mode.is_enabled);
                    if reader_mode.is_enabled {
                        eprintln!("[Catisen] Reader mode ON ({:?})", reader_mode.current_theme);
                        let js = reader_mode_enable_js(&reader_mode);
                        let _ = webview.evaluate_script(&js);
                    } else {
                        eprintln!("[Catisen] Reader mode OFF — reloading page");
                        // Reload restores original page HTML/CSS.
                        let _ = webview.evaluate_script("location.reload()");
                    }
                }

                // ── Sync Chain — NATIVE WINDOW (XSS security fix) ────────────
                //
                // SECURITY: The seed phrase is sensitive key material.
                //
                // PREVIOUS (VULNERABLE) approach: injected QR + seed into the main
                // WebView DOM as `document.documentElement.appendChild(panel)`.
                // ANY script on the current page could read the seed with a simple
                // `document.querySelector('.seed-box').textContent` call and POST
                // it to a remote server — a silent credential-exfiltration attack.
                //
                // CURRENT (SECURE) approach: seed and QR are displayed only in
                // `sync_wv`, a separate wry WebView inside `sync_window`.  The main
                // page's JS engine is sandboxed to its own window object and cannot
                // reach into a different OS-level window.  The isolation is enforced
                // by the OS process model and the WebView engine, not by our code.
                AppEvent::ShowSyncChain => {
                    eprintln!("[Catisen] ShowSyncChain: native overlay — generating identity…");

                    // Show the overlay window immediately with a spinner, so the user
                    // gets immediate visual feedback before the (potentially slow) QR
                    // matrix generation completes.
                    sync_window.set_visible(true);
                    let _ = sync_wv.evaluate_script(
                        "window.setLoading && window.setLoading()"
                    );

                    let seed = sync_chain.generate_new_identity();
                    match sync_chain.generate_visual_qr_matrix() {
                        Ok(qr_image) => {
                            // Encode the grayscale QR pixel matrix as a PNG so we can
                            // embed it as a base64 data URI inside the isolated WebView.
                            let dyn_img = DynamicImage::ImageLuma8(qr_image);
                            let mut cursor = Cursor::new(Vec::<u8>::new());
                            match dyn_img.write_to(&mut cursor, ImageFormat::Png) {
                                Ok(_) => {
                                    let png_bytes = cursor.into_inner();
                                    let b64       = STANDARD.encode(&png_bytes);
                                    // Serialize both values as JavaScript string literals.
                                    // The seed is currently hex, but keeping this boundary
                                    // generic prevents a future format change from becoming
                                    // an injection bug.
                                    let safe_b64 = serde_json::to_string(&b64)
                                        .unwrap_or_else(|_| "\"\"".to_string());
                                    let safe_seed = serde_json::to_string(&seed)
                                        .unwrap_or_else(|_| "\"\"".to_string());
                                    // Populate the isolated overlay — seed data NEVER
                                    // enters the main browsing WebView's DOM.
                                    let js = format!(
                                        "window.showSync && window.showSync({safe_b64}, {safe_seed})"
                                    );
                                    let _ = sync_wv.evaluate_script(&js);
                                    eprintln!("[Catisen] Sync identity displayed in native overlay.");
                                }
                                Err(e) => eprintln!("[Catisen] Sync chain: PNG encode failed: {}", e),
                            }
                        }
                        Err(e) => eprintln!("[Catisen] Sync chain: QR generation failed: {}", e),
                    }
                }

                // ── Start Download (libcurl_download_manager.rs) ──────────────
                // P0 9.2: confine download path via sanitize_download_path + downloads_dir()
                // P0 9.1: require is_navigable
                AppEvent::StartDownload { url, path } => {
                    if !is_navigable(&url)
                        || (is_onion_origin(&url) && active_tor_proxy.is_none())
                    {
                        eprintln!("[Security] Blocked download by URL/route policy: {}", url);
                        return;
                    }
                    let base = downloads_dir();
                    if let Err(e) = std::fs::create_dir_all(&base) {
                        eprintln!("[Download] Could not create downloads dir {:?}: {}", base, e);
                        return;
                    }
                    match sanitize_download_path(&path, &base) {
                        Ok(confined) => {
                            let confined_str = confined.to_string_lossy().to_string();
                            eprintln!("[Download] Starting: {} → {} | proxy={:?}", url, confined_str, active_tor_proxy);
                            download_manager.start_download(&url, &confined_str, active_tor_proxy.clone());
                        }
                        Err(e) => {
                            eprintln!("[Security] Rejected download path {:?}: {}", path, e);
                        }
                    }
                }

                // ── Ad blocker toggle ─────────────────────────────────────────
                AppEvent::SetAdblock { enabled } => {
                    eprintln!("[Catisen] Ad blocker → {}", if enabled { "ON" } else { "OFF" });
                    if let Ok(mut b) = get_adblocker().lock() {
                        b.set_enabled(enabled);
                    }
                }

                // ── Tab isolation toggle ──────────────────────────────────────
                AppEvent::SetIsolation { enabled } => {
                    eprintln!("[Catisen] Tab isolation → {}", if enabled { "ON" } else { "OFF" });
                    tab_manager.toggle_isolation(enabled);
                }

                // ── Tor toggle ────────────────────────────────────────────────
                // WebKitGTK env vars are applied at startup (before WebView build).
                // Changing them at runtime does not affect the already-running WebKit
                // process.  We log a clear note about this; the download manager DOES
                // respect the change immediately for new downloads started after this.
                AppEvent::SetTor { enabled } => {
                    active_tor_proxy = if enabled {
                        Some(configured_tor_proxy())
                    } else {
                        None
                    };
                    config.use_tor_by_default = enabled;
                    if let Err(error) = config.save() {
                        eprintln!("[Catisen] Failed to persist Tor preference: {error}");
                    }
                    eprintln!(
                        "[Catisen] Tor routing → {} \\
                         NOTE: WebView traffic requires restart to change; \\
                         new downloads use the updated route.",
                        if enabled { "ON" } else { "OFF" }
                    );
                }

                // ── Permission update (permissions.rs) ────────────────────────
                // Previously dead: PermissionManager.set_permission() was never called
                // from any runtime code path.  Now wired to the SetPermission IPC which
                // is fired by the Settings panel permission toggles.
                AppEvent::SetSaveHistory { enabled } => {
                    if let Err(error) = history.set_save_history(enabled) {
                        eprintln!("[History] Failed to update persistence setting: {error}");
                    } else {
                        eprintln!("[History] Persistent visit history → {}", if enabled { "ON" } else { "OFF" });
                    }
                }

                AppEvent::SetPermission { domain, permission, state } => {
                    let p_type = match permission.as_str() {
                        "geolocation"   => Some(PermissionType::Geolocation),
                        "camera"        => Some(PermissionType::Camera),
                        "microphone"    => Some(PermissionType::Microphone),
                        "notifications" => Some(PermissionType::Notifications),
                        other => {
                            eprintln!("[Permissions] Unknown permission type: {}", other);
                            None
                        }
                    };
                    let p_state = match state.as_str() {
                        "granted" => PermissionState::Granted,
                        "denied"  => PermissionState::Denied,
                        _         => PermissionState::Ask,
                    };
                    if let (Some(pt), Ok(mut pm)) = (p_type, permission_manager.lock()) {
                        eprintln!("[Permissions] {}/{} → {:?}", domain, permission, p_state);
                        if domain.trim() == "*" {
                            pm.set_global_default(pt, p_state);
                        } else {
                            pm.set_permission(&domain, pt, p_state);
                        }
                    }
                }
            },

            _ => {}
        }
    });
}

/// Determine the active Tor proxy URL for this browser session.
///
/// Calls the configured-proxy detector in `privacy::tor_manager`
/// instead of reading `config.tor_proxy_url` directly, so we get:
///   • TCP reachability validation before committing to a proxy
///   • CATISEN_TOR_PROXY env var override support
///   • Structured logging of the proxy source
///
/// Returns `None` if Tor is disabled in config; `Some(url)` if it's on
/// (reachable or not — we still configure it and let WebKit fail gracefully
/// with a clear error rather than silently falling back to clearnet).
fn resolve_tor_for_session(config: &CatisenConfig) -> Option<String> {
    if !config.use_tor_by_default {
        return None;
    }
    let status = detect_tor_proxy_status_for(Some(&config.tor_proxy_url));
    eprintln!(
        "[Tor] Proxy: {} | Source: {} | Reachable: {}",
        status.proxy, status.source, status.reachable
    );
    if !status.reachable {
        eprintln!(
            "[Tor] Warning: Tor is enabled in config but the proxy at {} is not \
             reachable.  Start the Tor daemon (tor/Tor Browser/Tails) and restart \
             Catisen for traffic to go through Tor.",
            status.proxy
        );
    }
    Some(status.proxy)
}

/// Generate JavaScript that applies Reader Mode to the current live page.
///
/// Called by the ToggleReaderMode handler when enabling the mode.
/// The JS removes known noise elements (scripts, iframes, videos, sidebars) and
/// applies the theme CSS from the Rust `ReaderMode` struct.
///
/// This bridges the Rust-side theme choice to the live WebView DOM/CSS path.
fn reader_mode_enable_js(reader: &ReaderMode) -> String {
    let (bg, fg, font, link) = match reader.current_theme {
        ReaderTheme::MentalityDark => ("#121212", "#E0E0E0", "Fira Code, sans-serif", "#f04747"),
        ReaderTheme::TechManual => ("#F8F9FA", "#333333", "Helvetica Neue, Arial, sans-serif", "#0055A4"),
        ReaderTheme::TerminalBlue => ("#0000B3", "#FFFFFF", "Courier New, monospace", "#FFCC00"),
    };

    // Build CSS as data and serialize it once. This avoids nesting font-family
    // quotes inside JavaScript string literals, which made every old theme's
    // generated script fail to parse.
    let css = format!(
        "body {{ background: {bg} !important; color: {fg} !important; font-family: {font} !important; margin: 5% auto !important; max-width: 800px !important; line-height: 1.8 !important; font-size: 18px !important; }} a {{ color: {link} !important; }} img {{ max-width: 100% !important; }}"
    );
    let css_js = serde_json::to_string(&css).unwrap_or_else(|_| "\"\"".to_string());

    format!(
        r#"
(function() {{
    ['script', 'iframe', 'video', 'aside',
     '[class*="sidebar"]', '[id*="sidebar"]',
     '[class*="ad-"]', '[id*="ad-"]'].forEach(function(sel) {{
        try {{ document.querySelectorAll(sel).forEach(function(el) {{ el.remove(); }}); }}
        catch(_) {{}}
    }});

    var style = document.getElementById('__cat_reader_css');
    if (!style) {{
        style = document.createElement('style');
        style.id = '__cat_reader_css';
        (document.head || document.documentElement).appendChild(style);
    }}
    style.textContent = {css_js};
    console.debug('[Catisen] Reader mode CSS applied');
}})();
    "#
    )
}

/// Extract just the `host:port` (or `host`) portion of a URL, for use as a
/// permission-manager domain key.
/// Deprecated: use `normalize_origin` from `permissions.rs` (lowercases, strips port).
#[allow(dead_code)]
fn extract_domain(url: &str) -> Option<String> {
    let without_scheme = url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let domain = without_scheme.split('/').next()?;
    if domain.is_empty() { None } else { Some(domain.to_string()) }
}
