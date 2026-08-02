// Catisen Bookmark & History Engine
//
// History is sensitive user data. It is kept in memory for the current session,
// but persistent writes are opt-out, bounded, owner-readable on Unix, exclude
// onion visits, and are performed off the UI event-loop thread for navigation.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

const MAX_VISITS: usize = 5000;
const HISTORY_FILE: &str = ".catisen_history.json";

fn default_save_history() -> bool {
    true
}

static WRITE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static WRITE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryEngine {
    pub visits: Vec<Visit>,
    pub bookmarks: Vec<Bookmark>,
    /// When false, visits remain available in memory for the current session
    /// but are not written to persistent storage. Bookmarks remain persistent.
    #[serde(default = "default_save_history")]
    pub save_history: bool,
}

impl Default for HistoryEngine {
    fn default() -> Self {
        Self {
            visits: Vec::new(),
            bookmarks: Vec::new(),
            save_history: true,
        }
    }
}

impl HistoryEngine {
    pub fn new() -> Self {
        Self::load().unwrap_or_default()
    }

    fn data_dir() -> PathBuf {
        if cfg!(target_os = "windows") {
            if let Ok(appdata) = std::env::var("APPDATA") {
                if !appdata.trim().is_empty() {
                    return PathBuf::from(appdata).join("catisen");
                }
            }
            if let Ok(local) = std::env::var("LOCALAPPDATA") {
                if !local.trim().is_empty() {
                    return PathBuf::from(local).join("catisen");
                }
            }
            if let Ok(home) = std::env::var("USERPROFILE") {
                return PathBuf::from(home).join(".local").join("share").join("catisen");
            }
        } else if cfg!(target_os = "macos") {
            if let Ok(home) = std::env::var("HOME") {
                if !home.trim().is_empty() {
                    return PathBuf::from(home)
                        .join("Library")
                        .join("Application Support")
                        .join("catisen");
                }
            }
        } else {
            if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
                if !xdg.trim().is_empty() {
                    return PathBuf::from(xdg).join("catisen");
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                if !home.trim().is_empty() {
                    return PathBuf::from(home).join(".local").join("share").join("catisen");
                }
            }
        }
        PathBuf::from("catisen_data")
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
        let data = fs::read_to_string(path)?;
        let mut engine: HistoryEngine = serde_json::from_str(&data)?;
        let before = engine.visits.len();
        engine.visits.retain(|visit| !is_onion_url(&visit.url));
        if !engine.save_history {
            engine.visits.clear();
        }
        if engine.visits.len() != before {
            // Best-effort scrub of onion entries already present in an older
            // file. Loading remains successful even if the cleanup write fails.
            let _ = engine.save_to(base);
        }
        Ok(engine)
    }

    /// Enable or disable persistent visit storage. Disabling also rewrites the
    /// file immediately so visits from a previous setting are not left behind.
    pub fn set_save_history(&mut self, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.save_history = enabled;
        self.save()
    }

    fn persisted_snapshot(&self) -> Self {
        let mut visits: Vec<Visit> = if self.save_history {
            self.visits
                .iter()
                .filter(|visit| !is_onion_url(&visit.url))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };
        if visits.len() > MAX_VISITS {
            let excess = visits.len() - MAX_VISITS;
            visits.drain(0..excess);
        }
        Self {
            visits,
            bookmarks: self.bookmarks.clone(),
            save_history: self.save_history,
        }
    }

