# Client demonstration: DrawLab alpha.2

Prepared 2026-10-10 for Gyile / GyLiber. Meeting duration: 8–10 minutes.
Status: ready for client review; acceptance has not been recorded.

## Outcome to demonstrate

A client can open the hosted laboratory, generate selections or simulated draws,
export a JSON record and validate it through the browser. The delivered software
is **0.1.0-alpha.2**, not stable 0.1.0 and not an official lottery system.
This batch packages evidence and a repeatable meeting; it adds no runtime feature
and does not warrant a version bump. No spend, return-on-investment or completion
percentage is inferred from commit counts.

Preview: https://gyliber-drawlab.onrender.com/

Deployed application commit: `0a970a046adca28f8d9ce8ecf4c9080a80d11bae`.
Documentation baseline: `4298c6f31bb62caf1b6c81f2c82f58b8011fa5ff` (PR #10).
The deployed application and documentation revisions differ intentionally.

## Presenter preparation

1. Open the preview and wait for the engine to enable generation. Confirm version
   alpha.2; open `/build-info.json` to compare the full deployed commit above.
2. Keep this guide and the [live evidence](../verification/LIVE-BROWSER-2026-10-10.md)
   available. Use an empty downloads folder or clearly named demonstration folder.
3. If the site fails, record the URL, time, browser and displayed error. Present
   the dated evidence as historical evidence; do not describe it as a live pass.
   Do not weaken TLS or browser security settings.

## Demonstration script and expected observations

| Time | Action | Expected observable result / explanation |
|---|---|---|
| 0–1 min | Show version, five laboratory models and disabled official candidates | Honest scope is visible before any generation |
| 1–3 min | Select 5/36, selection mode, board count 2; generate | Two boards; five distinct integers from 1 through 36 on each. Different boards may legitimately overlap or repeat |
| 3–4 min | Export JSON; import that same file | Validation reports two valid boards. This checks the record format and mathematical constraints, not authenticity or lottery certification |
| 4–5 min | Select 6/58, draw mode, count 2; simulate | Each board has six distinct main values and one bonus value outside that board's main set |
| 5–6 min | Select 5/50 + 1/20 and generate | Five distinct main values and an independent extra from 1–20; exact full-match odds 1 in 42,375,200. The extra may equal a main value |
| 6–8 min | Show the evidence and remaining-delivery table below | Client sees delivered behavior, verification limits and a concrete next gate |

Optional rejection demonstration: copy the exported 5/36 JSON, replace the first
board's `main` array with `[1,1,2,3,4]`, save separately and import the copy.
Expect `INVALID_RECORD`. Keep the original. This rejects an invalid board; it is
not proof that all valid-looking tampering can be detected.

Do not require a particular random sequence or treat visually patterned numbers
as failures. Simulation does not predict future winning numbers.

## Evidence and remaining delivery

| Deliverable | Evidence / acceptance boundary |
|---|---|
| Working hosted generation/export/import | [Live desktop checks, 2026-10-10](../verification/LIVE-BROWSER-2026-10-10.md); scoped checks passed |
| Shared Rust core, CLI and browser | [Alpha.2 verification](../verification/ALPHA-2.md), repository tests and CI |
| Sampling and exact probabilities | [Mathematical specification](../mathematics/SAMPLING.md); tests are evidence, not certification |
| Reviewable changes and dependency checks | [PR #10](https://github.com/GyLiber/gyliber-drawlab/pull/10); both pre-merge CI runs passed |
| Official ITHUBA game accuracy | Blocked on full applicable rule sources, clause extraction and conformance fixtures; [coverage](../coverage.md) |
| Client approval / independent security review | Not completed; developer tests do not substitute for either |

## Decision record to complete at the meeting

Record the date, reviewer and exact version/commit shown. Select one outcome:
**accepted as laboratory preview**, **accepted with listed follow-up items**, or
**changes required**. Leave it pending until the client explicitly responds.
For each issue record the steps, expected and actual behavior, and its priority.
Acceptance of this preview does not accept stable 0.1.0 or official-game coverage.

## Next measurable delivery

First obtain one authoritative game family's complete applicable rules, then
produce a page-referenced conformance table and independently reviewable test
fixtures. Only after that gate passes, implement the family through Rust, CLI and
browser and demonstrate valid generation, export/import and rejection cases.
Keep other families disabled. Dates remain uncommitted until source access and
scope are resolved; see the [source request](RULE-SOURCE-REQUEST.md).

If no source arrives, stop source implementation and select a bounded task with
independent value, such as resolving one dependency update or a reported UI defect.
Do not repeatedly spend sessions searching the same inaccessible URLs.
