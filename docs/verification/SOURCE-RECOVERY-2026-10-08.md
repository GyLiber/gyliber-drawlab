# Source recovery and owner controls: 2026-10-08

Scope: one documentation/evidence batch, with a maximum 30-minute session budget.
Baseline: feat/drawlab-foundation at
0a4c8a703d4564de45c568744bbc00163dd76048; PR #1 open and unmerged when inspected.
Software remains 0.1.0-alpha.1. No profile, code, dependency or release change.

## Evidence and limits

| Observation | Evidence | Consequence |
|---|---|---|
| Three 2025 content-host PDF links refused by Firefox | Owner report on 2026-10-08; warning about old server software | Suspend manual download request; no security bypass |
| Active main-protection ruleset | GitHub API /repos/GyLiber/gyliber-drawlab/rulesets/24749025 | PR/check/deletion/force-push controls confirmed; no bypass actors |
| Private vulnerability reporting and account protection completed | Owner report only | Do not claim independent API verification |
| Alternative official rule sources unavailable | Retrieval results below | Official profiles remain disabled |

Ruleset requires Core and browser verification and Dependency advisory audit from
integration 15368 (GitHub Actions), strict up-to-date checks, resolved review threads
and PRs. Zero required approvals reflects a single maintainer, not external review.

## Bounded source search

HTTPS search/retrieval was limited to official operator domains. No certificate
checks were disabled, HTTP downgrade used or access errors circumvented.

| URL | Result on 2026-10-08 |
|---|---|
| https://www.nationallottery.co.za/ | Retrieval tool reported 403 |
| https://www.nationallottery.co.za/assets/documents/Retailer_Manual.pdf | Retrieval tool reported timeout |
| https://ithubalottery.co.za/wp-content/uploads/2019/05/Retailer-Manual-Edit-29-May-2019.pdf | Retrieval tool reported timeout |
| https://www.nationallottery.co.za/images/docs/LOTTO-Rules-and-Regs-V10-26218.pdf | Search indexed it; direct retrieval reported 403 |
| https://www.nationallottery.co.za/Participants%20Code%20of%20Practice%20Version%201.1.pdf | Search indexed it; direct retrieval reported 403 |
| https://www.nationallottery.co.za/Sizekhaya_Website_Mobile_App_Terms_Conditions_v3.0.pdf | Search returned platform terms referring to separate game rules; not a replacement game specification |

These are retrieval observations, not proof of global website availability or a
diagnosis of the owner's TLS warning. Search-index text is discovery evidence,
not an authenticated local PDF. No source-file hash or conformance claim is made.
Platform terms do not establish ball pools, replacement rules or add-on semantics.

## Verification and checkpoint

Review this commit's diff and relative Markdown links. The prior application
verification remains recorded in ALPHA-1.md; this batch does not add runtime
behavior. GitHub CI is authoritative for the new head, accessible through PR #1.
The commit containing this document is the batch checkpoint; use its Git SHA
instead of embedding a self-referential hash. Final chat handoff records that SHA
and observed CI state. Next work is in ../NEXT-STEPS.md.
