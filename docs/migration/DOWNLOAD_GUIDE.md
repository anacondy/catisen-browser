# Catisen workspace download guide

## Download these files from this workspace

### Required

1. [`SPLIT_PLAN.md`](./SPLIT_PLAN.md) — explains the two repositories, commit selection, file ownership, and commands.
2. [`scripts/split-catisen-projects.ps1`](./scripts/split-catisen-projects.ps1) — extracts the historical and current projects from Git commits.
3. [`scripts/validate-split.ps1`](./scripts/validate-split.ps1) — verifies that the two repositories have different architectures and contain no clutter.
4. [`patches/0001-workflow-hardening.patch`](./patches/0001-workflow-hardening.patch) — hardens GitHub Actions and adds CI gates.
5. [`patches/0003-current-browser-security.patch`](./patches/0003-current-browser-security.patch) — patches the supplied current wry source.

### Recommended documentation

6. [`SOURCE_REVIEW_ADDENDUM.md`](./SOURCE_REVIEW_ADDENDUM.md) — confirmed findings from the supplied Rust source.
7. [`AUDIT_REPORT.md`](./AUDIT_REPORT.md) — complete project and architecture audit.
8. [`CLEANUP_MANIFEST.md`](./CLEANUP_MANIFEST.md) — files that must not remain in either repository.
9. [`scripts/cleanup-repository-artifacts.ps1`](./scripts/cleanup-repository-artifacts.ps1) — standalone cleanup for an existing checkout.

## Do not download/apply these for the normal split

- `patches/0002-policy-modules-reference.patch` — reference-only; patch 0003 already includes the policy for the current source.
- `reference/*.rs` — readable reference copies only.
- `source-snapshot/` — extracted analysis snapshot, not a replacement for Git history.
- `current-intended-patched/` — analysis workspace copy used to generate patch 0003; do not copy it over the project.

## Files to obtain from the actual project Git repository

The PowerShell split script obtains them automatically:

- Historical project: commit `ea726eb`.
- Current browser: commit `2e03418`.

Do not manually mix files between those commits.
