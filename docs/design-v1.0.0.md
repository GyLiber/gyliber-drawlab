# GyLiber DrawLab
## Software architecture and delivery specification • Design v1.0.0

**Owner:** Gyile / GyLiber  
**Prepared:** 5 October 2026, Africa/Johannesburg  
**Recommended repository:** `gyliber-drawlab`  
**First executable release:** `v0.1.0`  
**Document status:** Implementation baseline; rule verification gates remain open.  
**Product:** Educational lottery selection generator and mathematical draw simulator.  
**Language constraint:** No Python in the product, build scripts, tests, or documentation examples.

## 1. Executive decision

Build a small, inspectable system that generates valid random selections, simulates draws under explicitly identified rules, and explains its mathematics. Its distinguishing design is the connection between each output, the exact game profile, its source evidence, the sampling algorithm, and independently testable claims.

The first release must be useful end to end: choose a verified profile, generate selections or simulated draws, inspect the applicable rules and exact jackpot probability, and export a portable record. A Rust command-line application is the reference implementation; a static browser application runs the same Rust core through WebAssembly. Neither requires accounts, payment processing, a database, or a virtual machine.

The design document is version 1.0.0. That does **not** label unbuilt software as version 1.0.0. Software starts at 0.1.0 and reaches 1.0.0 only after the release criteria below are satisfied.

This is a simulator, not an authorized lottery operator, ticket seller, official result publisher, or certified draw system. It does not predict an independent fair draw. More computation, historical frequencies, or a sophisticated model cannot make a particular valid combination more likely under the uniform independent model.

## 2. Critical discovery: operator and rules are time dependent

The National Lotteries Commission published its welcome of Sizekhaya as the fourth operator on 1 June 2026 [S1]. Therefore “current South African rules” and “ITHUBA rules” cannot be treated as interchangeable requirements.

A current bank participant page reports LOTTO changing to 1–52, the separate PowerBall pool changing to 1–16, PowerBall PLUS becoming PowerBall Xtra, LOTTO PLUS 2 becoming LOTTO 5 Max, Daily LOTTO Plus being removed, and MatchPlay 13 being added [S2]. That same page contains inconsistent LOTTO add-on names elsewhere. It is useful transition evidence but insufficient as the sole implementation specification.

An indexed official ITHUBA retailer manual describes six LOTTO main numbers from 1–58 [S3]. Its direct PDF retrieval returned HTTP 403 during this design review. An older official ITHUBA manual is accessible [S4], but historical documents do not establish current availability or current rules.

**Decision:** preserve distinct historical ITHUBA profiles and current South African profiles. Never overwrite a historical profile with new ranges. Never silently fall back to an old profile when a current one is unverified.

### 2.1 Evidence classifications

- **Verified:** complete applicable authoritative rules inspected, necessary fields extracted, effective scope established, conflicts resolved, and tests reviewed.
- **Corroborated candidate:** credible evidence supports a format, but its complete authoritative specification or effective interval is missing.
- **Unresolved:** insufficient or conflicting evidence. Generation as an official game is disabled.
- **Historical:** applicability is explicitly a past rule era; this is separate from verification status.
- **Generic laboratory:** user-specified mathematical parameters; never presented as official rules.

No profile is promoted to verified merely because an internet snippet, result history, or another generator agrees with it. A source publication date is not automatically the rule's effective date.

## 3. Game coverage and the meaning of “all games”

The coverage register must contain every discovered game, including retired and non-number products. The executable coverage promise must identify an operator, rule era, and product category. A broad “all ITHUBA games” claim is prohibited until the full catalogue has been reconciled with authoritative documents.

### 3.1 Number-draw profile candidates

These are research candidates, not a completed or certified rule registry. A range alone is insufficient to enable a profile.