    fn to_persisted_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(&self.persisted_snapshot())?)
    }

    fn next_write_sequence() -> u64 {
        WRITE_SEQUENCE.fetch_add(1, Ordering::AcqRel)
    }

    fn write_json_to_disk(
        base: &Path,
        json: &str,
        sequence: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // A single process-wide lock prevents concurrent snapshots from
        // interleaving. The sequence check prevents an older detached writer
        // from overwriting a newer snapshot that was queued later.
        let lock = WRITE_LOCK.get_or_init(|| Mutex::new(()));
        let _guard = lock.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "history writer lock poisoned")
        })?;
        if sequence + 1 < WRITE_SEQUENCE.load(Ordering::Acquire) {
            return Ok(());
        }

        fs::create_dir_all(base)?;
        let path = Self::file_path_in(base);
        let temp_path = base.join(format!(
            ".catisen_history.{}.{}.tmp",
            std::process::id(),
            sequence
        ));
        fs::write(&temp_path, json)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&temp_path)?.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&temp_path, permissions)?;
        }

        // Unix rename replaces atomically. Windows does not replace an existing
        // file with rename, so remove the old destination while holding the
        // process lock, then rename the completed temporary file.
        #[cfg(windows)]
        if path.exists() {
            fs::remove_file(&path)?;
        }
        fs::rename(&temp_path, &path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&path)?.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&path, permissions)?;
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to(&Self::data_dir())
    }

    pub fn save_to(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_persisted_json()?;
        let sequence = Self::next_write_sequence();
        Self::write_json_to_disk(base, &json, sequence)
    }

    /// Record in-memory history and enqueue a persistent snapshot without
    /// blocking the tao/wry event loop on filesystem I/O.
    pub fn record_visit(&mut self, url: &str, title: Option<&str>) {
        let lower = url.to_ascii_lowercase();
        if lower.starts_with("chrome:") || lower.starts_with("about:") {
            return;
        }

        self.visits.push(Visit {
            url: url.to_string(),
            title: title.map(str::to_string),
            timestamp: Utc::now().timestamp(),
        });
        if self.visits.len() > MAX_VISITS {
            let excess = self.visits.len() - MAX_VISITS;
            self.visits.drain(0..excess);
        }
        if !self.save_history {
            return;
        }

        let Ok(json) = self.to_persisted_json() else {
            eprintln!("[History] Failed to serialize visit snapshot");
            return;
        };
        let base = Self::data_dir();
        let sequence = Self::next_write_sequence();
        std::thread::spawn(move || {
            if let Err(error) = Self::write_json_to_disk(&base, &json, sequence) {
                eprintln!("[History] Failed to save visit snapshot: {error}");
            }
        });
    }

    pub fn add_bookmark(&mut self, url: &str, title: &str) {
        if self.bookmarks.iter().any(|bookmark| bookmark.url == url) {
            return;
        }
        self.bookmarks.push(Bookmark {
            url: url.to_string(),
            title: title.to_string(),
        });
        if let Err(error) = self.save() {
            eprintln!("[History] Failed to save bookmark: {error}");
        }
    }

    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}

fn is_onion_url(url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(url) else {
        return false;
    };
    parsed
        .host_str()
        .map(|host| {
            let host = host.to_ascii_lowercase();
            host == "onion" || host.ends_with(".onion")
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_base(label: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!(
            "catisen_history_test_{}_{}_{}",
            label,
            std::process::id(),
            WRITE_SEQUENCE.load(Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        base
    }

    #[test]
    fn onion_visits_are_kept_in_memory_but_not_persisted() {
        let mut engine = HistoryEngine::default();
        engine.visits.push(Visit {
            url: "https://example.onion/private".to_string(),
            title: None,
            timestamp: 0,
        });
        engine.visits.push(Visit {
            url: "https://example.com".to_string(),
            title: None,
            timestamp: 0,
        });
        let json = engine.to_persisted_json().unwrap();
        assert!(!json.contains("example.onion"));
        assert!(json.contains("example.com"));
    }

    #[test]
    fn disabled_history_persists_bookmarks_but_no_visits() {
        let mut engine = HistoryEngine::default();
        engine.save_history = false;
        engine.visits.push(Visit {
            url: "https://example.com".to_string(),
            title: None,
            timestamp: 0,
        });
        engine.bookmarks.push(Bookmark {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
        });
        let persisted: HistoryEngine = serde_json::from_str(&engine.to_persisted_json().unwrap()).unwrap();
        assert!(persisted.visits.is_empty());
        assert_eq!(persisted.bookmarks.len(), 1);
        assert!(!persisted.save_history);
    }

    #[test]
    fn save_and_load_bound_history_in_temp_dir() {
        let base = temp_base("bound");
        let mut engine = HistoryEngine::default();
        for index in 0..5100 {
            engine.visits.push(Visit {
                url: format!("https://example.com/{index}"),
                title: None,
                timestamp: index,
            });
        }
        // Bound the in-memory list in the same way record_visit does.
        let excess = engine.visits.len() - MAX_VISITS;
        engine.visits.drain(0..excess);
        engine.save_to(&base).unwrap();
        let loaded = HistoryEngine::load_from(&base).unwrap();
        assert_eq!(loaded.visits.len(), MAX_VISITS);
        assert_eq!(loaded.visits[0].url, "https://example.com/100");
        let _ = fs::remove_dir_all(base);
    }
}
