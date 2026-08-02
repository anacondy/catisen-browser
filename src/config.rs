use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintLevel {
    Off,
    Standard,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CatisenConfig {
    pub tor_proxy_url: String,
    pub use_tor_by_default: bool,
    pub fingerprint_level: FingerprintLevel,
    pub browser_profile: String,
    pub geo_location: String,
    pub default_view_mode: String,
    pub request_timeout_secs: u64,
    pub request_retries: u8,
    pub ublock_rules_path: String,
    pub theme: String,
    pub home_page: String,
    pub target_fps: u32,
    pub fps_telemetry: bool,
}

impl Default for CatisenConfig {
    fn default() -> Self {
        Self {
            tor_proxy_url: "socks5h://127.0.0.1:9150".into(),
            use_tor_by_default: false,
            fingerprint_level: FingerprintLevel::Strict,
            browser_profile: "auto_desktop".to_string(),
            geo_location: "disabled".to_string(),
            default_view_mode: "visual".to_string(),
            request_timeout_secs: 30,
            request_retries: 1,
            ublock_rules_path: "easylist.txt".to_string(),
            theme: "dark".to_string(),
            home_page: "https://www.mozilla.org".to_string(),
            target_fps: 120,
            fps_telemetry: false,
        }
    }
}

impl CatisenConfig {
    pub fn config_path() -> PathBuf {
        Path::new("config.toml").to_path_buf()
    }

    pub fn load_or_create() -> Result<Self, ConfigError> {
        let path = Self::config_path();
        if !path.exists() {
            let cfg = Self::default();
            cfg.save_to_path(&path)?;
            return Ok(cfg);
        }

        let raw = std::fs::read_to_string(&path)?;
        let mut cfg: Self = toml::from_str(&raw)?;
        cfg.normalize();
        Ok(cfg)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to_path(&Self::config_path())
    }

    pub fn save_to_path(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let mut normalized = self.clone();
        normalized.normalize();
        let toml_str = toml::to_string_pretty(&normalized)?;
        std::fs::write(path, toml_str)?;
        Ok(())
    }

    fn normalize(&mut self) {
        self.request_timeout_secs = self.request_timeout_secs.clamp(5, 120);
        self.request_retries = self.request_retries.min(5);
        self.target_fps = self.target_fps.clamp(30, 240);

        if self.theme.trim().is_empty() {
            self.theme = "dark".to_string();
        }

        if self.home_page.trim().is_empty() {
            self.home_page = "https://www.mozilla.org".to_string();
        }

        if self.ublock_rules_path.trim().is_empty() {
            self.ublock_rules_path = "easylist.txt".to_string();
        }

        if self.browser_profile.trim().is_empty() {
            self.browser_profile = "auto_desktop".to_string();
        }

        if self.geo_location.trim().is_empty() {
            self.geo_location = "disabled".to_string();
        }

        if self.default_view_mode.trim().is_empty() {
            self.default_view_mode = "visual".to_string();
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read or write config file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid config format: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    Serialize(#[from] toml::ser::Error),
}
