// Catisen Bookmark & History Engine
// Uses flat-file JSON serialization to act as a lightweight local database
// for user visited URLs and saved tabs.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::Utc;

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
    /// Bootstraps the engine, attempting to load existing history from disk
    pub fn new() -> Self {
        Self::load().unwrap_or_default()
    }

    fn file_path() -> String {
        // Use a hidden local file for sandboxed history storage
        ".catisen_history.json".to_string()
    }

    /// Deserializes the local JSON database into memory
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::file_path();
        if !Path::new(&path).exists() {
            return Ok(Self::default());
        }
        let data = fs::read_to_string(path)?;
        let engine: HistoryEngine = serde_json::from_str(&data)?;
        Ok(engine)
    }

    /// Serializes and writes memory back to the disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(Self::file_path(), json)?;
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
        
        // Asynchronously or synchronously save (here we do sync for simplicity in MVP)
        if let Err(e) = self.save() {
            println!("⚠️ Failed to save history cache: {}", e);
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
            println!("⚠️ Failed to save bookmark: {}", e);
        }
    }

    /// Retrieves all bookmarks
    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_lifecycle() {
        let mut engine = HistoryEngine::default();
        engine.record_visit("https://duckduckgo.onion", Some("DuckDuckGo"));
        assert_eq!(engine.visits.len(), 1);
        
        engine.add_bookmark("https://secmail.onion", "Secure Mail");
        engine.add_bookmark("https://secmail.onion", "Secure Mail Duplicate");
        assert_eq!(engine.bookmarks.len(), 1); // Avoids duplicates
    }
}