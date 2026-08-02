// Catisen Hardware & Prompt Permissions Engine
//
// Permission keys are exact web origins (scheme + host + effective port), not
// bare hostnames.  Collapsing HTTP, HTTPS, and different ports into one key can
// let a grant made for one origin affect a different origin.

use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PermissionType {
    Geolocation,
    Camera,
    Microphone,
    Notifications,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionState {
    Granted,
    Denied,
    Ask,
}

#[derive(Debug, Default)]
pub struct PermissionManager {
    /// Exact-origin entries. The literal "*" is reserved for a global default.
    pub site_permissions: HashMap<(String, PermissionType), PermissionState>,
}

/// Canonicalize a permission key to an HTTP(S) origin.
///
/// A bare host is treated as HTTPS for backwards compatibility with old config
/// entries, but new callers should always pass `location.origin` or the full
/// navigation URL. Invalid/non-web origins return an empty key and are ignored.
pub fn normalize_origin(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return String::new();
    }
    if s == "*" {
        return "*".to_string();
    }

    let candidate = if s.contains("://") {
        s.to_string()
    } else {
        format!("https://{s}")
    };

    let Ok(url) = url::Url::parse(&candidate) else {
        return String::new();
    };
    if url.scheme() != "http" && url.scheme() != "https" {
        return String::new();
    }
    if !url.username().is_empty() || url.password().is_some() {
        return String::new();
    }
    let Some(host) = url.host_str() else {
        return String::new();
    };

    // Host strings for IPv6 literals need brackets in an origin serialization.
    let host = if host.contains(':') {
        format!("[{host}]")
    } else {
        host.to_ascii_lowercase()
    };
    let scheme = url.scheme().to_ascii_lowercase();
    let default_port = match scheme.as_str() {
        "http" => 80,
        "https" => 443,
        _ => unreachable!(),
    };
    let port = url.port().filter(|port| *port != default_port);

    if let Some(port) = port {
        format!("{scheme}://{host}:{port}")
    } else {
        format!("{scheme}://{host}")
    }
}

/// Return true when an origin is an ordinary insecure HTTP origin.
/// `.onion` is considered potentially trustworthy by modern browsers, but the
/// navigation layer still controls whether it is reachable through Tor.
pub fn is_insecure_http_origin(input: &str) -> bool {
    let normalized = normalize_origin(input);
    if !normalized.starts_with("http://") {
        return false;
    }
    let host = normalized
        .trim_start_matches("http://")
        .split(':')
        .next()
        .unwrap_or("")
        .trim_matches(&['[', ']'][..]);
    !host.eq_ignore_ascii_case("onion") && !host.to_ascii_lowercase().ends_with(".onion")
}

/// Return true only for an exact `.onion` hostname, not a path or a larger
/// hostname containing the substring.
pub fn is_onion_origin(input: &str) -> bool {
    let candidate = if input.contains("://") {
        input.trim().to_string()
    } else {
        format!("https://{}", input.trim())
    };
    let Ok(url) = url::Url::parse(&candidate) else {
        return false;
    };
    url.host_str()
        .map(|host| host.eq_ignore_ascii_case("onion") || host.to_ascii_lowercase().ends_with(".onion"))
        .unwrap_or(false)
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            site_permissions: HashMap::new(),
        }
    }

    /// Query an exact origin, then fall back to a per-permission global default.
    pub fn query_permission(&self, origin: &str, p_type: PermissionType) -> PermissionState {
        let key = normalize_origin(origin);
        if key.is_empty() {
            return PermissionState::Ask;
        }
        if let Some(state) = self.site_permissions.get(&(key, p_type.clone())) {
            return state.clone();
        }
        self.site_permissions
            .get(&("*".to_string(), p_type))
            .cloned()
            .unwrap_or(PermissionState::Ask)
    }

    /// Store a site-specific or wildcard permission. Grants for ordinary HTTP
    /// origins are converted to Denied so a UI toggle cannot bypass the secure
    /// origin policy.
    pub fn set_permission(&mut self, origin: &str, p_type: PermissionType, state: PermissionState) {
        let key = normalize_origin(origin);
        if key.is_empty() {
            return;
        }
        let effective = if state == PermissionState::Granted && is_insecure_http_origin(&key) {
            PermissionState::Denied
        } else {
            state
        };
        self.site_permissions.insert((key, p_type), effective);
    }

    /// Explicit name for callers that are updating the global default. The
    /// wildcard is retained in the same map so old serialized/settings paths
    /// remain compatible.
    pub fn set_global_default(&mut self, p_type: PermissionType, state: PermissionState) {
        self.site_permissions.insert(("*".to_string(), p_type), state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_origin_normalization_preserves_scheme_and_port() {
        assert_eq!(normalize_origin("https://Example.COM:443/path"), "https://example.com");
        assert_eq!(normalize_origin("http://Example.COM:80/path"), "http://example.com");
        assert_eq!(normalize_origin("https://example.com:8443/path"), "https://example.com:8443");
        assert_eq!(normalize_origin("http://example.com:8080"), "http://example.com:8080");
        assert_eq!(normalize_origin("Example.COM:3000"), "https://example.com:3000");
        assert_eq!(normalize_origin("*"), "*");
    }

    #[test]
    fn http_https_and_ports_do_not_share_grants() {
        let mut manager = PermissionManager::new();
        manager.set_permission(
            "https://example.com:8443/path",
            PermissionType::Camera,
            PermissionState::Granted,
        );
        assert_eq!(
            manager.query_permission("https://example.com:8443", PermissionType::Camera),
            PermissionState::Granted
        );
        assert_eq!(
            manager.query_permission("https://example.com", PermissionType::Camera),
            PermissionState::Ask
        );
        assert_eq!(
            manager.query_permission("http://example.com:8443", PermissionType::Camera),
            PermissionState::Ask
        );
    }

    #[test]
    fn ordinary_http_grant_is_forced_denied() {
        let mut manager = PermissionManager::new();
        manager.set_permission(
            "http://insecure.test/path",
            PermissionType::Camera,
            PermissionState::Granted,
        );
        assert_eq!(
            manager.query_permission("http://insecure.test", PermissionType::Camera),
            PermissionState::Denied
        );
    }

    #[test]
    fn wildcard_fallback_and_exact_override() {
        let mut manager = PermissionManager::new();
        manager.set_global_default(PermissionType::Geolocation, PermissionState::Denied);
        assert_eq!(
            manager.query_permission("https://other.example", PermissionType::Geolocation),
            PermissionState::Denied
        );
        manager.set_permission(
            "https://trusted.example",
            PermissionType::Geolocation,
            PermissionState::Granted,
        );
        assert_eq!(
            manager.query_permission("https://trusted.example", PermissionType::Geolocation),
            PermissionState::Granted
        );
    }

    #[test]
    fn onion_detection_checks_hostname_not_path() {
        assert!(is_onion_origin("http://example.onion/path"));
        assert!(!is_onion_origin("http://example.com/path.onion"));
        assert!(!is_onion_origin("http://example.onion.example"));
    }
}
