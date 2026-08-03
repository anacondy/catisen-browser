//! IPC message types: JS toolbar → Rust event loop.
//!
//! Every message the toolbar JavaScript sends to Rust goes through
//! `window.ipc.postMessage(JSON.stringify({ t: "<tag>", ...fields }))`.
//! The `#[serde(tag = "t")]` attribute maps the "t" field to the enum variant.

use serde::Deserialize;

/// Messages the toolbar JavaScript sends to Rust via `window.ipc.postMessage(json)`.
#[derive(Debug, Deserialize)]
#[serde(tag = "t")]
pub enum IpcMsg {
    // ── Navigation controls ───────────────────────────────────────────────────
    #[serde(rename = "nav")]
    Navigate { url: String },
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "fwd")]
    Forward,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "fullscreen")]
    ToggleFullscreen,
    #[serde(rename = "newtab")]
    NewTab,

    // ── UI panel toggles ──────────────────────────────────────────────────────
    /// Ctrl+, — open the Settings overlay panel.
    #[serde(rename = "settings")]
    OpenSettings,
    /// Ctrl+D — toggle the debug overlay panel.
    #[serde(rename = "debug")]
    ToggleDebugPanel,
    /// Ctrl+R — toggle Reader / text-only mode.
    /// Previously: ReaderMode existed in text_mode.rs but had no IPC entry point.
    #[serde(rename = "reader")]
    ToggleReaderMode,
    /// Settings panel button: generate a new sync identity and show QR code.
    /// Previously: SyncChain existed in sync_chain.rs but had no IPC entry point.
    #[serde(rename = "sync_chain")]
    ShowSyncChain,
    /// Trigger a file download through DownloadManager with Tor routing.
    /// Previously: DownloadManager was initialised but start_download() was never called.
    #[serde(rename = "start_download")]
    StartDownload { url: String, path: String },

    // ── Runtime setting changes (fired from the Settings overlay) ─────────────
    #[serde(rename = "set_adblock")]
    SetAdblock { enabled: bool },
    #[serde(rename = "set_isolation")]
    SetIsolation { enabled: bool },
    /// Tor toggle note: changing this at runtime is logged and saved to config, but
    /// WebKit proxy env-vars are set at process start so a full restart is needed
    /// for WebView traffic to actually go through/off Tor.  The download manager
    /// respects the change immediately for new downloads.
    #[serde(rename = "set_tor")]
    SetTor { enabled: bool },
    /// Grant or deny a hardware permission for a specific domain.
    /// Previously: PermissionManager existed in permissions.rs but set_permission()
    /// was never called from anywhere in the runtime.
    #[serde(rename = "set_permission")]
    SetPermission { domain: String, permission: String, state: String },
    #[serde(rename = "set_save_history")]
    SetSaveHistory { enabled: bool },
}

/// Events that flow into the tao event loop.
#[derive(Debug)]
pub enum AppEvent {
    // ── Navigation ────────────────────────────────────────────────────────────
    Navigate(String),
    Back,
    Forward,
    Reload,
    ToggleFullscreen,
    NewTab,

    // ── URL-bar / title sync ──────────────────────────────────────────────────
    UpdateUrlBar(String),

    // ── UI panel events ───────────────────────────────────────────────────────
    OpenSettings,
    ToggleDebugPanel,
    /// Toggle the ReaderMode struct in text_mode.rs and inject the CSS transform.
    ToggleReaderMode,
    /// Generate a SyncChain QR identity and show it as an in-page modal.
    ShowSyncChain,
    /// Start a file download, routing through the active Tor proxy if enabled.
    StartDownload { url: String, path: String },

    // ── Runtime setting changes ───────────────────────────────────────────────
    SetAdblock { enabled: bool },
    SetIsolation { enabled: bool },
    SetTor { enabled: bool },
    /// Update the PermissionManager for the given domain + permission type.
    SetPermission { domain: String, permission: String, state: String },
    SetSaveHistory { enabled: bool },
}
