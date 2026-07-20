//! Debug Panel and Settings Overlay
//!
//! Generates self-contained JavaScript strings that, when evaluated inside the
//! WebView via `webview.evaluate_script(...)`, inject or remove overlay panels.
//!
//! Two panels are provided:
//!   • `toggle_debug_panel_js()` — technical debug info overlay (Ctrl+D)
//!   • `open_settings_panel_js()` — user-facing settings panel (Ctrl+,)
//!
//! The settings panel now includes entries for:
//!   - Ad blocker toggle
//!   - Tab isolation toggle
//!   - Tor routing toggle (requires restart to affect WebView traffic)
//!   - Reader Mode toggle (wires text_mode.rs via IPC)
//!   - Sync Chain / QR identity button (wires sync_chain.rs via IPC)
//!   - Permission management (wires permissions.rs via IPC)

/// Returns JavaScript that toggles the Catisen debug overlay on/off.
/// Running the script a second time removes the panel — the JS implements the toggle.
pub fn toggle_debug_panel_js() -> String {
    r#"
(function () {
    'use strict';
    const PANEL_ID = '__cat_debug_panel';
    const CSS_ID   = '__cat_debug_css';

    const existing = document.getElementById(PANEL_ID);
    if (existing) {
        existing.remove();
        const css = document.getElementById(CSS_ID); if (css) css.remove();
        return;
    }

    const s = document.createElement('style');
    s.id = CSS_ID;
    s.textContent = `
#__cat_debug_panel {
    position:fixed!important; top:60px!important; right:16px!important;
    width:420px!important; max-height:calc(100vh - 80px)!important; overflow-y:auto!important;
    background:#0d1117!important; border:1px solid #388bfd!important; border-radius:8px!important;
    z-index:2147483646!important; padding:16px!important;
    font-family:'Cascadia Code','Fira Mono',monospace!important; font-size:12px!important;
    color:#c9d1d9!important; box-shadow:0 8px 32px rgba(0,0,0,.72)!important; line-height:1.6!important;
}
#__cat_debug_panel h2 { all:initial!important; display:block!important; color:#388bfd!important;
    font-family:inherit!important; font-size:14px!important; font-weight:700!important; margin:0 0 12px!important; }
#__cat_debug_panel .st { color:#8b949e!important; font-size:10px!important; text-transform:uppercase!important;
    letter-spacing:1px!important; margin:12px 0 4px!important; display:block!important;
    border-bottom:1px solid #21262d!important; padding-bottom:3px!important; }
#__cat_debug_panel .row { display:flex!important; justify-content:space-between!important; padding:2px 0!important; }
#__cat_debug_panel .k { color:#8b949e!important; flex-shrink:0!important; }
#__cat_debug_panel .v { color:#3fb950!important; text-align:right!important; word-break:break-all!important; }
#__cat_debug_panel .v.w { color:#d29922!important; }
#__cat_debug_panel .v.e { color:#f85149!important; }
#__cat_debug_panel #__cat_dbg_close { all:initial!important; cursor:pointer!important; float:right!important;
    color:#8b949e!important; font-family:inherit!important; font-size:16px!important;
    padding:2px 6px!important; border:1px solid #30363d!important; border-radius:4px!important;
    background:#161b22!important; }`;
    (document.head || document.documentElement).appendChild(s);

    // §9.7 XSS fix: build with textContent, not innerHTML — location.href / title may be attacker-controlled
    function row(k, v, cls) {
        const d = document.createElement('div'); d.className = 'row';
        const ks = document.createElement('span'); ks.className = 'k'; ks.textContent = k;
        const vs = document.createElement('span'); vs.className = 'v' + (cls ? ' ' + cls : ''); vs.textContent = v;
        d.appendChild(ks); d.appendChild(vs); return d;
    }
    function sec(t) { const s = document.createElement('span'); s.className='st'; s.textContent=t; return s; }

    const panel = document.createElement('div'); panel.id = PANEL_ID;
    const closeBtn = document.createElement('button'); closeBtn.id='__cat_dbg_close'; closeBtn.textContent='✕';
    closeBtn.onclick = () => { panel.remove(); s.remove(); };
    panel.appendChild(closeBtn);
    const h2 = document.createElement('h2'); h2.textContent='🦊 Catisen Debug Panel'; panel.appendChild(h2);

    panel.appendChild(sec('Page'));
    panel.appendChild(row('URL', location.href.slice(0,80)+(location.href.length>80?'…':'')));
    panel.appendChild(row('Title', (document.title||'(none)').slice(0,80)));
    panel.appendChild(row('Protocol', location.protocol, location.protocol==='https:'?'':'w'));
    panel.appendChild(row('Toolbar', document.getElementById('__cat_bar')?'✅ present':'❌ missing',
        document.getElementById('__cat_bar')?'':'e'));

    panel.appendChild(sec('Catisen'));
    panel.appendChild(row('Version', '0.2.0'));
    panel.appendChild(row('Engine', 'wry 0.38 + WebKitGTK'));
    panel.appendChild(row('Privacy JS', window.__catisen_injected?'✅ injected':'❌ missing',
        window.__catisen_injected?'':'e'));

    panel.appendChild(sec('Fingerprint / Privacy'));
    panel.appendChild(row('userAgent', (navigator.userAgent||'').slice(0,50)+'…'));
    panel.appendChild(row('platform', navigator.platform));
    panel.appendChild(row('language', navigator.language));
    panel.appendChild(row('hardwareConcurrency', String(navigator.hardwareConcurrency)));
    panel.appendChild(row('webdriver', String(navigator.webdriver), navigator.webdriver?'e':''));
    panel.appendChild(row('Timezone', (()=>{try{return Intl.DateTimeFormat().resolvedOptions().timeZone;}catch(_){return'?';}})()));

    panel.appendChild(sec('Shortcuts'));
    [['Ctrl+,','Settings'],['Ctrl+D','Debug Panel'],['Ctrl+R','Reader Mode'],
     ['Alt+←','Back'],['Alt+→','Forward'],['F5','Reload'],['Ctrl+L','URL bar']].forEach(([k,v])=>{
        panel.appendChild(row(k,v));
    });

    document.documentElement.appendChild(panel);
})();
    "#.to_string()
}

