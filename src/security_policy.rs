//! Centralised security policy helpers (P0 fixes).
//!
//! Pure functions — no I/O except `downloads_dir()` returning a PathBuf.
//! All validation uses the `url` crate already in Cargo.toml.

use std::path::{Path, PathBuf};

/// Normalise raw address-bar input into a fully-qualified URL.
///
/// * empty / whitespace only → DDG home
/// * contains "://" → as-is (scheme already present; validation happens later via `is_navigable`)
/// * no whitespace and (contains '.' or starts_with "localhost") → prefix "https://"
/// * else → DDG search
pub fn normalize_url(input: &str) -> String {
    let s = input.trim();
    if s.is_empty() {
        return "https://duckduckgo.com".to_string();
    }
    if s.contains("://") {
        return s.to_string();
    }
    if !s.contains(' ') && (s.contains('.') || s.starts_with("localhost")) {
        return format!("https://{}", s);
    }
    // Collapse runs of whitespace for the same address-bar behavior users
    // expect from a search box, then let form_urlencoded escape punctuation.
    let query = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let q = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", &query)
        .finish();
    format!("https://duckduckgo.com/?{q}")
}

/// Returns true iff `raw` is safe to navigate the WebView to.
///
/// * Must parse via `url::Url::parse`
/// * Scheme must be http or https only (Url parser lower-cases, so case-insensitive)
/// * username must be empty and password must be None (reject creds-in-URL)
/// * host must be present and non-empty
/// * Additionally, reject empty authority like "https:///path" via manual check
///
/// This kills javascript:, data:, blob:, file:, about:, etc., plus credentialed URLs.
pub fn is_navigable(raw: &str) -> bool {
    // Manual empty-authority guard: catches "https:///path" and "https://" even if url crate is lenient
    if let Some(pos) = raw.find("://") {
        let after = &raw[pos + 3..];
        if after.is_empty() {
            return false;
        }
        // host is up to first '/' '?' '#'
        let host_part = after
            .split(&['/', '?', '#'][..])
            .next()
            .unwrap_or("");
        if host_part.is_empty() {
            return false;
        }
        // Also reject if authority starts with "/" (should have been caught by empty host, but explicit)
        if after.starts_with('/') {
            return false;
        }
    }

    let parsed = match url::Url::parse(raw) {
        Ok(u) => u,
        Err(_) => return false,
    };
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return false;
    }
    if !parsed.username().is_empty() {
        return false;
    }
    if parsed.password().is_some() {
        return false;
    }
    match parsed.host_str() {
        Some(h) if !h.is_empty() => {},
        _ => return false,
    }
    // Extra: ensure host() is Some, not just host_str
    if parsed.host().is_none() {
        return false;
    }
    true
}

/// Confine a user-supplied download `req` path to `base`.
///
/// Only the basename (`file_name()`) is kept; directory components are stripped.
/// Rejects empty, "." , "..", and any basename containing '/' '\' or '\0'.
/// Returns `base.join(basename)` and asserts it starts_with(base).
pub fn sanitize_download_path(req: &str, base: &Path) -> Result<PathBuf, String> {
    // Extract basename via file_name()
    let p = Path::new(req);
    let fname_os = p.file_name().ok_or_else(|| format!("invalid download path: {:?}", req))?;
    let fname_str = fname_os.to_string_lossy();

    // Reject empty / "." / ".."
    if fname_str.is_empty() || fname_str == "." || fname_str == ".." {
        return Err(format!("rejected download name: {:?}", fname_str));
    }
    // Reject separators, NUL, ADS syntax, and device-name syntax. `file_name()`
    // strips directory components, but it does not make a Windows colon or
    // device name safe.
    if fname_str.contains('/')
        || fname_str.contains('\\')
        || fname_str.contains('\0')
        || fname_str.contains(':')
    {
        return Err(format!("rejected download name (separator/NUL/ADS): {:?}", fname_str));
    }
    #[cfg(windows)]
    {
        let stem = fname_str
            .split('.')
            .next()
            .unwrap_or("")
            .to_ascii_uppercase();
        let numbered_device = ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix)
                .map(|number| {
                    number.len() == 1
                        && number
                            .chars()
                            .next()
                            .map(|digit| ('1'..='9').contains(&digit))
                            .unwrap_or(false)
                })
                .unwrap_or(false)
        });
        if matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL") || numbered_device {
            return Err(format!("rejected Windows device name: {:?}", fname_str));
        }
    }

    let mut out = PathBuf::from(base);
    out.push(fname_os);

    // Ensure confined
    if !out.starts_with(base) {
        return Err(format!("download path escapes base: {:?}", out));
    }
    Ok(out)
}

