// Catisen Bookmark & History Engine
// §9.6: move storage out of CWD into OS user-data dir, atomic write, bound ~5000 visits.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::Utc;

const MAX_VISITS: usize = 5000;
const HISTORY_FILE: &str = ".catisen_history.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visit {
    pub url: String,
    pub title: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HistoryEngine {
    pub visits: Vec<Visit>,
    pub bookmarks: Vec<Bookmark>,
}

impl HistoryEngine {
    /// Bootstraps the engine, attempting to load existing history from disk (OS data dir)
    pub fn new() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Resolve OS user-data dir using only std env (no new deps)
    /// Windows: %APPDATA%/catisen or %LOCALAPPDATA%/catisen
    /// Linux: $XDG_DATA_HOME/catisen else $HOME/.local/share/catisen
    /// macOS: $HOME/Library/Application Support/catisen
    fn data_dir() -> PathBuf {
        let os = std::env::consts::OS;
        if os == "windows" {
            if let Ok(appdata) = std::env::var("APPDATA") {
                if !appdata.is_empty() {
                    return PathBuf::from(appdata).join("catisen");
                }
            }
            if let Ok(local) = std::env::var("LOCALAPPDATA") {
                if !local.is_empty() {
                    return PathBuf::from(local).join("catisen");
                }
            }
            // Fallback: HOME\.local\share\catisen or current dir
            if let Ok(home) = std::env::var("USERPROFILE") {
                return PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("catisen");
            }
            // Last resort: current dir (preserves old behaviour but in data_dir fn)
            PathBuf::from("catisen_data")
        } else if os == "macos" {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("catisen");
            }
            PathBuf::from("catisen_data")
        } else {
            // Linux and others
            if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
                if !xdg.is_empty() {
                    return PathBuf::from(xdg).join("catisen");
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("catisen");
            }
            PathBuf::from("catisen_data")
        }
    }

    fn file_path() -> PathBuf {
        Self::file_path_in(&Self::data_dir())
    }

    fn file_path_in(base: &Path) -> PathBuf {
        base.join(HISTORY_FILE)
    }

    /// Load from OS data dir
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from(&Self::data_dir())
    }

    /// Load from specific dir (test helper, parameterised base)
    pub fn load_from(base: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::file_path_in(base);
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(&path)?;
        let engine: HistoryEngine = serde_json::from_str(&data)?;
        Ok(engine)
    }

    /// Save to OS data dir atomically (write .tmp then rename), bound visits
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to(&Self::data_dir())
    }

    /// Save to specific dir atomically (test helper)
    pub fn save_to(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(base)?;
        let path = Self::file_path_in(base);
        let tmp_path = path.with_extension("tmp");

        // Bound visits to last MAX_VISITS
        let bounded = if self.visits.len() > MAX_VISITS {
            let mut cloned = self.clone();
            let excess = cloned.visits.len() - MAX_VISITS;
            cloned.visits.drain(0..excess);
            cloned
        } else {
            // Clone to avoid mutating self, but we need bounded for serialization
            // If not over limit, just use self
            // To keep logic simple, we handle bounding via a temporary copy for write
            // but we can't mutate self here, so we create a bounded view
            let mut tmp = HistoryEngine {
                visits: self.visits.clone(),
                bookmarks: self.bookmarks.clone(),
            };
            if tmp.visits.len() > MAX_VISITS {
                let excess = tmp.visits.len() - MAX_VISITS;
                tmp.visits.drain(0..excess);
            }
            tmp
        };

        let json = serde_json::to_string_pretty(&bounded)?;
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, &path)?;
        Ok(())
    }

    /// Records a new URL visit, skipping internal protocol schemes
    pub fn record_visit(&mut self, url: &str, title: Option<&str>) {
        if url.starts_with("chrome:") || url.starts_with("about:") {
            return;
        }

        let visit = Visit {
            url: url.to_string(),
            title: title.map(|t| t.to_string()),
            timestamp: Utc::now().timestamp(),
        };

        self.visits.push(visit);

        // Bound to last MAX_VISITS in memory too (not just on disk)
        if self.visits.len() > MAX_VISITS {
            let excess = self.visits.len() - MAX_VISITS;
            self.visits.drain(0..excess);
        }

        if let Err(e) = self.save() {
            eprintln!("[History] Failed to save: {}", e);
        }
    }

    /// Instantiates a bookmark, ignoring duplicates
    pub fn add_bookmark(&mut self, url: &str, title: &str) {
        if self.bookmarks.iter().any(|b| b.url == url) {
            return;
        }
        self.bookmarks.push(Bookmark {
            url: url.to_string(),
            title: title.to_string(),
        });

        if let Err(e) = self.save() {
            eprintln!("[History] Failed to save bookmark: {}", e);
        }
    }

    /// Retrieves all bookmarks (kept for API compat, but marked allow(dead_code) if unused)
    #[allow(dead_code)]
    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_history_lifecycle() {
        let mut engine = HistoryEngine::default();
        engine.record_visit("https://duckduckgo.onion", Some("DuckDuckGo"));
        // record_visit tries to save to data_dir, but we don't care for this test
        // Just check in-memory
        assert_eq!(engine.visits.len(), 1);

        engine.bookmarks.clear();
        engine.add_bookmark("https://secmail.onion", "Secure Mail");
        engine.add_bookmark("https://secmail.onion", "Secure Mail Duplicate");
        assert_eq!(engine.bookmarks.len(), 1); // Avoids duplicates
    }

    #[test]
    fn test_history_write_load_temp_dir() {
        let base = std::env::temp_dir().join(format!("catisen_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let mut engine = HistoryEngine::default();
        for i in 0..10 {
            engine.visits.push(Visit {
                url: format!("https://example.com/{}", i),
                title: None,
                timestamp: 0,
            });
        }
        engine.bookmarks.push(Bookmark {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
        });

        engine.save_to(&base).unwrap();
        let loaded = HistoryEngine::load_from(&base).unwrap();
        assert_eq!(loaded.visits.len(), 10);
        assert_eq!(loaded.bookmarks.len(), 1);
        assert_eq!(loaded.visits[0].url, "https://example.com/0");

        // Cleanup
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_history_bound() {
        let base = std::env::temp_dir().join(format!("catisen_test_bound_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let mut engine = HistoryEngine::default();
        // Push 5100 visits
        for i in 0..5100 {
            engine.visits.push(Visit {
                url: format!("https://example.com/{}", i),
                title: None,
                timestamp: i as i64,
            });
        }
        // Simulate record_visit bounding logic (we push then bound in method, but here we test save_to bounding)
        engine.save_to(&base).unwrap();
        let loaded = HistoryEngine::load_from(&base).unwrap();
        assert_eq!(loaded.visits.len(), MAX_VISITS);
        // Should keep last 5000, so first kept should be 100
        assert_eq!(loaded.visits[0].url, "https://example.com/100");

        let _ = fs::remove_dir_all(&base);
    }
}