| Profile family | Candidate selection format | Simulated result distinctions | Evidence / release condition |
|---|---|---|---|
| ITHUBA LOTTO, PLUS 1, PLUS 2, later era | 6 distinct main numbers from 1–58 | Six mains and a bonus from the remaining pool, subject to full-rule verification | 1–58 supported by indexed official manual [S3]; full source and dates required |
| Earlier ITHUBA LOTTO family | Separate 6/52 historical profile | Bonus semantics must follow that edition | Inspect and extract the relevant historical manual [S4]; do not infer dates |
| ITHUBA PowerBall / PowerBall PLUS | Candidate: 5/50 plus a separate 1/20 ball | Equal numeric value may occur in both pools | Full applicable rules, dates, and add-on relationship required |
| ITHUBA DAILY LOTTO / DAILY LOTTO PLUS | Candidate: 5/36 | No bonus in the candidate model | Verify both games separately, including add-on conditions and retirement |
| Current LOTTO and PLUS 1 | Candidate: 6/52 | Bonus and add-on semantics require operator rules | Transition corroborated by participant [S2]; authoritative rules required |
| Current LOTTO 5 Max | Do not infer result format from its name | May differ from the player's reused selection | Full rules required before algorithm or odds are enabled |
| Current PowerBall / PowerBall Xtra | Candidate: 5/50 plus a separate 1/16 ball | Separate pool and separate add-on draw | Participant corroborates changed bonus range [S2]; full rules required |
| Current DAILY LOTTO | Candidate: 5/36 | Standard selection and any multiplay expansion separately specified | Full current rules required |
| Historical PICK 3 / RAPIDO | Digit or other game-specific formats | Order, replacement, bet types and result format may differ | Discovery catalogue [S5]; unresolved until individual rules are obtained |

The catalogue evidence [S5] also lists SPORTSTAKE variants, RAFFLE, EAZiWIN and archived results. Listing proves discovery, not current availability.

### 3.2 Other product families

| Family | Correct abstraction | v0.1.0 treatment |
|---|---|---|
| Sports pools: SPORTSTAKE 4/8/13, rugby, cricket; current MatchPlay 13 | Choices tied to fixtures, periods, outcome alphabets and settlement rules | Visible coverage entries. Enable an educational selection adapter only after those rules are verified. Uniform choices do not model real sporting probabilities. |
| Raffles | Issued ticket identifiers and a finite eligible inventory | No fabricated official ticket IDs. A synthetic raffle requires an explicit synthetic inventory and distinct label. |
| Instant / scratch products, including EAZiWIN variants | A published prize allocation or game-specific outcome model | Do not replace with random ball numbers. Exact simulation blocked without sufficient prize/outcome specification. |
| Retired games | Historical profiles with explicit applicable scope | Never appear as currently available for entry. |

**Scope reconciliation:** v0.1.0 targets complete standard number-selection and draw workflows for the verified number-draw catalogue, including applicable add-ons. It must not claim to simulate every sports, raffle, and instant product. Completing that literal broader request requires separate rule specifications and adapters. Unsupported entries remain visible with reasons; they do not return invented numbers.

If rule acquisition remains blocked, a generic/historical preview can ship as `0.1.0-alpha.1`. It cannot be presented as the requested all-number-games `0.1.0`. This makes missing coverage measurable instead of quietly reducing the commitment.

## 4. Requirements and acceptance boundaries

| ID | Mandatory behavior for v0.1.0 |
|---|---|
| R01 | List profiles with operator, historical/current status, rule version and evidence status. |
| R02 | Generate 1–100 standard boards from a verified profile. Default: one board. |
| R03 | Separate “Generate selections” from “Simulate a draw”; outputs must carry the mode. |
| R04 | Preserve add-on selection reuse where prescribed; simulate separate add-on results with fresh random input. |
| R05 | Enforce pool sizes, uniqueness, ordering, replacement and dependent-pool rules. |
| R06 | Calculate exact combination counts and jackpot probabilities for supported mathematical models. |
| R07 | Export JSON and a readable text summary; CSV export follows a fixed safe schema. |
| R08 | Fail without output on entropy failure, invalid input or unverified rule selection. |
| R09 | Browser and CLI use one sampler and one rule validator. |
| R10 | Supply source-to-rule-to-test traceability and documented limitations. |
| R11 | Run locally without a network; browser assets can be served on localhost. |
| R12 | Display “Simulation — not a ticket or official result”; do not imitate a redeemable ticket. |

The initial release does not implement sales, wallets, identity checks, prizes owed, real-time results, fixture scraping, customer tracking, prediction scores, machine learning, paid subscriptions, custom RNG cryptography, or autonomous rule updates. These would create different operational and assurance requirements.

Number-selection correctness and payout correctness are separate. v0.1.0 reports mathematical match/jackpot probabilities for supported formats, not an asserted cash prize, expected profit, or official settlement.

## 5. Technology decisions

