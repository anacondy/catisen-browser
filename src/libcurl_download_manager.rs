// Catisen Download Manager — Multi-threaded, Pause/Resume with Tor/proxy support.
//
// Previously: `start_download()` had no proxy parameter — all downloads went
// directly over clearnet even when the user had Tor enabled.
//
// Now wired: `start_download()` accepts `tor_proxy: Option<String>` which is
// provided by the browser event loop from `privacy::tor_manager::resolve_tor_proxy()`.
// When a proxy address is supplied, libcurl routes the download through it using
// `ProxyType::Socks5Hostname` — critically **not** `Socks5` — so DNS resolution
// also happens through the proxy and does not leak the target hostname to the
// local network (DNS leak prevention).

use curl::easy::{Easy, ProxyType};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct DownloadManager {
    pub downloads: Arc<Mutex<Vec<Download>>>,
}

#[derive(Debug, Clone)]
pub struct Download {
    pub id: usize,
    pub url: String,
    pub path: String,
    /// Tor/proxy address used for this download (e.g. "socks5h://127.0.0.1:9150").
    /// `None` means clearnet.  Set once at `start_download()` time and preserved
    /// on `resume_download()` so the same privacy posture is maintained throughout.
    pub tor_proxy: Option<String>,
    pub progress_percent: Arc<Mutex<f32>>,
    pub total_bytes: Arc<Mutex<f64>>,
    pub downloaded_bytes: Arc<Mutex<f64>>,
    pub is_paused: Arc<Mutex<bool>>,
    pub is_completed: Arc<Mutex<bool>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        DownloadManager {
            downloads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Spawns a dedicated OS thread for a high-speed download with progress tracking.
    ///
    /// # Arguments
    /// * `url`       — The resource URL to fetch.
    /// * `path`      — Local filesystem path to write the bytes into.
    /// * `tor_proxy` — Optional SOCKS5h proxy URL (from `tor_manager::resolve_tor_proxy()`).
    ///                 When `Some`, libcurl uses `ProxyType::Socks5Hostname` so DNS resolution
    ///                 is also proxied — preventing hostname leaks to the local ISP.
    pub fn start_download(&mut self, url: &str, path: &str, tor_proxy: Option<String>) {
        let mut downloads_lock = self.downloads.lock().unwrap();
        let new_id = downloads_lock.len();

        let path_clone     = path.to_string();
        let tor_proxy_copy = tor_proxy.clone();

        let progress_percent  = Arc::new(Mutex::new(0.0_f32));
        let total_bytes       = Arc::new(Mutex::new(0.0_f64));
        let downloaded_bytes  = Arc::new(Mutex::new(0.0_f64));
        let is_paused         = Arc::new(Mutex::new(false));
        let is_completed      = Arc::new(Mutex::new(false));

        let download = Download {
            id: new_id,
            url: url.to_string(),
            path: path.to_string(),
            // Store the proxy alongside the download record so resume_download()
            // can re-use the same routing without re-querying tor_manager.
            tor_proxy,
            progress_percent: progress_percent.clone(),
            total_bytes: total_bytes.clone(),
            downloaded_bytes: downloaded_bytes.clone(),
            is_paused: is_paused.clone(),
            is_completed: is_completed.clone(),
        };
        downloads_lock.push(download.clone());

        let url_clone = url.to_string();

        thread::spawn(move || {
            let mut easy = Easy::new();
            easy.url(&url_clone).unwrap();
            easy.follow_location(true).unwrap();

            // ── Tor / proxy configuration ─────────────────────────────────────
            // When a SOCKS5 proxy address is available (Tor is active), configure
            // libcurl to route the entire download through it.
            //
            // ProxyType::Socks5Hostname (CURLPROXY_SOCKS5_HOSTNAME) is used
            // instead of plain Socks5 because it resolves the hostname inside
            // the proxy — preventing a DNS leak where the local network could
            // see which domain is being downloaded even if the bytes are hidden.
            if let Some(ref proxy_addr) = tor_proxy_copy {
                if let Err(e) = easy.proxy(proxy_addr) {
                    eprintln!("[Download] Warning: could not set proxy {}: {}", proxy_addr, e);
                } else if let Err(e) = easy.proxy_type(ProxyType::Socks5Hostname) {
                    eprintln!("[Download] Warning: could not set proxy type: {}", e);
                } else {
                    println!("🧅 DOWNLOAD via Tor ({}): {}", proxy_addr, url_clone);
                }
            } else {
                println!("🚀 STARTING direct download: {}", url_clone);
            }

            // ── Real-time progress tracking ───────────────────────────────────
            easy.progress(true).unwrap();
            let p_pause   = is_paused.clone();
            let p_percent = progress_percent.clone();
            let p_total   = total_bytes.clone();
            let p_down    = downloaded_bytes.clone();

            easy.progress_function(move |total_dl, downloaded, _, _| {
                // If the user hit pause in the UI, abort libcurl
                if *p_pause.lock().unwrap() {
                    return false;
                }
                if total_dl > 0.0 {
                    *p_percent.lock().unwrap() = (downloaded / total_dl) as f32;
                    *p_total.lock().unwrap()   = total_dl;
                    *p_down.lock().unwrap()    = downloaded;
                }
                true
            }).unwrap();

            // ── File open: resume if partially downloaded ──────────────────────
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path_clone)
                .unwrap();

            let starting_size = file.metadata().unwrap().len();
            if starting_size > 0 {
                println!("🚀 RESUMING download for {} at byte {}", path_clone, starting_size);
                easy.resume_from(starting_size).unwrap();
            }

            let write_path_clone = path_clone.clone();
            easy.write_function(move |data| {
                if *is_paused.lock().unwrap() {
                    println!("⏸️ PAUSED download at {}", write_path_clone);
                    return Ok(0);
                }
                file.write_all(data).unwrap();
                Ok(data.len())
            }).unwrap();

            match easy.perform() {
                Ok(_) => {
                    println!("✅ COMPLETED download to {}", path_clone);
                    *is_completed.lock().unwrap()   = true;
                    *progress_percent.lock().unwrap() = 1.0;
                }
                Err(e) => {
                    if !e.is_write_error() && !e.is_aborted_by_callback() {
                        println!("❌ ERROR downloading: {}", e);
                    } else if e.is_aborted_by_callback() {
                        println!("⏸️ PAUSE ACKNOWLEDGED for {}", path_clone);
                    }
                }
            }
        });
    }