/// Returns JavaScript that opens (or closes) the Settings overlay panel.
///
/// The settings panel now includes:
///   - Ad blocker, Tab isolation, Tor toggles (existing)
///   - Reader Mode toggle (NEW — wires `text_mode::ReaderMode` via IPC)
///   - Sync Chain QR button (NEW — wires `sync_chain::SyncChain` via IPC)
///   - Permission controls (NEW — wires `permissions::PermissionManager` via IPC)
pub fn open_settings_panel_js() -> String {
    r#"
(function () {
    'use strict';
    const PANEL_ID = '__cat_settings_panel';
    const CSS_ID   = '__cat_settings_css';

    const existing = document.getElementById(PANEL_ID);
    if (existing) { existing.remove(); const c=document.getElementById(CSS_ID); if(c)c.remove(); return; }

    const s = document.createElement('style'); s.id = CSS_ID;
    s.textContent = `
#__cat_settings_panel {
    position:fixed!important; top:60px!important; left:50%!important; transform:translateX(-50%)!important;
    width:500px!important; max-height:calc(100vh - 80px)!important; overflow-y:auto!important;
    background:#0d1117!important; border:1px solid #30363d!important; border-radius:10px!important;
    z-index:2147483645!important; padding:20px 24px!important;
    font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif!important;
    font-size:13px!important; color:#c9d1d9!important; box-shadow:0 12px 40px rgba(0,0,0,.80)!important;
}
#__cat_settings_panel h2 { all:initial!important; display:block!important; color:#e6edf3!important;
    font-family:inherit!important; font-size:16px!important; font-weight:700!important; margin:0 0 16px!important; }
#__cat_settings_panel .section-title { color:#8b949e!important; font-size:10px!important;
    text-transform:uppercase!important; letter-spacing:1px!important;
    margin:14px 0 6px!important; display:block!important; border-bottom:1px solid #21262d!important; padding-bottom:3px!important; }
#__cat_settings_panel .sr { display:flex!important; align-items:center!important;
    justify-content:space-between!important; padding:10px 0!important; border-bottom:1px solid #161b22!important; }
#__cat_settings_panel .sl { color:#c9d1d9!important; }
#__cat_settings_panel .sd { color:#8b949e!important; font-size:11px!important; margin-top:2px!important; }
#__cat_settings_panel .tog { position:relative!important; width:40px!important; height:22px!important;
    background:#21262d!important; border:1px solid #30363d!important; border-radius:11px!important;
    cursor:pointer!important; flex-shrink:0!important; transition:background .2s!important; }
#__cat_settings_panel .tog.on { background:#1f6feb!important; border-color:#388bfd!important; }
#__cat_settings_panel .tog::after { content:''!important; position:absolute!important;
    width:16px!important; height:16px!important; background:#fff!important; border-radius:50%!important;
    top:2px!important; left:2px!important; transition:left .2s!important; }
#__cat_settings_panel .tog.on::after { left:20px!important; }
#__cat_settings_panel .action-btn { all:initial!important; cursor:pointer!important; display:block!important;
    width:100%!important; padding:9px 0!important; margin-top:6px!important;
    background:#161b22!important; border:1px solid #30363d!important; border-radius:6px!important;
    color:#c9d1d9!important; font-family:inherit!important; font-size:13px!important;
    text-align:center!important; transition:background .15s!important; }
#__cat_settings_panel .action-btn:hover { background:#21262d!important; color:#e6edf3!important; }
#__cat_settings_panel .close-btn { all:initial!important; cursor:pointer!important; float:right!important;
    color:#8b949e!important; font-family:inherit!important; font-size:16px!important;
    padding:2px 8px!important; border:1px solid #30363d!important; border-radius:4px!important;
    background:#161b22!important; }
#__cat_settings_panel .close-btn:hover { color:#e6edf3!important; background:#21262d!important; }`;
    (document.head || document.documentElement).appendChild(s);

    function ipc(msg) { try { window.ipc && window.ipc.postMessage(JSON.stringify(msg)); } catch(_){} }

    function tog(id, on, cb) {
        const el = document.createElement('div');
        el.className = 'tog' + (on ? ' on' : ''); el.id = id;
        el.onclick = () => { const now = el.classList.toggle('on'); cb(now); };
        return el;
    }

    // §9.7 also fix settings row to use textContent (defense-in-depth)
    function row(label, desc, control) {
        const r = document.createElement('div'); r.className = 'sr';
        const lc = document.createElement('div');
        const sl = document.createElement('div'); sl.className='sl'; sl.textContent=label;
        const sd = document.createElement('div'); sd.className='sd'; sd.textContent=desc;
        lc.appendChild(sl); lc.appendChild(sd);
        r.appendChild(lc); if (control) r.appendChild(control); return r;
    }

    function sec(title) {
        const sp = document.createElement('span'); sp.className='section-title'; sp.textContent=title; return sp;
    }

    const panel = document.createElement('div'); panel.id = PANEL_ID;

    const close = document.createElement('button'); close.className='close-btn'; close.textContent='✕';
    close.onclick = () => { panel.remove(); s.remove(); };
    panel.appendChild(close);

    const h2 = document.createElement('h2'); h2.textContent='⚙️ Catisen Settings'; panel.appendChild(h2);

    // ── Privacy section ───────────────────────────────────────────────────────
    panel.appendChild(sec('Privacy & Blocking'));
    panel.appendChild(row('Ad & Tracker Blocking',
        'Block ads via EasyList rules + JS fetch/XHR intercept',
        tog('cs_adblock', true, on => ipc({ t:'set_adblock', enabled:on }))));
    panel.appendChild(row('Strict Tab Isolation',
        'Each tab uses an isolated cookie jar (multi-account support)',
        tog('cs_isolation', false, on => ipc({ t:'set_isolation', enabled:on }))));

    // ── Tor section ───────────────────────────────────────────────────────────
    panel.appendChild(sec('Tor Routing'));
    panel.appendChild(row('Tor (Downloads Only on Windows)',
        'Linux: WebView + downloads route via SOCKS5h (set before WebView build). ' +
        'Windows (WebView2): env vars ignored — only libcurl downloads use Tor. ' +
        'Requires Tor daemon running + browser restart.',
        tog('cs_tor', false, on => ipc({ t:'set_tor', enabled:on }))));

    // ── Reading section ───────────────────────────────────────────────────────
    panel.appendChild(sec('Reader Mode'));
    panel.appendChild(row('Text-only / Reader View',
        'Strip scripts, ads, and heavy media — clean typography (Ctrl+R)',
        tog('cs_reader', false, on => ipc({ t:'reader' }))));

    // ── Permissions section ───────────────────────────────────────────────────
    // §9.8 fix: use location.hostname for per-site permission, not '*' no-op
    panel.appendChild(sec('Hardware Permissions'));
    panel.appendChild(row('Geolocation (this site)',
        'Allow/deny geolocation for current site (HTTP auto-denied)',
        tog('cs_geo', false, on => ipc({ t:'set_permission', domain: location.hostname || '*', permission:'geolocation', state: on?'granted':'denied' }))));
    panel.appendChild(row('Camera & Microphone',
        'Default: deny to all sites unless explicitly granted',
        null));

    // ── Sync Chain section ────────────────────────────────────────────────────
    panel.appendChild(sec('Device Sync'));
    const syncBtn = document.createElement('button');
    syncBtn.className = 'action-btn';
    syncBtn.textContent = '🔗 Generate New Sync Identity & QR Code';
    syncBtn.onclick = () => {
        ipc({ t: 'sync_chain' });
        syncBtn.textContent = '⏳ Generating…';
        setTimeout(() => { syncBtn.textContent = '🔗 Generate New Sync Identity & QR Code'; }, 3000);
    };
    panel.appendChild(syncBtn);

    document.documentElement.appendChild(panel);
})();
    "#.to_string()
}
