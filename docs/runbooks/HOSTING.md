# Hosted laboratory preview: Render

Target: 0.1.0-alpha.2. Render reports the preview live as of 2026-10-09,
service `srv-db47psm0tbcc73dep2vg`, deployed commit `0a970a0`.
The provider build and CI passed. Direct public HTTPS/header checks and
live-browser/client acceptance have not been independently completed.
See `docs/verification/HOSTED-PREVIEW-2026-10-09.md`.
Client outcome: browser generation without installing Rust. Official games remain
disabled; this is not the completed official-game minor release.

## Infrastructure contract

render.yaml defines one static site, no database, VM, worker, secrets or API keys.
Static sites have no compute-plan field. Workspace bandwidth/build allowances
still apply; the owner must review displayed costs before creation. Auto deploys
are off: later updates require deliberate publication of a reviewed green commit.

The build pins Node 24.19.0, Rust 1.98.1 and wasm-bindgen 0.2.104, installs locked
dependencies and produces web/dist. When rustup is absent, its official HTTPS
installer is used with certificate verification. That bootstrap and the provider
remain supply-chain trust boundaries. CI runs the same build script, but provider
bootstrap and CDN behavior require live verification. No reproducibility or
independent certification claim is made.

Mutable Rust homes are explicitly relocated to .drawlab-build/rustup and
.drawlab-build/cargo inside the writable checkout. The directory is ignored by
Git. The build installs its own rustup and proxy executables into this local
Cargo bin directory even when Render preinstalls rustup. Moving RUSTUP_HOME
alone is insufficient: the system rustup binary tries to update sibling proxies
in read-only /usr/local/cargo/bin. CI deliberately supplies unwritable inherited
homes and verifies the local Rust executables exist after the build.

Headers restrict scripts, connections and frames, disable MIME sniffing/referrers
and unnecessary device permissions. WASM compilation uses wasm-unsafe-eval; no
unsafe-eval/unsafe-inline permission. No SPA rewrite: missing assets must fail.
No-cache requires revalidation so old HTML/build identity does not mask updates.
Render provides HTTPS; verify it at acceptance.

## Initial owner setup (completed 2026-10-09)

The connected Render creation tool cannot set the full response-header policy.
Use the version-controlled Blueprint. After render.yaml is merged to main and
CI passes:

1. Open https://dashboard.render.com/blueprint/new?repo=https://github.com/GyLiber/gyliber-drawlab
2. Select the intended workspace (the connector listed My Workspace); connect
   GitHub if prompted, granting only the repository access needed for DrawLab.
3. Select main and render.yaml. Name the Blueprint gyliber-drawlab.
4. Review one Static Site, no paid compute/database or secrets, build command
   bash scripts/build-render.sh, publish path web/dist, auto deploy off and the
   headers from render.yaml. If paid resources are requested or settings differ,
   stop and report the mismatch instead of accepting charges.
5. Apply. Wait for deployment, then return the displayed HTTPS URL or exact error.
   Never share credentials or recovery codes.

Applying creates the public preview, not a stable release tag.

## Post-deploy acceptance (developer)

1. Confirm Render reports live; record service/deploy IDs and deployed Git SHA.
2. Fetch / and /build-info.json over HTTPS: require 200, alpha.2 version and the
   deployed SHA. Build identity is a label, not a cryptographic attestation.
3. Inspect CSP including frame-ancestors, nosniff, DENY, no-referrer,
   Permissions-Policy and Cache-Control. Verify WASM loads successfully.
4. In a real browser generate two boards, export/import JSON, reject a duplicate
   main number, exercise models/modes, exact odds and narrow-screen layout, and
   confirm official candidates remain disabled. Record the evidence.
5. Only then record the URL as live and request client acceptance. Official rules,
   independent review and stable 0.1.0 gates remain separate.

## Failure, rollback and compatibility

Inspect failing asset URLs, MIME types and CSP directives; do not weaken headers
to hide errors. Repair through a tested PR and redeploy the green commit.
After a successful deployment, use Render's prior-deploy rollback for regressions
and verify build-info again. The first deployment has no previous live rollback:
stop publication until repaired. Keep auto deploy off.

Records are version-specific: alpha.2 rejects alpha.1 JSON. Retain alpha.1's
verifier for earlier records; do not edit version fields to imply compatibility.
A reviewed migration policy remains future work.

References reviewed 2026-10-09: https://render.com/docs/static-sites,
https://render.com/docs/static-site-headers,
https://render.com/docs/blueprint-spec, https://render.com/docs/rust-toolchain.