| Layer | Selected approach | Reason and boundary |
|---|---|---|
| Mathematical engine | Stable Rust; edition/toolchain pinned at implementation | Typed invariants, checked arithmetic, native and WASM targets; memory safety is not immunity to logic flaws. |
| Native interface | Rust CLI with `clap` | Scriptable, inspectable, offline reference application. |
| Serialization | `serde` / `serde_json`, strict versioned schemas | Portable records; deny unknown fields at external boundaries where forward compatibility is not intended. |
| Random source | `getrandom` using supported operating-system facilities; explicit browser Web Crypto backend | Platform cryptographic randomness, errors propagated. Confirm exact crate configuration against pinned docs [S8]. |
| Browser engine | Rust compiled to WASM with `wasm-bindgen` | Reuse the actual core instead of translating algorithms twice. |
| User interface | TypeScript strict mode, semantic HTML/CSS, Vite | A small static application; no framework needed for this initial workflow. |
| Tests | Rust unit/integration tests, `proptest`, parser fuzzing; Playwright browser tests | Mathematical, boundary and actual user-flow assurance. |
| Build/release | Cargo lockfile; Node LTS and npm lockfile; GitHub Actions | Pin exact versions and action commit SHAs; no floating “latest” in release builds. |
| Documentation | Markdown, mathematical notation, ADRs and machine-readable rule registry | Reviewable alongside implementation. |
| Initial hosting | Free static-hosting candidate: GitHub Pages, subject to eligibility and usage policy | Static educational app; no VM necessary [S10]. |

These are design selections, not assertions that dependencies have already been installed or audited. At implementation, check current supported versions, advisories, licenses and browser compatibility, then record the exact choices. Avoid unnecessary packages and build-time scripts. Third-party dependencies may contain unsafe internals; application crates must forbid unsafe code and explicitly document dependency trust.

The repository should initially be **private** while rule provenance and security workflows are established. Free GitHub Pages availability depends on account and repository visibility. Do not make the repository public merely to obtain hosting. Local execution is the zero-hosting-cost baseline; publication and visibility are separate owner decisions.

## 6. Architecture and trust boundaries

| Component | Responsibility | Must not do |
|---|---|---|
| `drawlab-core` | Validated profile types, bounded sampling, combinatorics, result validation | Network I/O, environment secrets, arbitrary code execution |
| `drawlab-rules` | Load bundled approved profiles; enforce schema and provenance | Fetch or enable remote rules automatically |
| `drawlab-cli` | Parse commands, use OS entropy, serialize outputs | Accept production deterministic seeds |
| `drawlab-wasm` | Narrow typed interface and platform entropy adapter | Expose test-only RNG hooks |
| `web` | Accessible selection flow, result display, downloads | Generate randomness with `Math.random`, send selections to a server |
| Test harness | Scripted random streams, independent small-space oracle, fault injection | Become a hidden production mode |

Data flow is: user request → bounded parser → verified profile → sampler plus entropy adapter → independent output validator → serializer → display/export.

Trust boundaries are user input, imported JSON, browser/OS randomness, dependency/build supply chain, published assets, and repository/deployment administration. A hash of a profile identifies bytes; it does not establish that the bytes represent the correct official rules.

Production entropy is fresh per request. Do not retain a home-made seeded PRNG between requests. The browser receives randomness from Web Crypto [S9]; the native build uses the platform backend [S8]. No timestamp, process ID, recent result, user name or public seed is an entropy substitute. There is no insecure fallback.

## 7. Mathematical specification

### 7.1 Probability space and exact arithmetic

For a uniform unordered selection of k distinct numbers from n, the number of possible boards is

`N = C(n,k) = n! / (k! (n-k)!)`, with `P(board) = 1/N`.

For an independent special ball with m possibilities, `N = C(n,k) × m`. Exact probabilities are stored as reduced integer numerator/denominator pairs; percentages are presentation only. Use checked integer arithmetic, with profile bounds that ensure intermediate calculations fit, or an explicitly reviewed big-integer implementation. Never silently round a combination count through floating point.

Independently calculated reference values (these validate models, not rule currency):

| Mathematical model | Number of jackpot combinations |
|---|---:|
| 6 from 58 | 40,475,358 |
| 6 from 52 | 20,358,520 |
| 5 from 50 and 1 from 20 | 42,375,200 |
| 5 from 50 and 1 from 16 | 33,900,160 |
| 5 from 36 | 376,992 |

