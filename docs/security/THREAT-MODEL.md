# Threat model and security limits

## Assets and boundaries

Protect mathematical correctness, rule provenance, build integrity and local
selections. Boundaries are user input, imported JSON, OS/Web Crypto, dependency
and compiler supply chains, repository permissions and served static assets.

## Implemented controls

- No accounts, secrets, payments, database or network generation endpoint.
- OS randomness natively and Web Crypto through getrandom in WASM.
- No time-based seeds, Math.random or deterministic production seed option.
- Errors return no partially successful record.
- Private sampler; forbidden unsafe code in application crates.
- Strict record fields, trusted registry comparison, bounded file reads (1 MiB)
  and batches (1..100); serde recursion limit remains enabled.
- Imported records are parsed by Rust, never interpreted as HTML.
- UI renders via textContent; CSV has fixed trusted identifiers and numeric data.
- Locked Cargo/npm resolution, ignored npm lifecycle scripts, pinned CI action
  commits and read-only workflow permissions. Checkout does not persist credentials.
- Meta CSP restricts scripts/assets to self and WASM compilation; no analytics.

## What is not established

There is no claim of NIST/FIPS/WLA certification, external penetration testing,
independent code review or unbreachability. Rust does not prevent logical bugs.
A compromised browser, OS, repository maintainer, compiler or hosted origin can
defeat application controls. A developer with deploy access can alter delivered
code; access revocation and independent review are owner controls.

Digest verification identifies a bundled model. It cannot prove output origin,
randomness, time of generation or absence of editing. Valid edited numbers can
pass. There are no signatures, transparent draw log or reproducible-build claim.

The meta CSP cannot enforce frame-ancestors or every security header. Production
hosting needs an additional header/HTTPS review. Development Vite HMR may be
restricted by the strict policy; production preview is the verified demo route.
No public hosting has been configured by committing a static application.

## Release and incident boundaries

The owner must enable main protections and private vulnerability reporting.
CODEOWNERS is descriptive without those controls. Branch access is not production
access. Revoke unused app grants and use MFA; never send credentials to an agent.

On a correctness/security defect: stop recommending the affected version, preserve
evidence, identify impacted records/profiles, implement a corrective commit and
publish an advisory through the appropriate private/public path. Never silently
replace rules behind an existing immutable identifier.

Dependency scans cover the application lockfiles; they do not certify toolchain
binaries or every transitive dependency of separately installed build tools.
