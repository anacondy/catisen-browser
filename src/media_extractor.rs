use std::process::Command;

pub fn open_media_stream(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("mpv").arg(url).spawn().or_else(|_| {
            Command::new("vlc").arg(url).spawn()
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("mpv").arg(url).spawn().or_else(|_| {
            Command::new("vlc").arg(url).spawn()
        });
    }
    eprintln!("[Catisen] Launched external media player for: {}", url);
}