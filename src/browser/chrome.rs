//! Browser toolbar — injected as JavaScript into every page at document-start.
//!
//! # Contents
//! - `TOOLBAR_JS`       — the full toolbar + keyboard shortcuts + Catisen flyout menu
//! - `privacy_init_js`  — stealth BOM overrides, fetch/XHR/popup ad blocking, cosmetic filters
//!
//! # Keyboard shortcuts
//!   Alt+Left / Alt+Right → Back / Forward
//!   F5                   → Reload
//!   Ctrl+L               → Focus URL bar
//!   Ctrl+T               → New Tab        ← NEW
//!   Ctrl+Tab             → Next Tab       ← NEW
//!   Ctrl+,               → Settings overlay
//!   Ctrl+D               → Debug panel overlay
//!   Ctrl+R               → Toggle Reader mode
//!
//! # New features vs previous version
//! - Ctrl+T and Ctrl+Tab shortcuts added (wired to IPC `newtab`)
//! - URL bar accepts drag-and-drop of URLs from other apps / tabs
//! - Catisen badge click opens a flyout with version info + Dark/Light/System theme toggle
//! - Dark scrollbar CSS injected on every page (`::-webkit-scrollbar` webkit extension)
//! - `window.open()` is intercepted to block ad-domain popups
//! - `MutationObserver` removes ad iframes injected after page load
//! - Cosmetic CSS filter hides common ad element classes/IDs

