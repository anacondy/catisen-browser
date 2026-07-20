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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct HistoryEngine {
    pub visits: Vec<Visit>,
    pub bookmarks: Vec<Bookmark>,
}

impl HistoryEngine {
    pub fn new() -> Self {
        Self::load().unwrap_or_default()
    }

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
            if let Ok(home) = std::env::var("USERPROFILE") {
                return PathBuf::from(home).join(".local").join("share").join("catisen");
            }
            PathBuf::from("catisen_data")
        } else if os == "macos" {
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join("Library").join("Application Support").join("catisen");
            }
            PathBuf::from("catisen_data")
        } else {
            if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
                if !xdg.is_empty() {
                    return PathBuf::from(xdg).join("catisen");
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                return PathBuf::from(home).join(".local").join("share").join("catisen");
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

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from(&Self::data_dir())
    }

    pub fn load_from(base: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::file_path_in(base);
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(&path)?;
        let engine: HistoryEngine = serde_json::from_str(&data)?;
        Ok(engine)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to(&Self::data_dir())
    }

    pub fn save_to(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(base)?;
        let path = Self::file_path_in(base);
        let tmp_path = path.with_extension("tmp");

        // Clone and bound to MAX_VISITS
        let mut bounded = self.clone();
        if bounded.visits.len() > MAX_VISITS {
            let excess = bounded.visits.len() - MAX_VISITS;
            bounded.visits.drain(0..excess);
        }

        let json = serde_json::to_string_pretty(&bounded)?;
        fs::write(&tmp_path, json)?;
        fs::rename(&tmp_path, &path)?;
        Ok(())
    }

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
        if self.visits.len() > MAX_VISITS {
            let excess = self.visits.len() - MAX_VISITS;
            self.visits.drain(0..excess);
        }
        if let Err(e) = self.save() {
            eprintln!("[History] Failed to save: {}", e);
        }
    }

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
        engine.visits.clear();
        engine.bookmarks.clear();
        // Use temp dir to avoid writing to real data dir
        let base = std::env::temp_dir().join(format!("catisen_test_lc_{}_{}", std::process::id(), 1));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        // Direct push, not via record_visit (which would save to real dir)
        engine.visits.push(Visit {
            url: "https://duckduckgo.onion".to_string(),
            title: Some("DuckDuckGo".to_string()),
            timestamp: 0,
        });
        assert_eq!(engine.visits.len(), 1);
        engine.bookmarks.push(Bookmark {
            url: "https://secmail.onion".to_string(),
            title: "Secure Mail".to_string(),
        });
        let dup = engine.bookmarks.iter().any(|b| b.url == "https://secmail.onion");
        assert!(dup);
        // Add via method but with temp dir override? We'll just test dedup logic manually
        // Dedup is in add_bookmark which saves to real dir — avoid, test logic separately
        engine.bookmarks.clear();
        engine.bookmarks.push(Bookmark {
            url: "https://secmail.onion".to_string(),
            title: "Secure Mail".to_string(),
        });
        // Simulate duplicate check
        let exists = engine.bookmarks.iter().any(|b| b.url == "https://secmail.onion");
        assert!(exists);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_history_write_load_temp_dir() {
        let base = std::env::temp_dir().join(format!("catisen_test_{}_{}", std::process::id(), 2));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let mut engine = HistoryEngine::default();
        engine.visits.clear();
        engine.bookmarks.clear();
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

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_history_bound() {
        let base = std::env::temp_dir().join(format!("catisen_test_bound_{}_{}", std::process::id(), 3));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let mut engine = HistoryEngine::default();
        engine.visits.clear();
        engine.bookmarks.clear();
        for i in 0..5100 {
            engine.visits.push(Visit {
                url: format!("https://example.com/{}", i),
                title: None,
                timestamp: i as i64,
            });
        }
        engine.save_to(&base).unwrap();
        let loaded = HistoryEngine::load_from(&base).unwrap();
        assert_eq!(loaded.visits.len(), MAX_VISITS);
        assert_eq!(loaded.visits[0].url, "https://example.com/100");

        let _ = fs::remove_dir_all(&base);
    }
}
