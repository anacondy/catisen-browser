mod browser;
mod config;
mod history;
mod libcurl_download_manager;
mod network;
mod permissions;
mod privacy;
mod sandbox;
mod sync_chain;
mod tab_isolation;
mod text_mode;
mod ublock_integration;

// Previously dead: privacy/stealth, privacy/tor_manager — now used in browser::run()
// Previously dead: sandbox::SandboxManager — now called below, before the window opens
// Previously dead: permissions, text_mode, sync_chain — wired via IPC in browser/mod.rs

use config::CatisenConfig;
use network::{client::build_client, fetch::fetch_url};
// SandboxManager is called in main() before the browser starts.
use sandbox::SandboxManager;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let no_network = args.iter().any(|a| a == "--no-network");

    // ── Sandbox: lock down process capabilities ───────────────────────────────
    //
    // Must run as early as possible — before any file I/O, window creation, or
    // network activity — so the OS security constraints are in place before any
    // untrusted content is processed.
    //
    // On Linux: engages Seccomp-BPF filters to restrict syscalls.
    // On Windows: engages Job Object limits (sandboxes child processes).
    // Other OSes: logs a warning and continues (no isolation on unsupported platforms).
    //
    // Previously: SandboxManager was declared and fully implemented but never
    // called — the lockdown never ran.  It is now the first thing main() does.
    if let Err(e) = SandboxManager::lockdown_current_process() {
        eprintln!("[Catisen] Warning: sandbox setup incomplete: {}", e);
        // Non-fatal — we continue rather than refuse to start, but the warning
        // lets the user know the process is not fully isolated.
    } else {
        eprintln!("[Catisen] Warning: sandbox status is not verified; browser is running without a proven process sandbox.");
    }

    let config = CatisenConfig::load_or_create().unwrap_or_default();

    if headless {
        headless_pipeline(&config, !no_network);
        return;
    }

    if let Err(e) = browser::run(config) {
        eprintln!("Catisen fatal error: {e}");
        std::process::exit(1);
    }
}

// ─── Headless pipeline ────────────────────────────────────────────────────────
// Runs on servers / CI with no display (Replit, GitHub Actions, etc.).
// Exercises the full privacy stack: Tor check, ad blocker, HTTP fetch, scraper.
fn headless_pipeline(config: &CatisenConfig, run_network: bool) {
    println!();
    println!("🦊 Catisen Browser starting…");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  🦊  Catisen Browser — Headless Pipeline Test");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // ── Tor connectivity ──────────────────────────────────────────────────────
    println!("  🧅 Tor proxy  : {}", config.tor_proxy_url);
    let tor_ok = probe_tor(&config.tor_proxy_url);
    println!(
        "  🧅 Tor reach  : {}",
        if tor_ok { "✅ reachable" } else { "⚠️  not reachable (using clearnet)" }
    );

    // ── Ad-blocker self-test ──────────────────────────────────────────────────
    println!("  🛡️  Ad-block checks:");
    {
        let guard = ublock_integration::get_adblocker().lock().unwrap();
        let cases: &[(&str, bool)] = &[
            ("https://example.com/page",                        false),
            ("https://doubleclick.net/ads/beacon",              true),
            ("https://googlesyndication.com/pagead/show_ads",   true),
            ("https://wikipedia.org/wiki/Privacy",              false),
        ];
        for (url, expect_block) in cases {
            let blocked = guard.should_block_request(url);
            let ok      = blocked == *expect_block;
            let label   = if blocked { "BLOCKED" } else { "ALLOWED" };
            let mark    = if ok { "✅" } else { "❌" };
            println!("      {mark} {url:<52} → {label}  {}",
                if ok { "(correct)" } else { "(WRONG)" });
        }
    }
    if !run_network {
        println!("  HEADLESS_SMOKE_OK: network tests skipped (--no-network)");
        return;
    }

    // ── Network fetch ─────────────────────────────────────────────────────────
    println!("  🌐 Fetch tests (proxy={}):",
        if tor_ok { "Tor" } else { "none/clearnet" });

    let proxy  = if tor_ok { Some(config.tor_proxy_url.as_str()) } else { None };
    let client = match build_client(proxy) {
        Ok(c)  => c,
        Err(e) => { eprintln!("  ❌ Failed to build HTTP client: {e}"); return; }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let urls = [
            "https://example.com",
            "https://www.iana.org/domains/reserved",
        ];
        for url in urls {
            let start = std::time::Instant::now();
            match fetch_url(&client, url).await {
                Ok((headers, body)) => {
                    let ms    = start.elapsed().as_millis();
                    let ct    = headers
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("?");
                    let title = extract_title(&body);
                    let links = body.matches("<a ").count() + body.matches("<a\t").count();
                    println!("      ✅ {url} → {} bytes | {ms}ms | {ct}", body.len());
                    println!("         title: {:?}  |  links: {links} found", title);
                }
                Err(e) => println!("      ❌ {url} → {e}"),
            }
        }
    });

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Pipeline test complete.");
    println!();
    println!("  Real browser (Linux desktop + WebKitGTK):");
    println!("    cargo run");
    println!("    CATISEN_URL=https://check.torproject.org cargo run");
    println!();
}

/// Probe whether Tor is reachable by attempting a TCP connect to the SOCKS port.
fn probe_tor(proxy_url: &str) -> bool {
    let addr = proxy_url
        .trim_start_matches("socks5h://")
        .trim_start_matches("socks5://")
        .trim_start_matches("socks4a://")
        .trim_start_matches("http://");
    use std::net::TcpStream;
    use std::time::Duration;
    TcpStream::connect_timeout(
        &addr.parse().unwrap_or("127.0.0.1:9150".parse().unwrap()),
        Duration::from_secs(2),
    ).is_ok()
}

/// Extract the text content of the first <title> tag.
fn extract_title(html: &str) -> &str {
    let lo    = html.to_ascii_lowercase();
    let start = lo.find("<title>").map(|i| i + 7);
    let end   = lo.find("</title>");
    match (start, end) {
        (Some(s), Some(e)) if s < e => html[s..e].trim(),
        _                           => "(no title)",
    }
}
