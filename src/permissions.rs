// Catisen Hardware & Prompt Permissions Engine
// §9.8: fix key mismatch — normalize_origin() used by both set and query.

use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PermissionType {
    Geolocation,
    Camera,
    Microphone,
    Notifications,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionState {
    Granted,
    Denied,
    Ask,
}

#[derive(Debug, Default)]
pub struct PermissionManager {
    pub site_permissions: HashMap<(String, PermissionType), PermissionState>,
}

/// Normalize an origin/domain to lowercase host without scheme/port.
/// - "*" stays "*"
/// - "https://Example.COM:8080/path" -> "example.com"
/// - "Example.COM:3000" -> "example.com"
/// - "example.com" -> "example.com"
/// - Lowercases, strips scheme, strips port, strips path/query
pub fn normalize_origin(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return String::new();
    }
    if s == "*" {
        return "*".to_string();
    }
    // Try url crate parse first (handles scheme)
    if let Ok(url) = url::Url::parse(s) {
        if let Some(host) = url.host_str() {
            return host.to_ascii_lowercase();
        }
    }
    // Try with https:// prefix for bare hosts
    let with_scheme = if s.contains("://") {
        s.to_string()
    } else {
        format!("https://{}", s)
    };
    if let Ok(url) = url::Url::parse(&with_scheme) {
        if let Some(host) = url.host_str() {
            return host.to_ascii_lowercase();
        }
    }

    // Fallback manual stripping
    let mut t = s;
    // Strip scheme
    if let Some(idx) = t.find("://") {
        t = &t[idx + 3..];
    }
    // Strip path/query/fragment
    t = t.split(&['/', '?', '#'][..]).next().unwrap_or("");
    // Strip port
    t = t.split(':').next().unwrap_or("");
    // Strip userinfo @
    if let Some(at) = t.rfind('@') {
        t = &t[at + 1..];
    }
    t.to_ascii_lowercase()
}

impl PermissionManager {
    pub fn new() -> Self {
        PermissionManager {
            site_permissions: HashMap::new(),
        }
    }

    /// Queries permission — uses normalized host, falls back to "*" wildcard.
    pub fn query_permission(&self, domain: &str, p_type: PermissionType) -> PermissionState {
        let norm = normalize_origin(domain);
        if norm.is_empty() {
            return PermissionState::Ask;
        }
        // Try exact
        if let Some(state) = self
            .site_permissions
            .get(&(norm.clone(), p_type.clone()))
        {
            return state.clone();
        }
        // Fallback to wildcard "*"
        if let Some(state) = self
            .site_permissions
            .get(&("*".to_string(), p_type.clone()))
        {
            return state.clone();
        }
        PermissionState::Ask
    }

