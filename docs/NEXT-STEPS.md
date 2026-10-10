# DrawLab remaining work and resumption

Updated 2026-10-10. Client: Gyile / GyLiber.
Current software: 0.1.0-alpha.2 laboratory preview, not complete official-game 0.1.0.
Deployment record: docs/verification/HOSTED-PREVIEW-2026-10-09.md.
Render reports https://gyliber-drawlab.onrender.com live at commit 0a970a0.
Release-fix review: https://github.com/GyLiber/gyliber-drawlab/pull/8 (merged).
No current feature branch is designated here; inspect main and open PRs on resumption.
Never import branch/version markers from another repo.

## Resume safely

1. Read the remote branch/PR state, recent commits and local working-tree status.
2. Read this file, coverage.md and the latest verification record. Check for new
   owner-supplied sources before repeating any retrieval attempt.
3. Verify the current head's CI and actual repository protections. Preserve local
   changes; do not force-push or reset away work after an interrupted session.
4. Select one packet below, finish its verification and record the next checkpoint.
5. Stop; wait for continue before beginning another packet.

## Next boundary: client review of the hosted preview

Render reports the static preview live, with exact deployed SHA and successful
GitHub CI recorded in verification/HOSTED-PREVIEW-2026-10-09.md. There is no
need to recreate the service or redeploy the same commit. Public HTTPS, exact
build identity, response security headers, live engine load and desktop
generate/export/import/invalid-record checks passed on 2026-10-10; see
verification/LIVE-BROWSER-2026-10-10.md for the tested models and limits.
Gyile now reviews the preview and accepts it or reports a specific defect.
Live mobile acceptance has not been run; prior automated mobile checks are
separate evidence. Do not equate developer smoke tests with client acceptance
or label this stable 0.1.0. Stop at each completed batch checkpoint.

## Following packet: authoritative rule acquisition and extraction

The old content-host links are blocked for the owner; do not ask him to retry or
bypass security. No further manual source retry is requested. Seek a working
authoritative publication or a lawfully supplied operator/regulator copy in a
bounded search. If unavailable, prepare a precise document request for Gyile's
review, with the recipient independently verified; do not send messages without
explicit authorization. Stop at that boundary instead of inventing game rules.

For any recovered document, record provenance, file digest, edition, applicability
and page/section references. Distinguish historical ITHUBA from current rules.
Do not publish entire third-party manuals without permission. Deliver a compact
conformance table covering main pools, extra pools, replacement, selection versus
draw behavior and PLUS relationships. Clearly mark unresolved clauses.

## Later packets, each separately authorized by continue

1. Implement the first fully evidenced game family with conformance fixtures and
   CLI/browser demonstrations. Keep unsupported profiles disabled. Extend family
   by family; reconcile the complete catalogue and non-ball product scope before
   claiming all-game coverage.
2. Complete remaining stable-release gates from runbooks/RELEASE.md and ALPHA-1.md:
   record compatibility/metadata, further correctness/security evidence,
   independent review and client acceptance. Record gaps, not blanket certification.
3. Prepare an accepted versioned client release after the remaining gates pass.
   No stable tag or release exists. Record hosting status only after live checks.

## Completed owner actions and current blocker

Main ruleset was independently verified active on 2026-10-08. Vulnerability
reporting and account protection were reported complete by the owner. See
verification/SOURCE-RECOVERY-2026-10-08.md for evidence boundaries.
The stable 0.1.0 blocker is authoritative full game-rule evidence (plus
conformance/review/acceptance), not compute capacity or an API account.
The immediate alpha.2 finishing gate is client acceptance; scoped desktop
HTTP/browser verification is complete. Avoid repeating passed checks without
a changed deployment or specific remaining risk.
Preserve the working laboratory preview.