/// Fixed toolbar injected into every page at document-start.
///
/// The script is idempotent (checks `window.__catisen_injected`) so SPA routers
/// that blow away the DOM don't get double toolbars.
pub const TOOLBAR_JS: &str = r#"
(function () {
    'use strict';

    // ── Idempotency guard ──────────────────────────────────────────────────────
    // Top-frame guard — only mount in the actual top-level page, never inside
    // an iframe. Without this, any page with an embedded iframe (e.g.
    // DuckDuckGo's video preview panel) gets a second toolbar mounted inside
    // the iframe's own separate window context.
    if (window.top !== window.self) return;

    if (window.__catisen_injected) return;
    window.__catisen_injected = true;

    const H = 52; // toolbar height in px

    /* ── Styles ─────────────────────────────────────────────────────────────── */
    const CSS = `
/* ── Reset ─────────────────────────────────────────────────── */
#__cat_bar * { box-sizing: border-box !important; }

/* ── Toolbar strip ──────────────────────────────────────────── */
#__cat_bar {
    all: initial !important;
    position: fixed !important;
    top: 0 !important; left: 0 !important; right: 0 !important;
    height: ${H}px !important;
    background: #0d1117 !important;
    border-bottom: 1px solid #21262d !important;
    z-index: 2147483647 !important;
    display: flex !important;
    align-items: center !important;
    padding: 0 10px !important;
    gap: 6px !important;
    box-shadow: 0 2px 12px rgba(0,0,0,.55) !important;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
}

/* ── Toolbar buttons ────────────────────────────────────────── */
#__cat_bar button {
    all: initial !important;
    cursor: pointer !important;
    color: #8b949e !important;
    background: #161b22 !important;
    border: 1px solid #30363d !important;
    border-radius: 5px !important;
    padding: 5px 9px !important;
    font-size: 14px !important;
    line-height: 1 !important;
    transition: background .12s, color .12s !important;
    user-select: none !important;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
}
#__cat_bar button:hover  { background: #21262d !important; color: #e6edf3 !important; }
#__cat_bar button:active { background: #30363d !important; }

/* ── URL bar wrapper ────────────────────────────────────────── */
#__cat_url_wrap {
    flex: 1 !important;
    display: flex !important;
    align-items: center !important;
    background: #010409 !important;
    border: 1px solid #30363d !important;
    border-radius: 6px !important;
    padding: 0 10px !important;
    height: 33px !important;
    gap: 7px !important;
    transition: border-color .15s !important;
}
#__cat_url_wrap:focus-within { border-color: #388bfd !important; }
#__cat_url_wrap.drag-over    { border-color: #3fb950 !important; border-style: dashed !important; }

/* ── Padlock icon ───────────────────────────────────────────── */
#__cat_lock { font-size: 13px !important; flex-shrink: 0 !important; }

/* ── URL input ──────────────────────────────────────────────── */
#__cat_url {
    all: initial !important;
    flex: 1 !important;
    color: #c9d1d9 !important;
    font-size: 13px !important;
    background: transparent !important;
    outline: none !important;
    border: none !important;
    caret-color: #58a6ff !important;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
}

/* ── Privacy badge ──────────────────────────────────────────── */
#__cat_badge {
    all: initial !important;
    font-size: 11px !important;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
    color: #3fb950 !important;
    background: #0f2d0f !important;
    border: 1px solid #238636 !important;
    border-radius: 4px !important;
    padding: 4px 9px !important;
    white-space: nowrap !important;
    letter-spacing: .3px !important;
    flex-shrink: 0 !important;
    cursor: pointer !important;
    user-select: none !important;
    transition: background .12s !important;
}
#__cat_badge:hover { background: #0f3a0f !important; }

/* ── New tab button ─────────────────────────────────────────── */
#__cb_newtab {
    color: #58a6ff !important;
    border-color: #1f6feb !important;
    font-size: 16px !important;
    padding: 3px 9px !important;
}
#__cb_newtab:hover { background: #1f6feb22 !important; color: #79c0ff !important; }

/* ── Reader mode button glows amber when active ─────────────── */
#__cb_reader.active { color: #e3b341 !important; border-color: #e3b341 !important; background: #2d1f0022 !important; }

/* ── Catisen flyout menu ────────────────────────────────────── */
#__cat_flyout {
    display: none !important;
    position: fixed !important;
    top: ${H + 4}px !important;
    right: 10px !important;
    width: 270px !important;
    background: #0d1117 !important;
    border: 1px solid #30363d !important;
    border-radius: 8px !important;
    z-index: 2147483646 !important;
    padding: 14px 16px !important;
    box-shadow: 0 8px 32px rgba(0,0,0,.80) !important;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif !important;
    font-size: 12px !important;
    color: #c9d1d9 !important;
}
#__cat_flyout.open { display: block !important; }
#__cat_flyout .fly-title {
    color: #3fb950 !important; font-weight: 700 !important; font-size: 13px !important;
    margin-bottom: 2px !important; display: block !important;
}
#__cat_flyout .fly-ver { color: #8b949e !important; font-size: 10px !important; margin-bottom: 12px !important; display: block !important; }
#__cat_flyout .fly-sec {
    color: #484f58 !important; font-size: 9px !important; text-transform: uppercase !important;
    letter-spacing: 1px !important; margin-bottom: 5px !important; display: block !important;
    border-top: 1px solid #21262d !important; padding-top: 8px !important;
}
#__cat_flyout .fly-feat { color: #8b949e !important; font-size: 11px !important; padding: 1px 0 !important; display: block !important; }
#__cat_flyout .fly-feat::before { content: "✓ " !important; color: #3fb950 !important; }
#__cat_flyout .fly-theme-row { display: flex !important; gap: 5px !important; margin-top: 8px !important; }
#__cat_flyout .fly-tbtn {
    all: initial !important;
    flex: 1 !important;
    text-align: center !important;
    border: 1px solid #30363d !important;
    border-radius: 4px !important;
    padding: 5px 0 !important;
    background: #161b22 !important;
    color: #8b949e !important;
    font-family: inherit !important;
    font-size: 11px !important;
    cursor: pointer !important;
    transition: background .12s !important;
}
#__cat_flyout .fly-tbtn:hover  { background: #21262d !important; color: #c9d1d9 !important; }
#__cat_flyout .fly-tbtn.active { border-color: #388bfd !important; color: #58a6ff !important; background: #1f6feb22 !important; }

/* ── Dark mode scrollbars (WebKit extension) ────────────────── */
/* Previously: scrollbars defaulted to white/grey system chrome,
   which clashed with dark pages.  Now all pages get matching dark bars. */
::-webkit-scrollbar           { width: 8px !important; height: 8px !important; }
::-webkit-scrollbar-track     { background: #0d1117 !important; }
::-webkit-scrollbar-thumb     { background: #30363d !important; border-radius: 4px !important; }
::-webkit-scrollbar-thumb:hover{ background: #484f58 !important; }
::-webkit-scrollbar-corner    { background: #0d1117 !important; }`;

    /* ── IPC helper ─────────────────────────────────────────────────────────── */
    function ipc(msg) {
        try { window.ipc && window.ipc.postMessage(JSON.stringify(msg)); } catch (_) {}
    }

    /* ── Toolbar markup ─────────────────────────────────────────────────────── */
    function buildBar() {
        const isHttps = location.protocol === 'https:';
        const bar = document.createElement('div');
        bar.id = '__cat_bar';
        bar.innerHTML = `
            <button id="__cb_back"     title="Back (Alt+Left)">&#9664;</button>
            <button id="__cb_fwd"      title="Forward (Alt+Right)">&#9654;</button>
            <button id="__cb_reload"   title="Reload (F5)">&#8635;</button>
            <div id="__cat_url_wrap">
                <span id="__cat_lock" title="${isHttps ? 'Secure connection (HTTPS)' : 'Not secure (HTTP)'}">
                    ${isHttps ? '&#128274;' : '&#9888;&#65039;'}
                </span>
                <input id="__cat_url"
                    type="text"
                    spellcheck="false"
                    autocomplete="off"
                    autocorrect="off"
                    autocapitalize="off"
                    value="${location.href.replace(/"/g, '&quot;')}"
                    placeholder="Search or enter address…" />
            </div>
            <button id="__cb_newtab"   title="New Tab (Ctrl+T)">&#43;</button>
            <button id="__cb_reader"   title="Reader Mode (Alt+R)">&#128214;</button>
            <button id="__cb_settings" title="Settings (Ctrl+,)">&#9881;&#65039;</button>
            <button id="__cb_debug"    title="Debug Panel (Ctrl+D)">&#128027;</button>
            <span   id="__cat_badge">&#128737;&#65039;&nbsp;Catisen</span>`;

        /* ── Flyout menu (separate element, appended outside the toolbar) ─── */
        const flyout = document.createElement('div');
        flyout.id = '__cat_flyout';
        flyout.innerHTML = `
            <span class="fly-title">&#128737;&#65039; Catisen</span>
            <span class="fly-ver">v0.2.0 &mdash; Privacy Browser</span>
            <span class="fly-sec">Privacy Features</span>
            <span class="fly-feat">EasyList ad &amp; tracker blocking</span>
            <span class="fly-feat">Navigator / canvas fingerprint spoofing</span>
            <span class="fly-feat">Strict HTTPS enforcement</span>
            <span class="fly-feat">Tab isolation (planned; separate profiles not active)</span>
            <span class="fly-feat">Tor routing (Linux WebView; downloads when configured)</span>
            <span class="fly-feat">Reader mode (best effort)</span>
            <span class="fly-feat">Sync identity (pairing not implemented)</span>
            <span class="fly-sec">Theme</span>
            <div class="fly-theme-row">
                <button class="fly-tbtn" id="__fly_dark"   title="Force dark mode"  >&#127770; Dark</button>
                <button class="fly-tbtn" id="__fly_light"  title="Force light mode" >&#127774; Light</button>
                <button class="fly-tbtn" id="__fly_system" title="Use system theme"  >&#9881; System</button>
            </div>`;

        return { bar, flyout };
    }

    /* ── Theme toggle ────────────────────────────────────────────────────────
     * "Dark" / "Light" / "System" buttons in the Catisen flyout.
     *
     * We inject a <style> tag that forces the page colour scheme.  This is a
     * best-effort override — some pages use Shadow DOM or inline styles that
     * may resist it — but it works well on most text-heavy sites.
     */
    function applyTheme(mode) {
        let s = document.getElementById('__cat_theme_override');
        if (!s) {
            s = document.createElement('style');
            s.id = '__cat_theme_override';
            (document.head || document.documentElement).appendChild(s);
        }
        if (mode === 'dark') {
            s.textContent = `
                html, body { background: #0d1117 !important; color: #c9d1d9 !important; color-scheme: dark !important; }
                a { color: #58a6ff !important; }`;
        } else if (mode === 'light') {
            s.textContent = `
                html, body { background: #ffffff !important; color: #1f2328 !important; color-scheme: light !important; }
                a { color: #0969da !important; }`;
        } else {
            // System: remove overrides, let the page/browser choose
            s.textContent = '';
        }
        // Reflect active state on theme buttons
        ['dark','light','system'].forEach(function(t) {
            var el = document.getElementById('__fly_' + t);
            if (el) el.classList.toggle('active', t === mode);
        });
    }

    /* ── Mount toolbar ───────────────────────────────────────────────────── */
    function mount() {
        if (document.getElementById('__cat_bar')) return;

        // Inject CSS
        if (!document.getElementById('__cat_css')) {
            var s = document.createElement('style');
            s.id = '__cat_css';
            s.textContent = CSS;
            (document.head || document.documentElement).appendChild(s);
        }

        var { bar, flyout } = buildBar();
        document.documentElement.insertBefore(bar, document.documentElement.firstChild);
        document.documentElement.appendChild(flyout);

        // Push page body down so content isn't hidden behind the toolbar
        function pushBody() {
            if (document.body)
                document.body.style.setProperty('margin-top', H + 'px', 'important');
        }
        pushBody();
        document.addEventListener('DOMContentLoaded', pushBody);

        /* ── Navigation buttons ──────────────────────────────────────────── */
        document.getElementById('__cb_back').onclick   = function() { ipc({ t: 'back' }); };
        document.getElementById('__cb_fwd').onclick    = function() { ipc({ t: 'fwd' }); };
        document.getElementById('__cb_reload').onclick = function() { ipc({ t: 'reload' }); };
        document.getElementById('__cb_newtab').onclick = function() { ipc({ t: 'newtab' }); };

        /* ── Reader mode button ──────────────────────────────────────────── */
        var readerBtn = document.getElementById('__cb_reader');
        readerBtn.onclick = function() {
            ipc({ t: 'reader' });
            // Toggle amber "active" class for immediate visual feedback.
            readerBtn.classList.toggle('active');
        };

        /* ── Settings & debug panel buttons ──────────────────────────────── */
        document.getElementById('__cb_settings').onclick = function() { ipc({ t: 'settings' }); };
        document.getElementById('__cb_debug').onclick    = function() { ipc({ t: 'debug' }); };

        /* ── Catisen badge — toggles the flyout menu ─────────────────────
         * Previously: badge was a static label.
         * Now: click toggles a flyout with version info + theme toggle.
         */
        var badge = document.getElementById('__cat_badge');
        badge.onclick = function(e) {
            e.stopPropagation();
            var fl = document.getElementById('__cat_flyout');
            fl.classList.toggle('open');
        };
        // Close flyout when clicking anywhere else on the page.
        document.addEventListener('click', function() {
            var fl = document.getElementById('__cat_flyout');
            if (fl) fl.classList.remove('open');
        }, true);

        /* ── Theme toggle buttons inside the flyout ──────────────────────── */
        document.getElementById('__fly_dark').onclick   = function(e) { e.stopPropagation(); applyTheme('dark');   };
        document.getElementById('__fly_light').onclick  = function(e) { e.stopPropagation(); applyTheme('light');  };
        document.getElementById('__fly_system').onclick = function(e) { e.stopPropagation(); applyTheme('system'); };

        /* ── URL bar: Enter to navigate, click to select all ─────────────── */
        var urlEl = document.getElementById('__cat_url');
        urlEl.addEventListener('keydown', function(e) {
            if (e.key === 'Enter') {
                e.preventDefault();
                window.__cat.navigate(this.value);
            }
            e.stopPropagation();
        });
        urlEl.addEventListener('click',  function(e) { this.select(); e.stopPropagation(); });
        urlEl.addEventListener('focus',  function()  { this.select(); });

        /* ── URL bar: drag-and-drop support ──────────────────────────────────
         * Previously: the address bar had no drag-and-drop support.
         * Now: dragging a URL (from a link, another tab, or the OS) onto
         * the address bar populates it and auto-navigates.
         */
        var wrap = document.getElementById('__cat_url_wrap');
        wrap.addEventListener('dragover', function(e) {
            e.preventDefault();
            e.dataTransfer.dropEffect = 'copy';
            wrap.classList.add('drag-over');
        });
        wrap.addEventListener('dragleave', function() {
            wrap.classList.remove('drag-over');
        });
        wrap.addEventListener('drop', function(e) {
            e.preventDefault();
            wrap.classList.remove('drag-over');
            var dropped = e.dataTransfer.getData('text/uri-list')
                       || e.dataTransfer.getData('text/plain')
                       || '';
            if (dropped.trim()) {
                urlEl.value = dropped.trim();
                window.__cat.navigate(dropped.trim());
            }
        });
    }

    /* ── Public API (called by Rust via evaluate_script) ────────────────── */
    window.__cat = {
        navigate: function(raw) {
            var url = (raw || '').trim();
            if (!url) return;
            if (!url.match(/^[a-z][a-z0-9+.\-]*:\/\//i)) {
                if (!url.includes(' ') && (url.includes('.') || url.startsWith('localhost'))) {
                    url = 'https://' + url;
                } else {
                    url = 'https://duckduckgo.com/?q=' + encodeURIComponent(url);
                }
            }
            ipc({ t: 'nav', url: url });
        },
        back:   function() { ipc({ t: 'back' }); },
        fwd:    function() { ipc({ t: 'fwd' }); },
        reload: function() { ipc({ t: 'reload' }); },

        // Called by Rust's UpdateUrlBar event after page-load to sync the address bar.
        // Also updates the dynamic padlock icon based on the URL scheme.
        updateUrl: function(url) {
            var el = document.getElementById('__cat_url');
            if (el && document.activeElement !== el) el.value = url;

            // ── Dynamic padlock ────────────────────────────────────────────────
            // Previously: padlock icon was set once at injection time and never updated.
            // Now: updated on every navigation so HTTP pages correctly show the warning.
            var lock = document.getElementById('__cat_lock');
            if (lock) {
                var secure = url.startsWith('https://');
                lock.innerHTML = secure ? '&#128274;' : '&#9888;&#65039;';
                lock.title     = secure
                    ? 'Secure connection (HTTPS)'
                    : 'Not secure — data is sent unencrypted (HTTP)';
            }
        },
    };

    /* ── Global keyboard shortcuts ───────────────────────────────────────────
     * Captured in the capture phase so page scripts cannot intercept them.
     *
     *   Alt+Left / Alt+Right → Back / Forward
     *   F5                   → Reload
     *   Ctrl+L               → Focus URL bar
     *   Ctrl+T               → New Tab        (NEW)
     *   Ctrl+Tab             → Next Tab       (NEW — sends newtab for now; full
     *                          tab switching requires wry multi-window support)
     *   Ctrl+,               → Settings overlay
     *   Ctrl+D               → Debug panel overlay
     *   Ctrl+R               → Reader mode
     */
    document.addEventListener('keydown', function(e) {
        if (e.altKey && e.key === 'ArrowLeft')  { e.preventDefault(); ipc({ t: 'back' }); }
        if (e.altKey && e.key === 'ArrowRight') { e.preventDefault(); ipc({ t: 'fwd' }); }
        if (e.key === 'F5')                     { e.preventDefault(); ipc({ t: 'reload' }); }

        if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
            e.preventDefault();
            var el = document.getElementById('__cat_url');
            if (el) { el.focus(); el.select(); }
        }
        // Ctrl+T — open a new tab (previously unimplemented)
        if ((e.ctrlKey || e.metaKey) && e.key === 't') {
            e.preventDefault();
            ipc({ t: 'newtab' });
        }
        // Ctrl+Tab — cycle to the next tab
        // (Full tab-bar UI is a future milestone; for now this opens a new tab
        //  so the shortcut does something meaningful rather than being swallowed.)
        if ((e.ctrlKey || e.metaKey) && e.key === 'Tab') {
            e.preventDefault();
            ipc({ t: 'newtab' });
        }
        if ((e.ctrlKey || e.metaKey) && e.key === ',') {
            e.preventDefault(); ipc({ t: 'settings' });
        }
        if ((e.ctrlKey || e.metaKey) && e.key === 'd') {
            e.preventDefault(); ipc({ t: 'debug' });
        }
        // Reader mode moved off Ctrl+R: Ctrl+R is now left as native reload,
        // Ctrl+Shift+R as native hard-refresh (WebView2 accelerators, matching
        // standard browser behavior). Alt+R avoids both collisions.
        if (e.altKey && (e.key === 'r' || e.key === 'R')) {
            e.preventDefault();
            ipc({ t: 'reader' });
            var btn = document.getElementById('__cb_reader');
            if (btn) btn.classList.toggle('active');
        }
    }, true);

    /* ── SPA navigation sync ─────────────────────────────────────────────────
     * Patch pushState / replaceState so the URL bar stays correct on SPAs
     * that update the URL without triggering a full page load.
     */
    ['pushState', 'replaceState'].forEach(function(fn) {
        var orig = history[fn].bind(history);
        history[fn] = function() {
            orig.apply(history, arguments);
            setTimeout(function() {
                var el = document.getElementById('__cat_url');
                if (el && document.activeElement !== el) el.value = location.href;
            }, 30);
        };
    });
    window.addEventListener('popstate', function() {
        var el = document.getElementById('__cat_url');
        if (el && document.activeElement !== el) el.value = location.href;
    });

    /* ── MutationObserver re-mount guard ─────────────────────────────────────
     * Some frameworks completely replace `document.documentElement` children.
     * We watch for the toolbar being removed and re-inject it.
     */
    var __cat_obs = new MutationObserver(function() {
        if (!document.getElementById('__cat_bar')) mount();
    });
    function startObserver() {
        __cat_obs.disconnect();
        if (document.documentElement) {
            __cat_obs.observe(document.documentElement, { childList: true, subtree: false });
        }
    }

    /* ── Ad iframe observer ──────────────────────────────────────────────────
     * Catches ad iframes that are injected AFTER the initial page parse.
     * The navigation_handler only fires for top-level navigations, so
     * inline ad iframes loaded via JS escape it.  This MutationObserver
     * closes that gap by watching for new <iframe> elements.
     *
     * Previously: only top-level navigations were blocked.
     * Now: dynamically injected ad iframes are also caught.
     */
    var _AD_DOMAINS = /doubleclick\.net|googlesyndication\.com|googletagmanager\.com|google-analytics\.com|adservice\.google\.|pagead2\.|facebook\.net\/.*pixel|scorecardresearch\.com|taboola\.com|outbrain\.com|criteo\.|adnxs\.com|adsrvr\.org|pubmatic\.com|rubiconproject\.com|openx\.net|casalemedia\.com|appnexus\.com|popads\.net|popcash\.net|propellerads\.com|adsterra\.com|advertising\.com|zedo\.com|exoclick\.com|amazon-adsystem\.com|moatads\.com|quantserve\.com|chartbeat\.com|addthis\.com|33across\.com/i;

    var __cat_iframe_obs = new MutationObserver(function(mutations) {
        mutations.forEach(function(m) {
            m.addedNodes.forEach(function(node) {
                if (node.nodeName !== 'IFRAME') return;
                var src = node.src || node.getAttribute('src') || '';
                if (_AD_DOMAINS.test(src)) {
                    node.style.display = 'none';
                    try { node.src = 'about:blank'; } catch(_) {}
                    console.debug('[Catisen] Ad iframe suppressed:', src);
                }
            });
        });
    });
    function startIframeObserver() {
        __cat_iframe_obs.disconnect();
        if (document.documentElement) {
            __cat_iframe_obs.observe(document.documentElement, { childList: true, subtree: true });
        }
    }

    /* ── Cosmetic element hiding ─────────────────────────────────────────────
     * CSS-based ad element hiding (complements the network-level blocking).
     * Targets known ad container selectors used by major ad platforms.
     */
    function injectCosmeticFilters() {
        if (document.getElementById('__cat_cosmetic')) return;
        var s = document.createElement('style');
        s.id = '__cat_cosmetic';
        s.textContent = [
            '[id^="google_ads_iframe"]', 'ins.adsbygoogle', '[id^="div-gpt-ad"]',
            '[class*="googletag"]', '[class*="taboola"]', '[id*="taboola"]',
            '[class*="outbrain"]', '[id*="outbrain"]',
            '[data-ad-unit]', '[data-ad-name]', '[data-ad-slot]', '[data-testid*="ad"]',
            '.ad-container', '.ad-wrapper', '.ad-slot', '.ad-banner',
        ].join(',') + ' { display: none !important; visibility: hidden !important; }';
        (document.head || document.documentElement).appendChild(s);
    }

    /* ── Bootstrap ───────────────────────────────────────────────────────── */
    // Mount immediately when document-start already provides <html>. Waiting
    // for DOMContentLoaded made the page appear blank and hid the browser
    // toolbar while slow sites were still loading.
    function bootstrap() {
        mount();
        startObserver();
        startIframeObserver();
        injectCosmeticFilters();
    }
    if (document.documentElement) {
        bootstrap();
    } else {
        document.addEventListener('DOMContentLoaded', bootstrap, { once: true });
    }
})();
"#;

