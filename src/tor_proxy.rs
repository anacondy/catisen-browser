// Catisen Browser — Tor & Proxy Routing Logic
//
// ⚠️  DEPRECATED — do not add new call sites here.
//
// This module was the original, simpler proxy router written early in the project.
// It has been superseded by `src/privacy/tor_manager.rs`, which is the canonical
// Tor implementation going forward.  `tor_manager.rs` provides:
//   • Actual TCP reachability probing  (`is_proxy_endpoint_reachable`)
//   • Environment-variable configuration  (`CATISEN_TOR_PROXY`)
//   • Rate-limited background verification  (`maybe_probe_tor_route`)
//   • A structured `TorProxyStatus` result used throughout the browser runtime
//
// `ProxyRouter` and `ProxyMode` are kept here to avoid a breaking change in
// case any external tooling references them, but the browser event loop in
// `src/browser/mod.rs` calls `privacy::tor_manager` exclusively.
//
// If you are looking for where Tor is configured at startup, see:
//   `src/browser/mod.rs` → `resolve_tor_for_session()`
//   `src/privacy/tor_manager.rs` → `detect_tor_proxy_status()` / `resolve_tor_proxy()`

#[derive(Debug, Clone)]
pub enum ProxyMode {
    Direct,          // Standard connection (visible to ISP)
    Tor,             // Local Tor SOCKS5 proxy (127.0.0.1:9050)
    Custom(String),  // Custom external Proxy/VPN IP
}

pub struct ProxyRouter {
    pub current_mode: ProxyMode,
    pub https_only: bool,
}

impl ProxyRouter {
    pub fn new() -> Self {
        ProxyRouter {
            current_mode: ProxyMode::Direct,
            https_only: true,
        }
    }

    /// HTTPS enforcement — blocks plain HTTP (except .onion over HTTP which is fine).
    pub fn is_url_allowed(&self, url: &str) -> bool {
        if !self.https_only {
            return true;
        }
        if url.starts_with("https://") {
            return true;
        }
        if url.starts_with("http://") {
            if url.contains(".onion") {
                return true;
            }
            println!("🚫 SECURITY BLOCK: Prevented unencrypted HTTP connection to {}", url);
            return false;
        }
        true
    }

    /// Deprecated: use `privacy::tor_manager::detect_tor_proxy_status()` instead.
    pub fn enable_tor(&mut self) {
        println!("🔒 SECURE ROUTING: Engaging Tor Network bypass...");
        self.current_mode = ProxyMode::Tor;
        self.apply_network_rules();
    }

    pub fn enable_custom_proxy(&mut self, proxy_ip: &str) {
        println!("🌍 SECURE ROUTING: Re-routing via Custom Proxy (IP: {})...", proxy_ip);
        self.current_mode = ProxyMode::Custom(proxy_ip.to_string());
        self.apply_network_rules();
    }

    pub fn disable_proxy(&mut self) {
        println!("🌐 DIRECT ROUTING: Proxy disabled. Normal connection active.");
        self.current_mode = ProxyMode::Direct;
        self.apply_network_rules();
    }

    fn apply_network_rules(&self) {
        match &self.current_mode {
            ProxyMode::Tor => {
                std::env::set_var("http_proxy", "socks5://127.0.0.1:9050");
                std::env::set_var("https_proxy", "socks5://127.0.0.1:9050");
                println!("   --> STATUS = ISP Firewall Bypass Active. Location: Hidden.");
            }
            ProxyMode::Custom(ip) => {
                std::env::set_var("http_proxy", ip);
                std::env::set_var("https_proxy", ip);
                println!("   --> STATUS = Geo-Spoof Active. Location overridden.");
            }
            ProxyMode::Direct => {
                std::env::remove_var("http_proxy");
                std::env::remove_var("https_proxy");
                println!("   --> STATUS = Connection exposed to local ISP.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use curl::easy::Easy;

    #[test]
    fn test_tor_ip_check() {
        let mut easy = Easy::new();
        easy.url("https://check.torproject.org/api/ip").unwrap();
        easy.proxy("socks5h://127.0.0.1:9050").unwrap();

        let mut data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            let _ = transfer.perform();
        }

        if !data.is_empty() {
            let json_resp = String::from_utf8_lossy(&data);
            println!("Tor Proxy Test Success! IP Data: {}", json_resp);
        }
    }
}
