# Repository cleanup manifest

## Remove from product Git

| Path/pattern | Classification | Reason |
|---|---|---|
| `.catisen_history.json` | Sensitive runtime data | Full browsing history and sensitive query URLs. Purge Git history if pushed. |
| `.cache/catisen/**` | Sensitive runtime data | Cookie/profile state and per-tab storage. |
| `target/**` | Generated build output | Build artifacts, path leakage, huge noise. |
| `attached_assets/Screenshot_*.png` | Agent evidence | Screenshots accidentally committed as progress. |
| `attached_assets/image_*.png` | Agent evidence | Captured UI/evidence artifacts, not product assets. |
| `attached_assets/Pasted-*.txt` | Agent evidence | Pasted prompts and workflow context. |
| `.continue/**` | Local developer/agent config | Floating `npx` MCP packages and local filesystem path. |
| `.agents/**` | Local agent memory | Not product source or reproducible project state. |
| `src/**/*.bak-*` | Source backups | Duplicate/dead source outside Git history. |
| `src/**/*.bak2-*` | Source backups | Duplicate/dead source outside Git history. |
| `run_err*.txt`, `*.log` | Runtime diagnostics | May contain full URLs and machine paths. |
| `chrome_dump.txt` | Runtime/browser dump | Generated output and possible private navigation data. |
| `catisen_test_report.txt` | Runtime report | Generated report; keep canonical reports under `docs/`. |
| `test.bin` | Test artifact | Generated binary. |
| `ublock_temp_to_read.txt` | Temporary staging file | Not a source-of-truth filter list. |

## Preserve or move

- Keep `assets/easylist.txt` and `assets/mentality_bg.png` only if they are actually used by the active build.
- Move the prior audit Markdown to `docs/reference/prior-audit-2026-07-02.md`.
- Keep one canonical current report in `docs/`; archive old reports outside the product repository if they contain raw URLs.

## Do not mistake cleanup for history removal

Deleting a file from the working tree only fixes future commits. If any sensitive path was pushed, use `git filter-repo`, rotate exposed credentials/tokens, and force-push with coordination.
