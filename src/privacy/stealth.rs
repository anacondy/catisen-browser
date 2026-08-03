//! Consistent browser identity profiles used by the document-start privacy script.
//!
//! The older source also contained a second, unused geolocation/fingerprint
//! script builder. The live browser uses browser::chrome::privacy_init_js;
//! keeping two independent implementations made fixes drift, so this module
//! now owns only the profile tuple selection.

use rand::seq::SliceRandom;

/// Pick a self-consistent UA/platform/language/timezone tuple. The tuple is
/// deliberately static and internally coherent; it is not an anonymity
/// guarantee and should not be confused with a renderer sandbox.
pub fn pick_profile() -> (&'static str, &'static str, &'static str, &'static str) {
    const PROFILES: &[(&str, &str, &str, &str)] = &[
        (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            "Win32",
            "en-US",
            "America/New_York",
        ),
        (
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
            "Linux x86_64",
            "en-GB",
            "Europe/Amsterdam",
        ),
        (
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_7_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            "MacIntel",
            "en-US",
            "America/Los_Angeles",
        ),
    ];

    let mut rng = rand::thread_rng();
    PROFILES.choose(&mut rng).copied().unwrap_or(PROFILES[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picked_profiles_are_internally_consistent() {
        for _ in 0..20 {
            let (ua, platform, language, timezone) = pick_profile();
            assert!(!ua.is_empty());
            assert!(!platform.is_empty());
            assert!(!language.is_empty());
            assert!(!timezone.is_empty());
            if platform == "Win32" {
                assert!(ua.contains("Windows"));
            } else if platform == "Linux x86_64" {
                assert!(ua.contains("Linux") || ua.contains("X11"));
            } else if platform == "MacIntel" {
                assert!(ua.contains("Macintosh") || ua.contains("Mac"));
            }
        }
    }
}
