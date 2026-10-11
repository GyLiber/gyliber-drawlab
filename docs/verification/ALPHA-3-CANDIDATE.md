# Alpha.3 candidate verification — 2026-10-11

Branch: feat/match-probability-explorer; PR #12.
Feature: browser main-match explorer using the Rust core via WASM.
This record is a candidate checkpoint, not release approval or live evidence.

## Observed local checks

- Rust workspace tests: passed, 9 core and 2 CLI integration tests.
- Rust formatting and Clippy with warnings denied: passed.
- Locked WASM release compilation, TypeScript check and Vite production build: passed.
- Browser-source Prettier check and git diff whitespace check: passed.
- Browser runtime tests: not executed locally. Chromium download failed with
  a truncated/invalid ZIP; do not confuse this with an application test failure.
- GitHub browser verification and dependency audit: inspect the exact PR head's
  checks. Pending at this document's creation; no pass is inferred here.

During verification, TypeScript caught an unchecked first row in the new table;
explicit empty-distribution handling corrected it. npm also altered optional
platform lock entries during tooling; the candidate restores the baseline package
entries with only the intended root version change. No dependency upgrade intended.

## Remaining gates

Review exact-head CI and its screenshot artifacts (including the new mobile
explorer screenshot), merge only when required checks pass, then deploy to the
existing Render static site. Verify alpha.3 build identity, explorer behavior and
existing generation/export/import in the live browser. Record client acceptance
before a release tag. Hosted alpha.2 has not been changed by this batch.

The client approved the palette informally on 2026-10-10; that is not acceptance
of this feature or a trademark-clearance claim. Official rules remain unverified.
The [demonstration](../client/MATCH-EXPLORER-ALPHA-3.md) and
[next actions](../NEXT-STEPS.md) define the continuation.