### 7.2 Unbiased bounded integers

Assume independent uniform 32-bit words supplied by the platform cryptographic source. To sample in `[0,b)` with `1 ≤ b ≤ 2^32`:

1. Compute `L = floor(2^32 / b) × b` using at least 64-bit arithmetic.
2. Read `x` as a 32-bit unsigned integer with explicitly defined byte order.
3. Reject if `x ≥ L`; otherwise return `x mod b`.
4. Limit retries defensively (for example, 128 attempts per sample) and return an error if exhausted. Do not replace failure with a biased result.

Proof: among the `L` accepted input values, each residue occurs exactly `L/b` times. Conditioning on acceptance therefore gives probability `1/b` for each result. The bounded retry policy may produce an error; it must not change the distribution of successful outputs.

Direct `x mod b` is generally biased because `2^32` need not be divisible by b. Reject random-comparator sorting and rounding floating-point random values.

### 7.3 Sampling without replacement

Initialize `[1,…,n]`. For `i = 0,…,k−1`, uniformly select `j` in `[i,n)`, swap positions i and j, and retain position i. This is a partial Fisher–Yates shuffle. Initialization costs O(n), selection costs O(k), memory O(n); these game pools are tiny.

Each ordered k-tuple has probability `1/(n(n−1)…(n−k+1))`. Each unordered k-subset has k! orderings, giving probability `1/C(n,k)`. Sort a copy for display. Preserve sampled order in draw records if the profile requires it.

When a bonus comes from the remaining main pool, draw `k+1` without replacement, take the first k as mains, and the next as bonus. Do not sort all `k+1` and take the last as bonus. For PowerBall-style independent pools, draw the special ball separately; overlapping numerical values across pools are valid.

Digit games require a different adapter: ordered positions with replacement can include leading zeros and repeated digits. Store digits as a string or array, not an integer that loses leading zeros.

### 7.4 Probability explanations

For r matches against k main numbers drawn from n:

`P(R=r) = C(k,r) C(n−k,k−r) / C(n,k)`.

For a bonus drawn from the remaining n−k balls, conditional on r main matches, the probability that the fixed ticket contains that bonus is `(k−r)/(n−k)`. For an independent m-ball special pool, multiply the main-match probability by `1/m` for a special match or `(m−1)/m` for a miss.

These are mathematical identities; mapping them to payout divisions requires separate verified rules. Multiple draws, add-ons, and roll-down rules must not be collapsed into one jackpot model.

Across t independent plays with win probability p, `P(at least one win) = 1−(1−p)^t`. For d distinct jackpot boards in one uniform draw with N outcomes, coverage is exactly `d/N`. These are different sampling situations. Duplicate boards are permitted in independent generation; do not silently deduplicate. A later “distinct boards” mode must state that its boards are dependent samples without replacement from the board space.

### 7.5 Advanced mathematics roadmap

Advanced work must answer a concrete question and remain outside the production entropy mechanism. Planned extensions are combinatorial ranking/unranking, exact multiplay expansion `C(s,k)`, finite-population coverage, confidence intervals for Monte Carlo experiments, and formal bounded-model checks of sampler invariants.

Statistical diagnostics may examine marginal frequencies, pair frequencies and serial dependence. Predeclare hypotheses, sample counts and multiplicity treatment. A small p-value triggers investigation; a large p-value does not prove randomness or cryptographic security. Never repeatedly run tests until they pass. Ordinary CI correctness gates should use deterministic evidence, not flaky significance thresholds.

Large simulations belong in bounded jobs with recorded parameters and seeds in a separate research executable. Expected jackpot waiting times make naive Monte Carlo inefficient for rare-event probability estimation; use exact combinatorics first. GPUs, vector machines and quantum randomness are unnecessary for v0.1.0.

## 8. Rule registry and portable outputs

Each rule profile requires: immutable ID; operator and jurisdiction; profile version; source edition and locator; source URL; retrieval timestamp; source digest where bytes were obtained; effective start/end or explicit unknown values; verification status; reviewer; selection pools; draw pools; replacement/order policies; bonus relationship; add-on dependencies; allowed modes; historical/current status; and test IDs.

Keep selection rules separate from draw rules. Cross-field validation rejects impossible configurations, mismatched dependent pools, cycles in add-on dependencies, unsupported types and ambiguous effective periods. Multiple plausible profiles for an as-of date cause an error rather than an arbitrary selection.

