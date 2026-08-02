use std::sync::atomic::{AtomicU64, Ordering};

const DEFAULT_TOR_PROXY: &str = "socks5h://127.0.0.1:9150";

#[derive(Clone, Debug)]
pub struct TorProxyStatus {
    pub proxy: String,
    pub source: String,
    pub reachable: bool,
}

fn normalize_proxy_value(raw: &str) -> Option<String> {
    let value = raw.trim();
    if value.is_empty() {
        return None;
    }
    if value.contains("://") {
        return Some(value.to_string());
    }
    if let Ok(port) = value.parse::<u16>() {
        return Some(format!("socks5h://127.0.0.1:{port}"));
    }
    Some(value.to_string())
}

fn normalized_proxy_from_env() -> Option<String> {
    normalize_proxy_value(&std::env::var("CATISEN_TOR_PROXY").ok()?)
}

pub fn configured_tor_proxy() -> String {
    normalized_proxy_from_env().unwrap_or_else(|| DEFAULT_TOR_PROXY.to_string())
}

fn configured_proxy_source() -> String {
    if std::env::var("CATISEN_TOR_PROXY").ok().map(|v| !v.trim().is_empty()).unwrap_or(false) {
        "Environment (CATISEN_TOR_PROXY)".to_string()
    } else {
        format!("Default ({})", DEFAULT_TOR_PROXY)
    }
}

fn is_local_port_open(port: u16) -> bool {
    use std::net::{SocketAddr, TcpStream};

    let addr: SocketAddr = match format!("127.0.0.1:{}", port).parse() {
        Ok(a) => a,
        Err(_) => return false,
    };

    TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(400)).is_ok()
}

fn parse_proxy_host_port(proxy: &str) -> Option<(String, u16)> {
    let no_scheme = proxy.rsplit("://").next().unwrap_or(proxy);
    let host_port = no_scheme.rsplit('@').next().unwrap_or(no_scheme);
    let mut split = host_port.rsplitn(2, ':');
    let port = split.next()?.trim().parse::<u16>().ok()?;
    let host = split.next()?.trim();
    if host.is_empty() {
        return None;
    }
    Some((host.to_string(), port))
}

fn is_proxy_endpoint_reachable(proxy: &str) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};

    let (host, port) = match parse_proxy_host_port(proxy) {
        Some(value) => value,
        None => return false,
    };

    if (host == "127.0.0.1" || host.eq_ignore_ascii_case("localhost")) && is_local_port_open(port) {
        return true;
    }

    let addr_string = format!("{host}:{port}");
    addr_string
        .to_socket_addrs()
        .map(|addrs| {
            addrs.into_iter().any(|addr| {
                TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(600)).is_ok()
            })
        })
        .unwrap_or(false)
}

/// Resolve an optional config value while preserving the environment override.
/// This keeps the browser's config.toml proxy effective without introducing a
/// second Tor policy implementation.
pub fn detect_tor_proxy_status_for(configured: Option<&str>) -> TorProxyStatus {
    let (proxy, source) = if let Some(proxy) = normalized_proxy_from_env() {
        (proxy, "Environment (CATISEN_TOR_PROXY)".to_string())
    } else if let Some(proxy) = configured.and_then(normalize_proxy_value) {
        (proxy, "Config (tor_proxy_url)".to_string())
    } else {
        (configured_tor_proxy(), configured_proxy_source())
    };
    let reachable = is_proxy_endpoint_reachable(&proxy);
    TorProxyStatus {
        proxy,
        source,
        reachable,
    }
}

static LAST_TOR_PROBE_MS: AtomicU64 = AtomicU64::new(0);

fn extract_json_string_field(body: &str, field: &str) -> Option<String> {
    let needle = format!("\"{}\":\"", field);
    let start = body.find(&needle)? + needle.len();
    let tail = &body[start..];
    let end = tail.find('"')?;
    Some(tail[..end].to_string())
}

fn extract_json_bool_field(body: &str, field: &str) -> Option<bool> {
    let needle = format!("\"{}\":", field);
    let start = body.find(&needle)? + needle.len();
    let tail = body[start..].trim_start();
    if tail.starts_with("true") {
        Some(true)
    } else if tail.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

fn fetch_text_with_curl(url: &str, proxy: Option<&str>, timeout_secs: u64) -> Option<String> {
    use curl::easy::{Easy, ProxyType};

    let mut easy = Easy::new();
    easy.url(url).ok()?;
    let _ = easy.follow_location(true);
    let _ = easy.timeout(std::time::Duration::from_secs(timeout_secs));
    if let Some(proxy_addr) = proxy {
        easy.proxy(proxy_addr).ok()?;
        // Always use remote DNS for SOCKS proxy diagnostics. The default
        // configured URI is socks5h; this explicit setting also protects when
        // a caller supplies a bare/normalised endpoint.
        easy.proxy_type(ProxyType::Socks5Hostname).ok()?;
    }

    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        let _ = transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        });
        transfer.perform().ok()?;
    }

    Some(String::from_utf8_lossy(&data).to_string())
}

pub fn maybe_probe_tor_route(proxy: &str) {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let last = LAST_TOR_PROBE_MS.load(Ordering::Relaxed);
    if now_ms.saturating_sub(last) < 120_000 {
        return;
    }

    if LAST_TOR_PROBE_MS
        .compare_exchange(last, now_ms, Ordering::Relaxed, Ordering::Relaxed)
        .is_err()
    {
        return;
    }

    let proxy_addr = proxy.to_string();
    let tor_check = fetch_text_with_curl("https://check.torproject.org/api/ip", Some(&proxy_addr), 12);
    let exit_ip = tor_check
        .as_ref()
        .and_then(|body| extract_json_string_field(body, "IP"))
        .unwrap_or_else(|| "unknown".to_string());

    let is_tor = tor_check
        .as_ref()
        .and_then(|body| extract_json_bool_field(body, "IsTor"))
        .map(|v| if v { "yes" } else { "no" }.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // §9.12 fix: pass proxy to avoid clearnet leak (previously None leaked country lookup over clearnet)
    let exit_country = if exit_ip != "unknown" {
        fetch_text_with_curl(&format!("https://ipapi.co/{}/country_name/", exit_ip), Some(&proxy_addr), 8)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    };

    let _bridge_hint = std::env::var("CATISEN_TOR_BRIDGE_HINT")
        .unwrap_or_else(|_| "unknown (SOCKS only; ControlPort required for exact bridge)".to_string());

    eprintln!(
        "[Tor] Proxy={} IsTor={} ExitIP={} Country={}",
        proxy_addr, is_tor, exit_ip, exit_country
    );
}
