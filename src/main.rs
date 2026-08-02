mod browser;
mod config;
mod history;
mod libcurl_download_manager;
mod network;
mod permissions;
mod privacy;
mod sandbox;
mod security_policy;
mod sync_chain;
mod tab_isolation;
mod text_mode;
mod ublock_integration;

use config::CatisenConfig;
use network::{
    client::build_client_with_timeout,
    fetch::fetch_url,
};
use sandbox::SandboxManager;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");
    let no_network = args.iter().any(|arg| arg == "--no-network");
    let use_tor_flag = args.iter().any(|arg| arg == "--tor");
    let cli_url = cli_flag_value(&args, "--url");
    let text_only = std::env::var("CATISEN_VIEW_MODE")
        .map(|value| value.eq_ignore_ascii_case("textonly"))
        .unwrap_or(false);

    // Apply process hardening before configuration, WebView creation, or any
    // network activity. A failure is visible and non-fatal because some hosts
    // already place the process in a job/container; it is never reported as a
    // successful full sandbox.
    if let Err(error) = SandboxManager::lockdown_current_process() {
        eprintln!("[Catisen] Process hardening unavailable: {error}");
    }

    let config = CatisenConfig::load_or_create().unwrap_or_default();

    // This one-shot path is intentionally checked before GUI startup so CI and
    // diagnostics can fetch a URL without opening WebView2/WebKit.
    if let Some(url) = cli_url {
        if text_only {
            text_fetch_mode(&url, use_tor_flag || config.use_tor_by_default, &config);
            return;
        }
    }

    if headless {
        headless_pipeline(
            &config,
            !no_network,
            use_tor_flag || config.use_tor_by_default,
        );
        return;
    }

    if let Err(error) = browser::run(config) {
        eprintln!("Catisen fatal error: {error}");
        std::process::exit(1);
    }
}

fn cli_flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|index| args.get(index + 1))
        .filter(|value| !value.starts_with('-'))
        .cloned()
}

fn configured_proxy_url(config: &CatisenConfig) -> String {
    std::env::var("CATISEN_TOR_PROXY")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| config.tor_proxy_url.clone())
}

fn text_fetch_mode(url: &str, use_tor: bool, config: &CatisenConfig) {
    let log_path = std::env::var("CATISEN_LOG_FILE").ok();
    let proxy_url = configured_proxy_url(config);

    if use_tor && !probe_tor(&proxy_url) {
        let line = format!(
            "❌ Network Error: Tor was required but the proxy is unreachable: {proxy_url}"
        );
        eprintln!("{line}");
        write_req_log(log_path.as_deref(), &line);
        return;
    }

    let proxy = if use_tor {
        Some(proxy_url.as_str())
    } else {
        None
    };
    let client = match build_client_with_timeout(proxy, config.request_timeout_secs) {
        Ok(client) => client,
        Err(error) => {
            let line = format!("❌ Network Error: failed to build HTTP client: {error}");
            eprintln!("{line}");
            write_req_log(log_path.as_deref(), &line);
            return;
        }
    };

    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => {
            let line = format!("❌ Network Error: failed to start async runtime: {error}");
            eprintln!("{line}");
            write_req_log(log_path.as_deref(), &line);
            return;
        }
    };

    runtime.block_on(async {
        let start = std::time::Instant::now();
        match fetch_url(
            &client,
            url,
            config.request_retries.max(1),
            use_tor,
        )
        .await
        {
            Ok((status, _headers, body)) => {
                let elapsed = start.elapsed().as_millis();
                let line = format!("[REQ] status={status} bytes={} full={elapsed}", body.len());
                println!("{line}");
                write_req_log(log_path.as_deref(), &line);
            }
            Err(error) => {
                let line = format!("❌ Network Error: {error}");
                println!("{line}");
                write_req_log(log_path.as_deref(), &line);
            }
        }
    });
}

fn write_req_log(path: Option<&str>, line: &str) {
    if let Some(path) = path {
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = writeln!(file, "{line}");
        }
    }
}