Define profile digests over a documented canonical UTF-8 representation. Do not hash incidental pretty-print whitespace. Immutable source evidence is retained only where reuse permission permits; otherwise preserve bibliographic identifiers, precise locators and extraction notes. Do not invent a source digest when the source bytes were unavailable.

An output record includes `schema_version`, `software_version`, `build_commit`, `profile_id`, `profile_version`, `profile_digest`, `mode`, `generated_at`, `entropy_backend`, `boards` or `draws`, and the simulation label. The timestamp is informational, not trusted proof of when a draw occurred. No entropy seed or internal random buffer is exported.

Verification checks syntax, resource limits, profile identity and mathematical validity. In v0.1.0 it does **not** prove that a result was sampled randomly, that a timestamp is authentic, or that the record was never edited. Output hashes alone provide none of those guarantees.

Proposed commands, to be implemented rather than assumed available:

```sh
drawlab games list
drawlab games show <verified-profile-id>
drawlab generate --profile <verified-profile-id> --boards 2 --format json
drawlab simulate-draw --profile <verified-profile-id> --format json
drawlab odds --profile <verified-profile-id>
drawlab verify ./simulation.json
```

Cap imported files at 1 MiB, batches at 100 boards, depth at the parser's documented limit, and all numerical parameters at supported profile bounds. Unknown identifiers, malformed JSON and mismatched versions return actionable errors with nonzero CLI exit codes. Error categories include `RULES_UNVERIFIED`, `INVALID_REQUEST`, `ENTROPY_UNAVAILABLE`, `UNSUPPORTED_MODE`, and `INVALID_RECORD`.

## 9. Security model, including the developer

The objective is verifiable integrity with minimal authority. No design honestly guarantees that its author, a future attacker, or a compromised platform can never breach it. Knowing the source should not grant access; possessing release credentials or controlling the delivered executable is a different capability.

| Threat | Mandatory control | Residual limitation |
|---|---|---|
| Developer inserts biased code | Independent review of sampler, profiles and release diff; protected branches and release environment | A single owner acting as author and approver is not independent separation of duties. |
| Developer or assistant retains access | Scoped development access; no production signing keys; owner revokes sessions/tokens after work | While an assistant is granted write/deploy authority it can affect those resources. |
| Supply-chain replacement | Locked dependencies, pinned actions, minimal packages, advisory checks, release manifest/SBOM | A lockfile can lock a malicious version; review is still required. |
| Weak or failed randomness | OS/Web Crypto source, fail closed, bounded sampler, test fault injection | Trust remains in OS/browser/platform implementation. |
| Rule tampering or stale defaults | Bundled versioned profiles, reviewable rule diffs, source evidence, historical/current separation | A malicious approved build can lie about its own registry. |
| Malicious hosted JavaScript/WASM | HTTPS, self-hosted assets, strict CSP where supported, no third-party scripts, independent release checks | A compromised origin can replace both app and in-page integrity checker. |
| XSS or malicious imports | Text rendering, strict input schemas, no HTML interpretation, bounded parsing | Browser extensions and device compromise are outside app control. |
| Resource exhaustion | Bounded batch size and file size, cancellable long jobs, no initial public compute API | Static host availability remains provider-dependent. |
| Selection privacy | No accounts, analytics, server generation or selection telemetry | Host access logs and the user's own device remain external data surfaces. |

Use owner-controlled MFA/passkeys and recovery methods. Do not put signing secrets into the repository, chat, browser bundle or agent workspace. Pull-request CI has read-only permissions and no deployment credentials; untrusted forks must not run privileged workflows. Avoid executing untrusted code under `pull_request_target`. Deployment uses only the necessary scoped permissions on an approved release.

The owner should verify checksums/signatures through an independent trusted channel and retain a known release locally. A checksum published only beside a compromised binary is weak evidence. Signatures demonstrate a signer and bytes, not fairness. Reproducible builds are a later goal requiring a recorded toolchain and independent rebuild comparison, not merely a successful CI job.

For web delivery, use a restrictive policy allowing only required self-hosted scripts and WASM execution; avoid inline scripts and broad `unsafe-eval`. Test actual WASM/CSP compatibility. Controls requiring response headers, such as `frame-ancestors`, require a host that supports those headers; a meta CSP cannot substitute for every header directive. Host capability gaps must be documented or resolved before claiming the corresponding control.

