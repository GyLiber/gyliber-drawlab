# DrawLab 0.1.0-alpha.2 — hosted-preview deployment checkpoint

Date: 2026-10-09 (UTC); client: Gyile / GyLiber.

Follow-up: [2026-10-10 live browser verification](LIVE-BROWSER-2026-10-10.md)
completed the scoped public HTTP/header and desktop interaction checks below.
This historical record describes what was known on 2026-10-09; client acceptance
is still outstanding.

## Delivery identity

- Product: mathematical laboratory preview `0.1.0-alpha.2`, **not** stable `0.1.0` or verified official ITHUBA game coverage.
- Render service: `srv-db47psm0tbcc73dep2vg` (Static Site, auto deploy off).
- Public URL supplied by Render: https://gyliber-drawlab.onrender.com
- Deployment: `dep-db483mjncjis73c4rik0`.
- Deployed commit: `0a970a046adca28f8d9ce8ecf4c9080a80d11bae`.
- Provider status: `live`, finished `2026-10-09T05:58:00Z`.
- Source: https://github.com/GyLiber/gyliber-drawlab/pull/8 (merged).
- CI evidence: https://github.com/GyLiber/gyliber-drawlab/actions/runs/37890554287 (pull request) and https://github.com/GyLiber/gyliber-drawlab/actions/runs/37890541863 (push); each passed both required jobs.

## Observed production build

Render's log reported a successful pinned Rust `1.98.1` build of `drawlab-core` and `drawlab-wasm` at version `0.1.0-alpha.2`, followed by TypeScript/Vite production output:
- `dist/index.html` (6.30 KB)
- `dist/assets/drawlab_wasm_bg-D8nYQcY6.wasm` (184.22 KB)
- `dist/assets/index-B0fDMw7X.css` (5.40 KB)
- `dist/assets/index-CUvbfjTD.js` (9.61 KB)

The log ended in `Your site is live`. Unlike earlier failed deploys, the builder progressed beyond the provider's read-only system Rust directories by installing Rust tools within an ignored writable project directory.

## Verification boundaries

**Passed:** source PR CI Rust formatting/lint/tests; native CLI sample; WASM production build; automated browser tests; dependency advisory audit; Render production build; provider-reported `live` status and deploy SHA matching the `main` commit at deployment.

**Not directly verified:** public HTTP 200 for `/` and `/build-info.json`, deployed build-info content, response security headers, runtime WASM loading at the public URL, or a human-operated live-browser generation/export/import session. Direct HTTPS fetch was unavailable from this working environment. GitHub CI browser tests validate the built application, not the hosted HTTP response. Provider `live` is not proof of every acceptance criterion.

**Pending client acceptance:** Open the public URL in a current browser; confirm the 0.1.0-alpha.2 lab interface; generate two boards; export and import JSON; verify duplicate-number import is rejected; confirm candidate official games are disabled. Do not enter payment or account details.

## Non-goals and next release gates

Do not label the above as a final `0.1.0` release. Full applicable operator rules, independent conformance fixtures, current-versus-historical profile separation, catalogue coverage, review and owner acceptance remain unresolved. No stable tag or GitHub Release has been published. See `docs/coverage.md`, `docs/NEXT-STEPS.md`, `docs/runbooks/RELEASE.md`.

Data classification: Public. This is a deployment/provenance record, not an independent security audit or an attestation of official-game correctness.