    /// Sends a signal to immediately halt the download thread.
    pub fn pause_download(&mut self, id: usize) {
        if let Some(dl) = self.downloads.lock().unwrap().get_mut(id) {
            *dl.is_paused.lock().unwrap() = true;
        }
    }

    /// Resumes a paused download, preserving the original Tor/proxy routing.
    ///
    /// The `tor_proxy` stored on the `Download` struct is reused so that a
    /// download started over Tor cannot silently switch to clearnet on resume.
    pub fn resume_download(&mut self, id: usize) {
        let (url_clone, path_clone, tor_proxy_clone) = {
            let mut lock = self.downloads.lock().unwrap();
            let Some(dl) = lock.get_mut(id) else { return };
            *dl.is_paused.lock().unwrap() = false;
            (dl.url.clone(), dl.path.clone(), dl.tor_proxy.clone())
        };

        if !url_clone.is_empty() {
            // Re-start with the same proxy that was used originally.
            self.start_download(&url_clone, &path_clone, tor_proxy_clone);
        }
    }

    /// Returns the current progress [0.0, 1.0] of a specific download.
    pub fn get_progress(&self, id: usize) -> Option<f32> {
        let lock = self.downloads.lock().unwrap();
        lock.get(id).map(|dl| *dl.progress_percent.lock().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_manager_lifecycle() {
        let mut dm = DownloadManager::new();
        assert_eq!(dm.downloads.lock().unwrap().len(), 0);

        // Pass None for tor_proxy — clearnet in test environment
        let url  = "https://example.com/test.bin";
        let path = "test.bin";
        dm.start_download(url, path, None);
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert_eq!(dm.downloads.lock().unwrap().len(), 1);

        let id = 0;
        dm.pause_download(id);
        {
            let lock = dm.downloads.lock().unwrap();
            assert!(*lock[id].is_paused.lock().unwrap());
            // Verify tor_proxy is stored (None in this test case)
            assert!(lock[id].tor_proxy.is_none());
        }

        dm.resume_download(id);
    }

    #[test]
    fn test_download_stores_proxy() {
        let mut dm = DownloadManager::new();
        dm.start_download(
            "https://example.com/file.bin",
            "file.bin",
            Some("socks5h://127.0.0.1:9150".to_string()),
        );
        std::thread::sleep(std::time::Duration::from_millis(50));
        let lock = dm.downloads.lock().unwrap();
        assert_eq!(
            lock[0].tor_proxy.as_deref(),
            Some("socks5h://127.0.0.1:9150")
        );
    }
}