## 10. Professional standards and assurance claims

WLA-SCS:2024 includes controls concerning RNG and draw protection, transmission integrity, independent verification and segregation of duties [S6]. DrawLab adopts those ideas proportionately as design references. It is not WLA-certified and does not reproduce all operational controls of a licensed lottery.

NIST SP 800-90C was finalized in September 2025 and connects random-bit-generator constructions with the 800-90A mechanisms and 800-90B entropy-source framework [S7]. Use it to understand the trust chain. Calling a platform random API is not proof of NIST or FIPS validation. Do not build a custom entropy source or claim to have measured its min-entropy from ordinary output samples.

OWASP ASVS supplies a structured security verification reference [S11]. During implementation, pin a specific edition and record applicable requirements, evidence and justified exclusions. Do not assert whole-standard compliance from a short checklist.

Maintain an assurance matrix linking each claim to a mechanism, test or review, and residual assumption. Examples: uniform selection → sampler proof plus bounded-integer tests; rule accuracy → authoritative clauses plus conformance fixtures; release identity → manifest plus independent verification. None substitutes for the others.

## 11. Verification plan and release gates

| Gate | Evidence required |
|---|---|
| Rule conformance | Every enabled field traced to authoritative source and effective scope; current-versus-historical conflicts resolved. |
| Bounded sampling | Reduced-width exhaustive test, including rejection tail; boundary and overflow tests; mocked entropy failure; retry exhaustion. |
| Uniform subset logic | Independent exhaustive small-n permutation/combination oracle; mutation tests catch biased modulo and incorrect bonus selection. |
| Invariants | Property tests for valid ranges, counts, uniqueness, order, independent-pool overlap and round-trip serialization. |
| Shared implementation | CLI and WASM deterministic fixtures agree using test-only adapters; production exports cannot activate seeded mode. |
| Probability engine | Exact table values above, normalization of match distributions, invalid-parameter and arithmetic overflow tests. |
| User workflow | Fresh checkout builds; CLI and browser generate/export/verify; keyboard and narrow-screen checks; visible error states. |
| Security | Threat-model review, dependency/advisory and secret scans, bounded import fuzzing, protected release settings checked where available. |
| Packaging | Versioned binaries/static assets, checksum manifest, SBOM, changelog, known limitations, reproducible build instructions. |
| Coverage | Signed-off catalogue scope; no unimplemented mandatory number-draw profile hidden by an “all games” label. |

Suggested implementation commands are `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --locked`, supplemented by actual WASM builds, browser tests and pinned security tools. These commands are planned gates, not completed tests of an existing application.

Performance targets to benchmark: one board feels immediate, 100 boards complete within one second on the declared reference laptop, and initial static assets remain small enough for ordinary mobile connections. Record hardware/browser/build conditions. These are acceptance targets, not current measured results.

## 12. Repository organization and delivery units

| Path | Responsibility |
|---|---|
| `README.md` | Purpose, quick start, limitations and current coverage |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Pinned workspace and native toolchain |
| `crates/drawlab-core/` | Pure mathematics and validators |
| `crates/drawlab-rules/` | Registry loader and cross-field checks |
| `crates/drawlab-cli/` | Native interface |
| `crates/drawlab-wasm/` | Browser binding |
| `web/` | Static TypeScript application and locked frontend dependencies |
| `rules/`, `schemas/` | Profiles, evidence index and versioned data contracts |
| `tests/`, `fuzz/` | Independent fixtures, conformance and malformed-input testing |
| `docs/design-v1.0.0.md` | This baseline |
| `docs/mathematics.md`, `docs/threat-model.md` | Proofs, assumptions and security boundaries |
| `docs/rules-evidence.md`, `docs/coverage.md` | Sources, unresolved items, profile support |
| `docs/adr/`, `docs/runbooks/` | Decisions, releases, rollback and incidents |
| `SECURITY.md`, `CHANGELOG.md`, `CONTRIBUTING.md` | Reporting, history and contribution gates |
| `.github/workflows/` | CI and approved release automation |

Delivery sequence:

