// Catisen Ad & Tracker Blocker
// Parses raw .txt filter lists (EasyList format, same as uBlock Origin) and
// drops network requests that match.
//
// The bundled list is embedded at compile time via include_str! so the blocker
// works out of the box without any external download.  Users can also supply
// their own easylist.txt alongside the binary to override it.

use adblock::engine::Engine;
use adblock::lists::ParseOptions;
use adblock::request::Request as AdRequest;
use std::sync::{Mutex, OnceLock};

// Bundled filter list (trimmed EasyList) — embedded at compile time.
const BUNDLED_LIST: &str = include_str!("../assets/easylist.txt");

pub static GLOBAL_ADBLOCKER: OnceLock<Mutex<AdBlocker>> = OnceLock::new();

/// Get-or-create the global ad blocker instance.
pub fn get_adblocker() -> &'static Mutex<AdBlocker> {
    GLOBAL_ADBLOCKER.get_or_init(|| {
        let mut adb = AdBlocker::new();
        adb.load_bundled();
        // If user placed a custom list next to the binary, merge it in.
        if let Ok(rules) = std::fs::read_to_string("easylist.txt") {
            adb.merge_rules(&rules);
        } else if let Ok(rules) = std::fs::read_to_string("assets/easylist.txt") {
            // Already loaded as bundled, but refresh in case user edited it.
            adb.merge_rules(&rules);
        }
        Mutex::new(adb)
    })
}

// ─────────────────────────────────────────────────────────────────────────────

pub struct AdBlocker {
    engine: Option<Engine>,
    pub is_enabled: bool,
    rules: Vec<String>,
}

impl AdBlocker {
    pub fn new() -> Self {
        AdBlocker {
            engine: None,
            is_enabled: true,
            rules: Vec::new(),
        }
    }

    /// Load the compile-time bundled filter list.
    pub fn load_bundled(&mut self) {
        let new_rules: Vec<String> = BUNDLED_LIST
            .lines()
            .map(str::to_owned)
            .collect();
        self.rules.extend(new_rules);
        self.rebuild();
        eprintln!("[Catisen] Ad blocker: {} rules loaded (bundled list)", self.rules.len());
    }

    /// Merge additional rules from a string (EasyList format).
    pub fn merge_rules(&mut self, extra: &str) {
        let before = self.rules.len();
        for line in extra.lines() {
            let l = line.trim().to_owned();
            if !l.is_empty() && !self.rules.contains(&l) {
                self.rules.push(l);
            }
        }
        if self.rules.len() > before {
            self.rebuild();
            eprintln!(
                "[Catisen] Ad blocker: merged {} extra rules (total {})",
                self.rules.len() - before,
                self.rules.len()
            );
        }
    }

    /// Load from file path (replaces all current rules).
    pub fn load_filter_list(&mut self, file_path: &str) -> std::io::Result<()> {
        let text = std::fs::read_to_string(file_path)?;
        self.rules = text.lines().map(str::to_owned).collect();
        self.rebuild();
        eprintln!("[Catisen] Ad blocker: {} rules loaded from {}", self.rules.len(), file_path);
        Ok(())
    }

    fn rebuild(&mut self) {
        self.engine = Some(Engine::from_rules(&self.rules, ParseOptions::default()));
    }

    /// Check a request with its actual initiator and resource type.
    ///
    /// EasyList rules can be scoped by both type and third-party status. The
    /// caller must therefore provide the page that initiated the request and a
    /// type such as `document`, `script`, `image`, `stylesheet`, `xhr`, or
    /// `media` rather than making every request look like a first-party script.
    pub fn should_block_resource(
        &self,
        url: &str,
        source_url: &str,
        resource_type: &str,
    ) -> bool {
        if !self.is_enabled {
            return false;
        }
        let Some(engine) = &self.engine else {
            return false;
        };
        match AdRequest::new(url, source_url, resource_type) {
            Ok(req) => engine.check_network_request(&req).matched,
            Err(_) => false,
        }
    }

    /// Compatibility wrapper for diagnostics that have no initiator context.
    pub fn should_block_request(&self, url: &str) -> bool {
        self.should_block_resource(url, url, "document")
    }

    /// Toggle the blocker on/off at runtime.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

impl Default for AdBlocker {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn armed() -> AdBlocker {
        let mut b = AdBlocker::new();
        b.load_bundled();
        b
    }

    #[test]
    fn blocks_google_analytics() {
        let b = armed();
        assert!(b.should_block_request("https://www.google-analytics.com/ga.js"));
    }

    #[test]
    fn blocks_doubleclick() {
        let b = armed();
        assert!(b.should_block_request("https://doubleclick.net/ads/pixel.gif"));
    }

    #[test]
    fn allows_wikipedia() {
        let b = armed();
        assert!(!b.should_block_request("https://en.wikipedia.org/wiki/Main_Page"));
    }

    #[test]
    fn disabled_blocker_allows_all() {
        let mut b = armed();
        b.set_enabled(false);
        assert!(!b.should_block_request("https://doubleclick.net/ads.js"));
    }
}
