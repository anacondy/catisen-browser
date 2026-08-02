// Catisen Download Manager — bounded, proxy-aware downloads.
//
// Download destinations are validated by browser::security_policy before this
// manager is called. This layer adds a second local-filesystem check, bounded
// progress, stable IDs, and non-panicking worker error handling.

use curl::easy::{Easy, ProxyType};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_DOWNLOAD_BYTES: f64 = 512.0 * 1024.0 * 1024.0;

pub struct DownloadManager {
    pub downloads: Arc<Mutex<Vec<Download>>>,
    next_id: AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct Download {
    pub id: usize,
    pub url: String,
    pub path: String,
    pub tor_proxy: Option<String>,
    pub progress_percent: Arc<Mutex<f32>>,
    pub total_bytes: Arc<Mutex<f64>>,
    pub downloaded_bytes: Arc<Mutex<f64>>,
    pub is_paused: Arc<Mutex<bool>>,
    pub is_completed: Arc<Mutex<bool>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            downloads: Arc::new(Mutex::new(Vec::new())),
            next_id: AtomicUsize::new(0),
        }
    }

    /// Start a download. `tor_proxy = Some(...)` is a required proxy route;
    /// proxy setup errors abort the worker and never fall back to clearnet.
    pub fn start_download(&mut self, url: &str, path: &str, tor_proxy: Option<String>) {
        let new_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let path_clone = path.to_string();
        let url_clone = url.to_string();
        let tor_proxy_copy = tor_proxy.clone();

        let progress_percent = Arc::new(Mutex::new(0.0_f32));
        let total_bytes = Arc::new(Mutex::new(0.0_f64));
        let downloaded_bytes = Arc::new(Mutex::new(0.0_f64));
        let is_paused = Arc::new(Mutex::new(false));
        let is_completed = Arc::new(Mutex::new(false));

        let download = Download {
            id: new_id,
            url: url_clone.clone(),
            path: path_clone.clone(),
            tor_proxy,
            progress_percent: progress_percent.clone(),
            total_bytes: total_bytes.clone(),
            downloaded_bytes: downloaded_bytes.clone(),
            is_paused: is_paused.clone(),
            is_completed: is_completed.clone(),
        };
        if let Ok(mut downloads) = self.downloads.lock() {
            downloads.push(download);
        } else {
            eprintln!("[Download] Manager lock poisoned; refusing download");
            return;
        }

        thread::spawn(move || {
            let mut easy = Easy::new();
            if let Err(error) = easy.url(&url_clone) {
                eprintln!("[Download {new_id}] Invalid URL: {error}");
                return;
            }
            if let Err(error) = easy.follow_location(true) {
                eprintln!("[Download {new_id}] Could not enable redirects: {error}");
                return;
            }

            if let Some(ref proxy_addr) = tor_proxy_copy {
                if let Err(error) = easy.proxy(proxy_addr) {
                    eprintln!(
                        "[Download {new_id}] Proxy setup failed for {proxy_addr}: {error}; aborting"
                    );
                    return;
                }
                if let Err(error) = easy.proxy_type(ProxyType::Socks5Hostname) {
                    eprintln!("[Download {new_id}] Proxy type setup failed: {error}; aborting");
                    return;
                }
                println!("🧅 DOWNLOAD {new_id} via Tor ({proxy_addr}): {url_clone}");
            } else {
                println!("🚀 STARTING direct download {new_id}: {url_clone}");
            }

            if let Err(error) = easy.progress(true) {
                eprintln!("[Download {new_id}] Progress setup failed: {error}");
                return;
            }
            let p_pause = is_paused.clone();
            let p_percent = progress_percent.clone();
            let p_total = total_bytes.clone();
            let p_down = downloaded_bytes.clone();
            if let Err(error) = easy.progress_function(move |total, downloaded, _, _| {
                let paused = p_pause.lock().map(|value| *value).unwrap_or(true);
                if paused || total > MAX_DOWNLOAD_BYTES || downloaded > MAX_DOWNLOAD_BYTES {
                    return false;
                }
                if total > 0.0 {
                    if let Ok(mut value) = p_percent.lock() {
                        *value = (downloaded / total).clamp(0.0, 1.0) as f32;
                    }
                    if let Ok(mut value) = p_total.lock() {
                        *value = total;
                    }
                    if let Ok(mut value) = p_down.lock() {
                        *value = downloaded;
                    }
                }
                true
            }) {
                eprintln!("[Download {new_id}] Progress callback setup failed: {error}");
                return;
            }

            let path = Path::new(&path_clone);
            if let Ok(metadata) = fs::symlink_metadata(path) {
                if metadata.file_type().is_symlink() {
                    eprintln!("[Download {new_id}] Refusing symlink destination: {path_clone}");
                    return;
                }
            }
            let mut file = match OpenOptions::new().create(true).append(true).open(path) {
                Ok(file) => file,
                Err(error) => {
                    eprintln!("[Download {new_id}] Cannot open {path_clone}: {error}");
                    return;
                }
            };
            let starting_size = match file.metadata() {
                Ok(metadata) => metadata.len(),
                Err(error) => {
                    eprintln!("[Download {new_id}] Cannot stat {path_clone}: {error}");
                    return;
                }
            };
            if starting_size as f64 > MAX_DOWNLOAD_BYTES {
                eprintln!("[Download {new_id}] Existing file exceeds size limit");
                return;
            }
            if starting_size > 0 {
                if let Err(error) = easy.resume_from(starting_size) {
                    eprintln!("[Download {new_id}] Resume setup failed: {error}");
                    return;
                }
            }

            let write_pause = is_paused.clone();
            if let Err(error) = easy.write_function(move |data| {
                if write_pause.lock().map(|value| *value).unwrap_or(true) {
                    return Ok(0);
                }
                // Returning zero tells libcurl to stop with a write error. It is
                // preferable to panicking a detached worker on a disk failure.
                if file.write_all(data).is_err() {
                    return Ok(0);
                }
                Ok(data.len())
            }) {
                eprintln!("[Download {new_id}] Write callback setup failed: {error}");
                return;
            }

            match easy.perform() {
                Ok(()) => {
                    if let Ok(mut complete) = is_completed.lock() {
                        *complete = true;
                    }
                    if let Ok(mut progress) = progress_percent.lock() {
                        *progress = 1.0;
                    }
                    println!("✅ COMPLETED download {new_id} to {path_clone}");
                }
                Err(error) => {
                    if error.is_aborted_by_callback() {
                        println!("⏸️ PAUSE/BOUNDS ACKNOWLEDGED for {path_clone}");
                    } else {
                        eprintln!("[Download {new_id}] Transfer failed: {error}");
                    }
                }
            }
        });
    }

    pub fn pause_download(&mut self, id: usize) {
        if let Ok(downloads) = self.downloads.lock() {
            if let Some(download) = downloads.iter().find(|download| download.id == id) {
                if let Ok(mut paused) = download.is_paused.lock() {
                    *paused = true;
                }
            }
        }
    }

    pub fn resume_download(&mut self, id: usize) {
        let original = self
            .downloads
            .lock()
            .ok()
            .and_then(|downloads| downloads.iter().find(|download| download.id == id).cloned());
        let Some(download) = original else { return };
        if download
            .is_completed
            .lock()
            .map(|completed| *completed)
            .unwrap_or(true)
        {
            return;
        }
        if let Ok(mut paused) = download.is_paused.lock() {
            *paused = false;
        }
        self.start_download(&download.url, &download.path, download.tor_proxy);
    }

    pub fn get_progress(&self, id: usize) -> Option<f32> {
        let downloads = self.downloads.lock().ok()?;
        let download = downloads.iter().find(|download| download.id == id)?;
        download.progress_percent.lock().ok().map(|value| *value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_stable_and_not_vector_indexes() {
        let mut manager = DownloadManager::new();
        manager.start_download("https://example.com/a", "test-a.bin", None);
        manager.start_download("https://example.com/b", "test-b.bin", None);
        let downloads = manager.downloads.lock().unwrap();
        assert_eq!(downloads[0].id, 0);
        assert_eq!(downloads[1].id, 1);
    }

    #[test]
    fn proxy_is_retained_on_download_record() {
        let mut manager = DownloadManager::new();
        manager.start_download(
            "https://example.com/file.bin",
            "test-proxy.bin",
            Some("socks5h://127.0.0.1:9150".to_string()),
        );
        let downloads = manager.downloads.lock().unwrap();
        assert_eq!(downloads[0].tor_proxy.as_deref(), Some("socks5h://127.0.0.1:9150"));
    }
}
