// Catisen Browser - Tab Isolation Logic
// This module handles assigning unique dummy/temporary profiles to individual tabs
// allowing users to log into the same site (e.g. WhatsApp, Twitter) with multiple
// accounts simultaneously without the site cross-referencing their data.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static TAB_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
pub struct IsolatedTabContext {
    pub tab_id: usize,
    pub storage_path: String, // Path to a temporary isolated cookie jar
    pub is_isolated: bool,
}

pub struct TabManager {
    pub is_isolation_enabled: bool,
    pub active_tabs: HashMap<usize, IsolatedTabContext>,
}

impl TabManager {
    pub fn new() -> Self {
        TabManager {
            is_isolation_enabled: false, // Updated via the settings toggle later
            active_tabs: HashMap::new(),
        }
    }

    /// Enable or disable Strict Tab Isolation based on user UI settings
    pub fn toggle_isolation(&mut self, enabled: bool) {
        self.is_isolation_enabled = enabled;
        if enabled {
            println!("🛡️ TAB ISOLATION: Enabled. New tabs will use isolated sandboxes.");
        } else {
            println!("🌐 TAB ISOLATION: Disabled. Sharing standard global cookies.");
        }
    }

    /// Spawns a new tab. If isolation is enabled, it generates a sandboxed profile path.
    pub fn create_new_tab(&mut self) -> IsolatedTabContext {
        let new_id = TAB_COUNTER.fetch_add(1, Ordering::SeqCst);

        // If enabled, point the web engine to a temporary, sandboxed ID path.
        // If disabled, point it to the global "default" profile path.
        let path = if self.is_isolation_enabled {
            format!(".cache/catisen/isolated_profiles/tab_{}", new_id)
        } else {
            String::from(".cache/catisen/global_profile")
        };

        let context = IsolatedTabContext {
            tab_id: new_id,
            storage_path: path.clone(),
            is_isolated: self.is_isolation_enabled,
        };

        self.active_tabs.insert(new_id, context.clone());

        println!("📝 NEW TAB [ID: {}] spawned. Cookie Jar Path: {}", new_id, path);

        context
    }

    /// Cleans up the temporary isolated profile when a tab is closed. The
    /// context records whether isolation was enabled when it was created, so
    /// disabling the setting later cannot accidentally skip cleanup.
    pub fn close_tab(&mut self, tab_id: usize) {
        if let Some(context) = self.active_tabs.remove(&tab_id) {
            if !context.is_isolated {
                return;
            }
            match std::fs::remove_dir_all(&context.storage_path) {
                Ok(()) => println!(
                    "🧹 CLEANUP: Wiped isolated data for Tab {} at {}",
                    tab_id, context.storage_path
                ),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    // The WebView may never have created the profile directory.
                }
                Err(error) => eprintln!(
                    "[Tab isolation] Failed to wipe Tab {} profile {}: {}",
                    tab_id, context.storage_path, error
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation_and_cleanup() {
        let mut manager = TabManager::new();
        manager.toggle_isolation(true);

        let t1 = manager.create_new_tab();
        let t2 = manager.create_new_tab();

        assert_ne!(t1.tab_id, t2.tab_id);
        assert_ne!(t1.storage_path, t2.storage_path);

        assert!(manager.active_tabs.contains_key(&t1.tab_id));
        std::fs::create_dir_all(&t1.storage_path).unwrap();
        assert!(std::path::Path::new(&t1.storage_path).exists());
        manager.close_tab(t1.tab_id);
        assert!(!manager.active_tabs.contains_key(&t1.tab_id));
        assert!(!std::path::Path::new(&t1.storage_path).exists());
    }
}