    /// Updates permission — stores under normalized host, no scheme heuristic.
    /// The HTTP auto-deny is now done by the caller in browser/mod.rs based on parsed scheme.
    pub fn set_permission(&mut self, domain: &str, p_type: PermissionType, state: PermissionState) {
        let norm = normalize_origin(domain);
        if norm.is_empty() {
            return;
        }
        self.site_permissions
            .insert((norm, p_type), state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_origin() {
        assert_eq!(normalize_origin("https://Example.COM:8080/path"), "example.com");
        assert_eq!(normalize_origin("http://example.com"), "example.com");
        assert_eq!(normalize_origin("Example.COM:3000"), "example.com");
        assert_eq!(normalize_origin("example.com"), "example.com");
        assert_eq!(normalize_origin("EXAMPLE.COM/path?q=1"), "example.com");
        assert_eq!(normalize_origin("https://example.com:443"), "example.com");
        assert_eq!(normalize_origin("*"), "*");
        assert_eq!(normalize_origin("  https://Example.com  "), "example.com");
        assert_eq!(normalize_origin("http://sub.example.com:3000/"), "sub.example.com");
    }

    #[test]
    fn test_set_query_agree() {
        let mut manager = PermissionManager::new();
        // Set with scheme, query without
        manager.set_permission(
            "https://example.com/path",
            PermissionType::Geolocation,
            PermissionState::Granted,
        );
        assert_eq!(
            manager.query_permission("example.com", PermissionType::Geolocation),
            PermissionState::Granted
        );
        assert_eq!(
            manager.query_permission("https://example.com", PermissionType::Geolocation),
            PermissionState::Granted
        );
        assert_eq!(
            manager.query_permission("https://example.com:8080", PermissionType::Geolocation),
            PermissionState::Granted
        );
        assert_eq!(
            manager.query_permission("EXAMPLE.COM", PermissionType::Geolocation),
            PermissionState::Granted
        );
    }

    #[test]
    fn test_strict_http_rejection_via_caller() {
        // New design: set_permission itself does NOT auto-deny based on scheme heuristic.
        // The caller (browser/mod.rs) does HTTP auto-deny from navigation URL's parsed scheme.
        // This test simulates that caller logic.
        let mut manager = PermissionManager::new();
        let http_url = "http://insecure.test/page";
        // Simulate navigation handler: if http and not .onion, auto-deny
        if http_url.starts_with("http://") && !http_url.contains(".onion") {
            manager.set_permission(
                http_url,
                PermissionType::Camera,
                PermissionState::Denied,
            );
        }
        let result = manager.query_permission("http://insecure.test", PermissionType::Camera);
        assert_eq!(result, PermissionState::Denied);

        // HTTPS should not auto-deny
        let mut manager2 = PermissionManager::new();
        let https_url = "https://secure.test/page";
        // Caller would NOT auto-deny for https
        // If user grants, it should be Granted
        manager2.set_permission(
            https_url,
            PermissionType::Camera,
            PermissionState::Granted,
        );
        let result2 = manager2.query_permission("secure.test", PermissionType::Camera);
        assert_eq!(result2, PermissionState::Granted);
    }

    #[test]
    fn test_wildcard_fallback() {
        let mut manager = PermissionManager::new();
        // Set wildcard
        manager.set_permission("*", PermissionType::Geolocation, PermissionState::Denied);
        // Any domain should fallback to wildcard if no exact match
        assert_eq!(
            manager.query_permission("example.com", PermissionType::Geolocation),
            PermissionState::Denied
        );
        assert_eq!(
            manager.query_permission("other.org", PermissionType::Geolocation),
            PermissionState::Denied
        );
        // Exact overrides wildcard
        manager.set_permission(
            "example.com",
            PermissionType::Geolocation,
            PermissionState::Granted,
        );
        assert_eq!(
            manager.query_permission("example.com", PermissionType::Geolocation),
            PermissionState::Granted
        );
        assert_eq!(
            manager.query_permission("other.org", PermissionType::Geolocation),
            PermissionState::Denied
        );
    }

    #[test]
    fn test_http_auto_deny_simulation() {
        // Ensure HTTP URL auto-denies geolocation when caller does it
        let mut manager = PermissionManager::new();
        let url = "http://example.com/";
        // Simulate browser logic
        let is_http = url.starts_with("http://");
        if is_http && !url.contains(".onion") {
            manager.set_permission(url, PermissionType::Geolocation, PermissionState::Denied);
        }
        assert_eq!(
            manager.query_permission("example.com", PermissionType::Geolocation),
            PermissionState::Denied
        );

        // HTTPS does not auto-deny
        let mut manager2 = PermissionManager::new();
        let url2 = "https://example.com/";
        let is_http2 = url2.starts_with("http://");
        if is_http2 && !url2.contains(".onion") {
            manager2.set_permission(url2, PermissionType::Geolocation, PermissionState::Denied);
        } else {
            // No auto-deny, default Ask
            assert_eq!(
                manager2.query_permission("example.com", PermissionType::Geolocation),
                PermissionState::Ask
            );
        }
    }
}
