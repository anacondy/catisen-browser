# Catisen Browser

This repository is the current wry/tao browser extracted from commit `2e03418`.

Supported direction: embedded WebView, document-start privacy scripts, toolbar IPC, EasyList filtering, reader mode, and platform-specific WebKitGTK/WebView2 integration.

Release blockers still require explicit work: Windows WebView2 proxy routing, real renderer sandboxing, per-tab WebView storage isolation, privileged IPC isolation, and valid current-runtime integration tests.