1. **Rules and contracts:** acquire documents, resolve the catalogue, define source locators and typed schemas. Output: reviewable coverage register and conformance fixtures.
2. **Working native core:** unbiased sampler, entropy adapter, exact odds, validator, CLI and tests. Output: one full verified family working locally.
3. **Number-game completion:** add remaining verified families and add-on relationships. Output: coverage report and all mandatory profile tests.
4. **Browser integration:** WASM bindings, small UI, accessible export and errors. Output: the same operations work locally in a browser.
5. **Security and release:** review, lock dependencies, test clean builds, package artifacts and tag 0.1.0 only after gates pass.
6. **Optional hosting:** publish reviewed static assets with the selected provider after confirming visibility, policy and controls.

Each unit uses a short-lived branch and a reviewable PR. Implementation progress should not require keeping the entire project unfinished until every research extension is ready. Missing official rules are a concrete dependency, not an excuse to make up behavior.

## 13. Version roadmap

| Version | Completion boundary |
|---|---|
| Design 1.0.0 | Architecture, mathematical specification, evidence gaps, security model and release criteria recorded. |
| Software 0.1.0-alpha.x | Honest preview of implemented verified profiles while catalogue or other gates remain open. |
| Software 0.1.0 | End-to-end CLI and browser number-game scope, exact basic odds, provenance, exports and baseline security gates. |
| Software 0.2.0 | Verified multiplay expansion, explicit distinct-board mode, richer match probabilities and reproducible research executable. |
| Software 0.3.0 | Reviewed additional game adapters where source data permits; simulation confidence reporting and bounded research jobs. |
| Software 0.4.0 | Independently checked build provenance and experimental public audit protocols if justified. |
| Software 1.0.0 | Stable documented interfaces, complete declared coverage, independent mathematics/security review, release/recovery rehearsal and no unresolved critical defects. |

Rule profiles and output schemas have their own versions. Never relabel existing output under a new profile. Security fixes and behavior changes are documented even when the visible UI does not change. A rule correction requires a new profile and explicit notice identifying affected outputs.

A public verifiable-draw research protocol would need precommitted inputs, domain separation, participant assumptions, reveal deadlines, abort handling, protection against selective redraws, and independent timestamp/publication evidence. A simple hash chain, public random beacon or commit/reveal demonstration is not automatically a fair or secret lottery system. Keep that research separate from normal generation.

## 14. Deployment, costs and operations

Start with local execution and static hosting if desired. Generating dozens of lottery boards has negligible computation requirements; a VM adds administration without solving the current problem. Free hosting has eligibility, policy, bandwidth and build limits and is not a lifetime guarantee [S10]. Review conditions at deployment. Configure no paid service or billing upgrade automatically.

If future experiments require compute, first benchmark bounded Rust jobs locally. A later service can use a small Rust HTTP layer with strict job quotas, deadlines, cancellation and cost limits, but it needs a new threat model. A free VM is an option only after verifying current eligibility and operational requirements.

Release runbook: review source/rule diff → pass gates → build in pinned environment → produce manifest and evidence → owner-controlled release approval → tag and publish immutable version → verify installed/served bytes → retain previous release.

Incident runbook: disable affected generation/profile or remove the affected hosted release → preserve evidence → identify affected versions → fix and independently review → publish advisory → require explicit update. Rollback must not silently restore stale rules as current. Local installations cannot be centrally disabled; publish a clear affected-version notice and replacement instructions.

## 15. Business boundary and originality

Professional lottery operations include ticket integrity, controlled sales closure, audited draws, payout accounting, independent oversight and incident management. A number generator is only one component. NLC oversight includes inspection of draws and equipment and auditing operational financial flows [S12]. Building DrawLab does not grant operating permission. Any later sale of entries, handling of stakes or award of prizes requires a separate legal and regulatory assessment before implementation.

The intended professional value for GyLiber is demonstrable software engineering: mathematically justified sampling, evolving rule schemas, secure delivery and transparent verification. Potential future products are educational tools, testing harnesses or simulation services; no income claim is made.

Originality is a design goal, not a verified worldwide exclusivity claim. The proposed distinction is a source-linked “why this output is valid” explanation with exact mathematics and explicit uncertainty. A competitor and intellectual-property review would be required before claiming the product is one of a kind.

## 16. Outstanding evidence and implementation decisions