/// YouTube-specific ad-configuration stripping.
///
/// Domain blocking cannot distinguish YouTube's ad video segments from real
/// content -- both are served from the same googlevideo.com CDN. This strips
/// ad-placement data out of YouTube's own player response before the player
/// code reads it, so it never learns an ad break exists.
///
/// Covers initial page load (window.ytInitialPlayerResponse property trap)
/// and SPA navigation (fetch() intercept on /youtubei/v1/player and /next,
/// which YouTube's router uses for related-video clicks without a reload).
///
/// NOTE: best-effort against YouTube's current internal shape. Check console
/// for "[Catisen] YT adblock" messages to confirm it is active.
pub const YT_ADBLOCK_JS: &str = r#"
(function () {
    'use strict';
    if (!location.hostname.includes('youtube.com')) return;
    if (window.__catisen_yt_adblock) return;
    window.__catisen_yt_adblock = true;

    function stripAds(resp) {
        try {
            if (resp && resp.playerAds) delete resp.playerAds;
            if (resp && resp.adPlacements) delete resp.adPlacements;
            if (resp && resp.adBreakHeartbeatParams) delete resp.adBreakHeartbeatParams;
            if (resp && resp.adSlots) delete resp.adSlots;
        } catch (e) { console.warn('[Catisen] YT adblock strip failed', e); }
        return resp;
    }

    var _ytResp;
    try {
        Object.defineProperty(window, 'ytInitialPlayerResponse', {
            configurable: true,
            get: function () { return _ytResp; },
            set: function (v) {
                _ytResp = stripAds(v);
                console.debug('[Catisen] YT adblock: stripped initial player response');
            }
        });
    } catch (e) { console.warn('[Catisen] YT adblock trap failed', e); }

    var _origFetch = window.fetch;
    window.fetch = function (resource, ...rest) {
        var url = (typeof resource === 'string') ? resource : (resource && resource.url) || '';
        if (/\/youtubei\/v1\/(player|next)/.test(url)) {
            return _origFetch.call(this, resource, ...rest).then(function (res) {
                return res.clone().json().then(function (data) {
                    stripAds(data);
                    console.debug('[Catisen] YT adblock: stripped SPA response', url);
                    return new Response(JSON.stringify(data), {
                        status: res.status, statusText: res.statusText, headers: res.headers
                    });
                }).catch(function () { return res; });
            });
        }
        return _origFetch.call(this, resource, ...rest);
    };

    console.debug('[Catisen] YouTube ad-config stripping active');
})();
"#;
/// Privacy/BOM override script — runs before any page JS.
///
/// Changes vs previous version:
/// - `window.open()` is intercepted to block popup ads (gaming sites, DDG overlays)
/// - Extended `_AD_RE` regex covers more ad/tracker domains
/// - Cosmetic CSS hides common ad element patterns
///
/// # Arguments
/// * `ua`       — User-agent string to spoof
/// * `platform` — `navigator.platform` value to report
/// * `language` — `navigator.language` value to report
/// * `timezone` — `Intl.DateTimeFormat` timezone to report
pub fn privacy_init_js(ua: &str, platform: &str, language: &str, timezone: &str) -> String {
    // Serialize values as JSON string literals. JSON string escaping is also
    // valid JavaScript string escaping and covers quotes, backslashes, and
    // control/line-separator characters that quote-only replacement misses.
    // Each result already contains its surrounding quotes, so the template
    // below must not add another pair around the placeholders.
    let ua = serde_json::to_string(ua).unwrap_or_else(|_| "\"\"".to_string());
    let platform = serde_json::to_string(platform).unwrap_or_else(|_| "\"\"".to_string());
    let language = serde_json::to_string(language).unwrap_or_else(|_| "\"en-US\"".to_string());
    let timezone = serde_json::to_string(timezone).unwrap_or_else(|_| "\"UTC\"".to_string());

    format!(r#"
(function () {{
    'use strict';
    try {{
        // ── navigator overrides ────────────────────────────────────────────────
        const def = (prop, val) =>
            Object.defineProperty(navigator, prop, {{ get: () => val, configurable: true }});

        def('userAgent',           {ua});
        def('appVersion',          {ua}.replace('Mozilla/', ''));
        def('platform',            {platform});
        def('language',            {language});
        def('languages',           Object.freeze([{language}, 'en']));
        def('hardwareConcurrency', 8);
        def('deviceMemory',        8);
        def('webdriver',           undefined);
        def('pdfViewerEnabled',    true);
        def('plugins',   Object.freeze([
            {{ name: 'Chrome PDF Plugin',  filename: 'internal-pdf-viewer' }},
            {{ name: 'Chrome PDF Viewer',  filename: 'mhjfbmdgcfjbbpaeojofohoefgiehjai' }},
            {{ name: 'Native Client',      filename: 'internal-nacl-plugin' }},
        ]));
        def('mimeTypes', Object.freeze([
            {{ type: 'application/pdf', description: 'Portable Document Format' }},
        ]));

        // ── Suppress chrome.runtime deprecation warnings ───────────────────────
        // FIX: capture chrome.webview BEFORE overwriting window.chrome, and
        // preserve it on the replacement object. Without this, freezing
        // window.chrome silently destroyed wry's IPC bridge to WebView2
        // (window.chrome.webview), breaking every toolbar button and
        // keyboard shortcut in the app.
        const _origChromeWebview = window.chrome && window.chrome.webview;
        window.chrome = Object.freeze({{ runtime: {{}}, webview: _origChromeWebview }});

        // ── Permissions API — preserve notification state ──────────────────────
        if (navigator.permissions && navigator.permissions.query) {{
            const _origQ = navigator.permissions.query.bind(navigator.permissions);
            navigator.permissions.query = (p) =>
                (p && p.name === 'notifications')
                    ? Promise.resolve({{ state: Notification.permission }})
                    : _origQ(p);
        }}

        // Insecure HTTP origins must not reach hardware APIs before the native
        // permission layer has a chance to respond. This document-start guard is
        // defense-in-depth; the Rust/WebView permission callback remains the
        // authoritative control for HTTPS and platform prompts.
        const _insecureHttp = location.protocol === 'http:'
            && !location.hostname.toLowerCase().endsWith('.onion');
        if (_insecureHttp) {{
            if (navigator.geolocation) {{
                const _denyGeo = function (success, error) {{
                    if (typeof error === 'function') error({{ code: 1, message: 'Denied by Catisen' }});
                }};
                navigator.geolocation.getCurrentPosition = _denyGeo;
                navigator.geolocation.watchPosition = function (success, error) {{
                    _denyGeo(success, error); return 0;
                }};
            }}
            if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {{
                navigator.mediaDevices.getUserMedia = function () {{
                    return Promise.reject(new DOMException('Denied by Catisen', 'NotAllowedError'));
                }};
            }}
        }}

        // ── Timezone spoofing ──────────────────────────────────────────────────
        const _origRO = Intl.DateTimeFormat.prototype.resolvedOptions;
        Intl.DateTimeFormat.prototype.resolvedOptions = function () {{
            const o = _origRO.call(this); o.timeZone = {timezone}; return o;
        }};

        // ── Canvas fingerprint noise ───────────────────────────────────────────
        // §9.9: per-session seed (once per page load, not random per call) — deterministic per session, less fingerprintable
        const _catCanvasSeed = '#' + Math.floor(Math.random()*16777215).toString(16).padStart(6,'0');
        const _origToDU = HTMLCanvasElement.prototype.toDataURL;
        HTMLCanvasElement.prototype.toDataURL = function (...args) {{
            const ctx = this.getContext('2d');
            if (ctx) {{
                ctx.save();
                ctx.globalAlpha = 0.004;
                ctx.fillStyle   = _catCanvasSeed;
                ctx.fillRect(0, 0, 1, 1);
                ctx.restore();
            }}
            return _origToDU.apply(this, args);
        }};

        // ── WebGL vendor/renderer spoofing ─────────────────────────────────────
        const _patchWebGL = (Ctx) => {{
            if (!Ctx) return;
            const _orig = Ctx.prototype.getParameter;
            Ctx.prototype.getParameter = function (p) {{
                if (p === 37445) return 'Intel Inc.';
                if (p === 37446) return 'Intel Iris OpenGL Engine';
                return _orig.call(this, p);
            }};
        }};
        _patchWebGL(window.WebGLRenderingContext);
        _patchWebGL(window.WebGL2RenderingContext);

        // ── Ad domain regex — extended list ────────────────────────────────────
        // Previously covered: Google, DoubleClick, Facebook pixel, Twitter, ScoreCard,
        // Taboola, Outbrain, Criteo, Amazon, Moat, Quantserve, Chartbeat.
        //
        // Now also covers: popads, popcash, propellerads, adsterra, advertising.com,
        // zedo, exoclick, trafficjunky, adnxs, adsrvr, pubmatic, openx, rubicon,
        // casalemedia, appnexus, 33across, addthis, shareaholic — common on gaming
        // and video streaming sites that the old regex was missing.
        const _AD_RE = /google-analytics\.com|googletagmanager\.com|googletagservices\.com|doubleclick\.net|googlesyndication\.com|pagead2\.googlesyndication\.com|adservice\.google\.|facebook\.net\/.*pixel|ads\.twitter\.com|scorecardresearch\.com|taboola\.com|outbrain\.com|criteo\.(com|net)|adsystem\.amazon\.|moatads\.com|quantserve\.com|chartbeat\.com|popads\.net|popcash\.net|propellerads\.com|yllix\.com|adsterra\.com|bidvertiser\.com|advertising\.com|zedo\.com|exoclick\.com|trafficjunky\.com|adnxs\.com|adsrvr\.org|pubmatic\.com|openx\.net|rubiconproject\.com|casalemedia\.com|appnexus\.com|33across\.com|addthis\.com|shareaholic\.com|amazon-adsystem\.com|securepubads\.g\.doubleclick\.net/i;

        const _isAd = (url) => {{
            try {{ const u = new URL(url, location.href); return _AD_RE.test(u.hostname + u.pathname); }}
            catch {{ return false; }}
        }};

        // ── fetch() intercept ──────────────────────────────────────────────────
        const _origFetch = window.fetch;
        window.fetch = function (resource, ...rest) {{
            const url = (typeof resource === 'string') ? resource : (resource && resource.url) || '';
            if (_isAd(url)) {{
                console.debug('[Catisen] fetch blocked:', url);
                return Promise.reject(new TypeError('Blocked by Catisen'));
            }}
            return _origFetch.call(this, resource, ...rest);
        }};

        // ── XMLHttpRequest intercept ───────────────────────────────────────────
        const _origOpen = XMLHttpRequest.prototype.open;
        const _origSend = XMLHttpRequest.prototype.send;
        XMLHttpRequest.prototype.open = function (method, url, ...rest) {{
            this.__catisenBlocked = _isAd(String(url));
            // Always put the XHR into the OPENED state. Returning before the
            // native open() call makes a later send() throw InvalidStateError
            // in unrelated page code.
            return _origOpen.call(this, method, url, ...rest);
        }};
        XMLHttpRequest.prototype.send = function (...rest) {{
            if (this.__catisenBlocked) {{
                console.debug('[Catisen] XHR blocked');
                return; // no network request, but no page exception
            }}
            return _origSend.call(this, ...rest);
        }};

        // ── window.open() popup intercept ─────────────────────────────────────
        // Previously: popup ads were not intercepted at the JS level.
        // Now: window.open() calls to known ad domains are blocked.
        //
        // Gaming sites (e.g. cartoongames.net) and some search results use
        // window.open() to launch full popup ad pages in new tabs.  We return
        // null for ad-domain targets (matching the CSP-blocked-popup behaviour)
        // and allow legitimate popups (OAuth, payment windows, etc.) through.
        const _origWindowOpen = window.open;
        window.open = function (url, target, features) {{
            const u = String(url || '');
            if (u && _isAd(u)) {{
                console.debug('[Catisen] Popup blocked:', u);
                return null; // simulates popup blocker — calling code gets null WindowProxy
            }}
            return _origWindowOpen.call(window, url, target, features);
        }};

    }} catch (e) {{ console.warn('[Catisen privacy]', e); }}
}})();
"#)
}