/// Downloads directory (relative to CWD). Callers must create it.
pub fn downloads_dir() -> PathBuf {
    PathBuf::from("downloads")
}

/// Returns true if IPC `uri` is allowed to issue commands.
///
/// If URI parses successfully and its scheme is in the blocklist
/// {javascript, data, blob, file, about}, reject.
/// If parsing fails (unparseable platform URI like wry's ipc://...), allow
/// (must not brick normal toolbar IPC).
pub fn ipc_origin_allowed(uri: &str) -> bool {
    match url::Url::parse(uri) {
        Ok(parsed) => {
            let scheme = parsed.scheme().to_ascii_lowercase();
            match scheme.as_str() {
                "javascript" | "data" | "blob" | "file" | "about" => false,
                _ => true,
            }
        }
        Err(_) => {
            // Unparseable platform URI — wry custom schemes etc. — allow
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_normalize_empty() {
        assert_eq!(normalize_url(""), "https://duckduckgo.com");
        assert_eq!(normalize_url("   "), "https://duckduckgo.com");
    }

    #[test]
    fn test_normalize_with_scheme() {
        assert_eq!(normalize_url("https://example.com"), "https://example.com");
        assert_eq!(normalize_url("http://example.com/path"), "http://example.com/path");
        // Per spec, anything containing "://" is returned as-is (validation later via is_navigable)
        assert_eq!(normalize_url("file:///etc/passwd"), "file:///etc/passwd");
        assert_eq!(normalize_url("ftp://example.com"), "ftp://example.com");
        assert_eq!(normalize_url("http://localhost:3000"), "http://localhost:3000");
        // javascript: does NOT contain "://", so goes to search fallback (blocked later by is_navigable)
        assert_eq!(
            normalize_url("javascript:alert(1)"),
            "https://duckduckgo.com/?q=javascript%3Aalert%281%29"
        );
    }

    #[test]
    fn test_normalize_dot_or_localhost() {
        assert_eq!(normalize_url("example.com"), "https://example.com");
        assert_eq!(normalize_url("sub.example.com/path"), "https://sub.example.com/path");
        assert_eq!(normalize_url("localhost"), "https://localhost");
        assert_eq!(normalize_url("localhost:3000"), "https://localhost:3000");
        // no space, has dot
        assert_eq!(normalize_url("127.0.0.1"), "https://127.0.0.1");
    }

    #[test]
    fn test_normalize_search() {
        assert_eq!(
            normalize_url("hello world"),
            "https://duckduckgo.com/?q=hello+world"
        );
        assert_eq!(
            normalize_url("cats  are cute"),
            "https://duckduckgo.com/?q=cats+are+cute"
        );
        // single word no dot, no localhost => search
        assert_eq!(
            normalize_url("catisen"),
            "https://duckduckgo.com/?q=catisen"
        );
    }

    #[test]
    fn test_is_navigable_ok() {
        assert!(is_navigable("https://example.com"));
        assert!(is_navigable("http://example.com"));
        assert!(is_navigable("https://example.com/path?q=1"));
        assert!(is_navigable("https://duckduckgo.com/?q=test"));
        // case-insensitive scheme via parser lower-casing
        assert!(is_navigable("HTTPS://example.com"));
        assert!(is_navigable("Http://example.com"));
    }

    #[test]
    fn test_is_navigable_reject_schemes() {
        assert!(!is_navigable("javascript:alert(1)"));
        assert!(!is_navigable("JavaScript:alert(1)"));
        assert!(!is_navigable("JAVASCRIPT:alert(1)"));
        assert!(!is_navigable("data:text/html,<h1>hi</h1>"));
        assert!(!is_navigable("blob:https://example.com/123"));
        assert!(!is_navigable("file:///etc/passwd"));
        assert!(!is_navigable("file:///C:/Windows/win.ini"));
        assert!(!is_navigable("about:blank"));
        assert!(!is_navigable("ftp://example.com"));
    }

    #[test]
    fn test_is_navigable_reject_creds() {
        assert!(!is_navigable("https://user:pass@example.com"));
        assert!(!is_navigable("https://user@example.com"));
        assert!(!is_navigable("http://admin:1234@example.com/path"));
    }

    #[test]
    fn test_is_navigable_reject_missing_host() {
        assert!(!is_navigable("https://"));
        assert!(!is_navigable("https:///path"));
        assert!(!is_navigable(""));
        assert!(!is_navigable("not a url"));
        assert!(!is_navigable("https:"));
    }

    #[test]
    fn test_sanitize_basic() {
        let base = Path::new("downloads");
        let out = sanitize_download_path("file.txt", base).unwrap();
        assert_eq!(out, PathBuf::from("downloads/file.txt"));
    }

    #[test]
    fn test_sanitize_traversal() {
        let base = Path::new("downloads");
        // Path::new(...).file_name() strips dirs, so these should still be confined to basename
        let out = sanitize_download_path("/etc/passwd", base).unwrap();
        assert_eq!(out, PathBuf::from("downloads/passwd"));
        let out2 = sanitize_download_path("../../evil.txt", base).unwrap();
        assert_eq!(out2, PathBuf::from("downloads/evil.txt"));
        let out3 = sanitize_download_path("a/b/c.txt", base).unwrap();
        assert_eq!(out3, PathBuf::from("downloads/c.txt"));
    }

    #[test]
    fn test_sanitize_reject_empty_dot() {
        let base = Path::new("downloads");
        assert!(sanitize_download_path("", base).is_err());
        assert!(sanitize_download_path(".", base).is_err());
        assert!(sanitize_download_path("..", base).is_err());
        // file_name of "/" is None -> err
        assert!(sanitize_download_path("/", base).is_err());
    }

    #[test]
    fn test_sanitize_reject_separators() {
        let base = Path::new("downloads");
        // NUL byte must always be rejected
        let with_nul = "file\0name.txt";
        assert!(sanitize_download_path(with_nul, base).is_err());

        // Backslash handling is platform-dependent:
        // - On Unix, Path::new("a\\b").file_name() == "a\\b" which contains '\' => should be rejected
        // - On Windows, Path::new("a\\b").file_name() == "b" (backslash is separator) => confined, not rejected
        #[cfg(not(windows))]
        {
            assert!(sanitize_download_path("a\\b", base).is_err());
        }
        #[cfg(windows)]
        {
            let out = sanitize_download_path("a\\b", base).unwrap();
            assert_eq!(out, PathBuf::from("downloads/b"));
            assert!(out.starts_with(base));
        }

        // Forward slash is stripped by file_name on both platforms, so "a/b" => "b" (allowed, confined)
        // The spec's '/' rejection applies to basename itself containing '/', which file_name never does.
        // So we test that "a/b/c.txt" is confined, not err.
        let out = sanitize_download_path("a/b/c.txt", base).unwrap();
        assert_eq!(out, PathBuf::from("downloads/c.txt"));
    }

    #[test]
    fn test_sanitize_starts_with_base() {
        let base = Path::new("/tmp/downloads_test_base");
        let out = sanitize_download_path("hello.txt", base).unwrap();
        assert!(out.starts_with(base));
    }

    #[test]
    fn test_downloads_dir() {
        assert_eq!(downloads_dir(), PathBuf::from("downloads"));
    }

    #[test]
    fn test_ipc_origin_allowed_blocked() {
        assert!(!ipc_origin_allowed("javascript:alert(1)"));
        assert!(!ipc_origin_allowed("data:text/html,hi"));
        assert!(!ipc_origin_allowed("blob:https://example.com/x"));
        assert!(!ipc_origin_allowed("file:///etc/passwd"));
        assert!(!ipc_origin_allowed("about:blank"));
        // case variations — url crate lower-cases scheme
        assert!(!ipc_origin_allowed("JavaScript:alert(1)"));
        assert!(!ipc_origin_allowed("DATA:text/html,hi"));
    }

    #[test]
    fn test_ipc_origin_allowed_ok() {
        assert!(ipc_origin_allowed("https://example.com/"));
        assert!(ipc_origin_allowed("http://example.com/"));
        assert!(ipc_origin_allowed("https://duckduckgo.com/?q=test"));
    }

    #[test]
    fn test_ipc_origin_allowed_unparseable_returns_true() {
        // wry uses custom scheme like ipc://... or similar; some might not parse?
        // Actually ipc:// would parse (scheme ipc), which is not in blocklist, so true.
        // Unparseable like "not a url at all" should return true per spec.
        assert!(ipc_origin_allowed("not a url"));
        assert!(ipc_origin_allowed(""));
        // Custom wry internal uris — they should be allowed if not in blocklist
        assert!(ipc_origin_allowed("ipc://localhost"));
        assert!(ipc_origin_allowed("https+ipc://something"));
    }
}