| Item | Required resolution | Blocks |
|---|---|---|
| Full later ITHUBA retailer manual | Obtain readable authoritative copy; extract clauses and effective scope | Corresponding profiles and broad ITHUBA completeness claim |
| Current operator rule set | Resolve ranges, draw semantics, official product names, LOTTO 5 Max and add-ons | Current-game verification |
| Complete catalogue | Reconcile active, historical, sports, raffle and instant entries | Literal “all games” claim |
| Additional products | Obtain their outcome/fixture/prize specifications | Each affected adapter, not unrelated verified profiles |
| GitHub repository | Owner creates repository and supplies URL; verify available write route | Remote implementation and PRs |
| Independent reviewer | Identify reviewer without relying solely on the author/assistant | Independent-assurance claims and software 1.0.0 gate |
| Publication/licensing | Owner selects visibility and license before public release | Public distribution, not private implementation |

No software repository, release, hosted service or security certification is created by this document. GitHub write capability must be verified when the repository exists; do not promise direct pushes from a read-only connector. Use a legitimate authorized write route or deliver a precise patch if access is unavailable.

## 17. Primary references and evidence notes

All references checked on 5 October 2026. URLs may change; archive precise editions and locators during rule implementation. This document paraphrases standards and source material; it does not reproduce the standards or operator manuals.

- **S1 — National Lotteries Commission, 1 June 2026 statement:** https://www.nlcsa.org.za/wp-content/uploads/2026/06/Media-Statement-NLC-Welcomes-Sizekhaya-as-the-Fourth-National-Lottery-and-Sports-Pools-Operator.pdf . Regulator evidence of operator transition.
- **S2 — Capitec, lottery product transition:** https://www.capitecbank.co.za/personal/transact/lotto/ . First-party participant information; naming inconsistencies prevent use as the sole rules authority.
- **S3 — National Lottery, indexed ITHUBA retailer manual:** https://www.nationallottery.co.za/assets/documents/Retailer_Manual.pdf . Indexed 6/58 clause; direct retrieval returned 403. Not a completed source inspection.
- **S4 — ITHUBA, historical retailer manual (2019):** https://ithubalottery.co.za/wp-content/uploads/2019/05/Retailer-Manual-Edit-29-May-2019.pdf . Historical primary source; requires clause-by-clause extraction before enabling associated profiles.
- **S5 — Indexed ITHUBA product catalogue:** https://www.nationallottery.co.za/?s= . Historical catalogue discovery only; crawl predates the operator transition and does not establish current availability.
- **S6 — World Lottery Association, WLA-SCS:2024:** https://publications.world-lotteries.org/security-and-risk-management/wla-security-control-standard . Especially RNG controls in L.8. See also https://world-lotteries.org/services/industry-standards/security-and-risk-management-2/security-standard-2024 . Design reference, no certification claim.
- **S7 — NIST, final SP 800-90C publication announcement, 25 September 2025:** https://www.nist.gov/news-events/news/2025/09/recommendation-random-bit-generator-constructions-nist-publishes-sp-800-90c . Modern random-bit-generator framework; platform validation not established here.
- **S8 — Rust `getrandom` documentation:** https://docs.rs/getrandom/latest/getrandom/ . Platform backends and browser-target configuration; replace “latest” with the selected version in implementation records.
- **S9 — W3C Web Cryptography specification:** https://www.w3.org/TR/webcrypto/ . Retrieved page identifies itself as Web Cryptography Level 2, First Public Working Draft, 22 April 2025; treat that edition as a draft, not a finalized Level 2 standard. Consult the established API and target implementations for `getRandomValues` behavior.
- **S10 — GitHub Pages documentation:** https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages . Static hosting availability and plan/visibility conditions; recheck usage restrictions before publishing.
- **S11 — OWASP ASVS:** https://owasp.org/projects/asvs . Pin applicable edition and requirements at implementation.
- **S12 — NLC oversight description:** https://www.nlcsa.org.za/overseeing-national-lottery/ . Operational oversight context; its older operator history is not used to establish current operator identity.

## 18. Immediate handoff

Create an empty private GitHub repository named **`gyliber-drawlab`**, with description:

> GyLiber's rule-versioned lottery simulation laboratory: unbiased generation, exact mathematics, and verifiable engineering.

Provide the repository URL. The next unit is repository scaffolding plus authoritative rule evidence and one complete verified native workflow. Keep the repository empty if convenient; README and other project files can be introduced together in the first reviewed change. Do not share passwords or personal access tokens in chat.

The intended outcome is a working, modest first release with correct foundations, followed by independently justified improvements. Advanced research strengthens understanding and assurance; it does not replace verified rules or honest claims.
