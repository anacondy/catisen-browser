# Catisen CHANGELOG HISTORY

**June 7, 2026** - Initial auto-setup.
## 2026-06-07 16:02 UTC - 855fa27f732781648b6b78501860c11945bb6d63
**Message**: feat(ci): Add basic CI pipeline for Rust build & check (June 7, 2026)
**Author**: anacondy
**Changes**: See commit 855fa27f732781648b6b78501860c11945bb6d63 for stats

## Changelog Entry - 2026-06-07 17:16 UTC
Commit: b5e7706e1a3a49f5e69082537698aa276ead90eb
Message: fix(workflow): Create proper changelog and CI workflows with correct permissions
---
\n## 2026-06-08 17:47 UTC - 5de05b9f788b260e18084ce4594de169eba639ef
**Commit Message:** fix(workflows): Create proper changelog and CI workflows + README
\n## 2026-07-05 12:01 UTC - 40207113ee658bdf3434c53a85fcd0aed1c29174
**Commit Message:** Untrack .cache/ (cookie profile data) and gitignore it
\n## 2026-07-15 20:04 UTC - 9ab513a4939474dceda5649e1d2f4d73e09c94f1
**Commit Message:** Update dependencies and configuration files

Replit-Commit-Author: Agent

## 2026-08-03 - Catisen browser audit remediation

- Verified Windows/WebView2 compilation and 35 unit tests plus the integration smoke test.
- Added native Catisen window icon and 16:9 default sizing.
- Changed F11/Fn+F11 to title-bar-only borderless mode; resizing returns when decorations return.
- Removed Catisen's custom F-key fullscreen interception so YouTube owns its player shortcut.
- Removed forced video aspect-ratio/height mutation after it caused clipping on non-16:9 media.
- Reduced inactive helper/dead-code surface and removed the unused text extraction module.
- Kept this work in the open review PR; no merge or force push.