fn headless_pipeline(config: &CatisenConfig, run_network: bool, use_tor: bool) {
    println!();
    println!("🦊 Catisen Browser starting…");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  🦊  Catisen Browser — Headless Pipeline Test");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("  🛡️  Ad-block checks:");
    {
        let guard = match ublock_integration::get_adblocker().lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("  ❌ Ad blocker lock is poisoned; refusing self-test");
                return;
            }
        };
        let cases: &[(&str, bool)] = &[
            ("https://example.com/page", false),
            ("https://doubleclick.net/ads/beacon", true),
            ("https://googlesyndication.com/pagead/show_ads", true),
            ("https://wikipedia.org/wiki/Privacy", false),
        ];
        for (url, expected_block) in cases {
            let blocked = guard.should_block_request(url);
            let ok = blocked == *expected_block;
            println!(
                "      {} {url:<52} → {} {}",
                if ok { "✅" } else { "❌" },
                if blocked { "BLOCKED" } else { "ALLOWED" },
                if ok { "(correct)" } else { "(WRONG)" }
            );
        }
    }

    // --no-network must not even probe a Tor socket.
    if !run_network {
        println!("  HEADLESS_SMOKE_OK: network tests skipped (--no-network)");
        return;
    }

    let proxy_url = configured_proxy_url(config);
    let tor_reachable = if use_tor {
        probe_tor(&proxy_url)
    } else {
        false
    };
    println!("  🧅 Tor proxy  : {proxy_url}");
    println!(
        "  🧅 Tor reach  : {}",
        if !use_tor {
            "disabled"
        } else if tor_reachable {
            "✅ reachable"
        } else {
            "❌ required but unreachable"
        }
    );

    if use_tor && !tor_reachable {
        eprintln!("  ❌ Tor is required; refusing a clearnet fallback.");
        return;
    }

    let proxy = if use_tor {
        Some(proxy_url.as_str())
    } else {
        None
    };
    let client = match build_client_with_timeout(proxy, config.request_timeout_secs) {
        Ok(client) => client,
        Err(error) => {
            eprintln!("  ❌ Failed to build HTTP client: {error}");
            return;
        }
    };

    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("  ❌ Failed to start async runtime: {error}");
            return;
        }
    };
    runtime.block_on(async {
        for url in [
            "https://example.com",
            "https://www.iana.org/domains/reserved",
        ] {
            let start = std::time::Instant::now();
            match fetch_url(
                &client,
                url,
                config.request_retries.max(1),
                use_tor,
            )
            .await
            {
                Ok((status, headers, body)) => {
                    let elapsed = start.elapsed().as_millis();
                    let content_type = headers
                        .get("content-type")
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("?");
                    println!(
                        "      ✅ {url} → {status} | {} bytes | {elapsed}ms | {content_type}",
                        body.len()
                    );
                    println!(
                        "         title: {:?} | links: {} found",
                        extract_title(&body),
                        body.matches("<a ").count() + body.matches("<a\t").count()
                    );
                }
                Err(error) => println!("      ❌ {url} → {error}"),
            }
        }
    });

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Pipeline test complete.");
}

/// Probe the configured proxy endpoint, including hostname resolution. A
/// malformed or unresolvable endpoint is false; there is no default-address
/// fallback because that can report the wrong proxy as healthy.
fn probe_tor(proxy_url: &str) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Duration;

    let endpoint = match url::Url::parse(proxy_url) {
        Ok(parsed) => {
            let Some(host) = parsed.host_str() else { return false };
            let Some(port) = parsed.port() else { return false };
            if host.contains(':') {
                format!("[{host}]:{port}")
            } else {
                format!("{host}:{port}")
            }
        }
        Err(_) => {
            let value = proxy_url
                .trim_start_matches("socks5h://")
                .trim_start_matches("socks5://")
                .trim_start_matches("socks4a://")
                .trim_start_matches("http://");
            value.to_string()
        }
    };

    endpoint
        .to_socket_addrs()
        .map(|addresses| {
            addresses.into_iter().any(|address| {
                TcpStream::connect_timeout(&address, Duration::from_secs(2)).is_ok()
            })
        })
        .unwrap_or(false)
}

fn extract_title(html: &str) -> &str {
    let lower = html.to_ascii_lowercase();
    let start = lower.find("<title>").map(|index| index + 7);
    let end = lower.find("</title>");
    match (start, end) {
        (Some(start), Some(end)) if start < end => html[start..end].trim(),
        _ => "(no title)",
    }
}
