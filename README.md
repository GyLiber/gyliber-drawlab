# GyLiber DrawLab

A mathematical lottery laboratory for **Gyile / GyLiber**.
Unbiased generation, exact combinatorics, inspectable records.

**Version: 0.1.0-alpha.1 — laboratory preview, not stable 0.1.0.**
No official game profile is enabled. This tool does not sell tickets, publish
official results, predict draws or promise winnings.

## Working scope

- Rust CLI and a static browser interface running the same Rust core via WASM.
- Five mathematical models: 6/58, 6/52, 5/50+1/20, 5/50+1/16, and 5/36.
- Separate selection and simulated-draw modes; correct bonus-pool distinctions.
- Fresh OS/Web Crypto randomness, unbiased bounded sampling, no insecure fallback.
- 1–100 independent boards, exact match probabilities and CLI main-match tables.
- JSON records with model digests; CLI/browser validation; browser text/CSV export.
- Visible, disabled ITHUBA candidates until full official rules are verified.

These models are intentionally named lab-* rather than marketed as official
games. Full source retrieval, add-on relationships, historical/current rule
versions and complete catalogue verification remain release blockers.
See [coverage](docs/coverage.md).

## Native quick start

Install Rust with rustup using its official instructions. The committed toolchain
file selects Rust 1.98.1. From the repository root:

    cargo build -p drawlab-cli --locked
    cargo run -p drawlab-cli --locked -- games list
    cargo run -p drawlab-cli --locked -- generate --profile lab-5of36 --boards 2
    cargo run -p drawlab-cli --locked -- simulate-draw --profile lab-6of58 --format text
    cargo run -p drawlab-cli --locked -- odds --profile lab-5of50-20

Export and check:

    cargo run -q -p drawlab-cli --locked -- generate --profile lab-5of36 > sample.json
    cargo run -q -p drawlab-cli --locked -- verify sample.json

A valid record proves only supported format/model/number validity. A modified
record containing valid numbers can pass; origin and randomness are not proved.

## Browser quick start

Requires Node 24 and the pinned Rust toolchain. Initial dependency/toolchain
downloads need internet. Generated selections never leave the browser.

    cargo install wasm-bindgen-cli --version 0.2.104 --locked
    npm --prefix web ci --ignore-scripts
    bash scripts/build-web.sh
    npm --prefix web run preview -- --port 4173

Open http://127.0.0.1:4173. Use production preview for the tested CSP behavior.
Do not open index.html through file://. No account, database, VM or paid service
is needed. No public deployment is configured automatically.

## Verification

    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked
    cargo build -p drawlab-cli --locked
    bash scripts/build-web.sh
    cd web
    npx playwright install --with-deps chromium
    npm test

Separately run cargo-audit 0.22.2 and npm audit. CI reproduces these checks with
read-only permissions and pinned action commits. See
[verification record](docs/verification/ALPHA-1.md) for actual observed results,
not merely intended checks.

## Engineering map

| Boundary | Location |
|---|---|
| Sampling, models, probability and record validation | crates/drawlab-core |
| Native adapter and CLI integration tests | crates/drawlab-cli |
| Narrow browser exports | crates/drawlab-wasm |
| Accessible static UI and browser tests | web |
| Bundled profile registry | rules |
| Mathematics, ADRs, security, coverage and evidence | docs |
| Locked build helpers and automated checks | scripts, .github |

Future official profiles require a source, edition/effective scope, extracted
clauses, test fixtures and review. Future product categories use new adapters,
not silent reinterpretations of ball-based models. Do not add plugins that can
execute arbitrary imported code.

## Project records

- [Remaining next steps and batch checkpoints](docs/NEXT-STEPS.md)
- [Design baseline v1.0.0](docs/design-v1.0.0.md) — proposal, not completion claim.
- [Actual preview decisions](docs/architecture/ADR-0002-preview-and-record-contract.md)
- [Sampling mathematics](docs/mathematics/SAMPLING.md)
- [Threat model](docs/security/THREAT-MODEL.md)
- [Contribution policy](CONTRIBUTING.md) and [governance](docs/governance/DEVELOPMENT.md)
- [Release and demonstration runbook](docs/runbooks/RELEASE.md)
- [Changelog](CHANGELOG.md)

Conventional Commits, focused PRs and GyLiber's reserved-rights license follow
the Command Center's established practices. A passing build is not independent
security certification. Public visibility does not confer an open-source license.
