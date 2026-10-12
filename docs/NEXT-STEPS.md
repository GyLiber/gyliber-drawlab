# DrawLab release status and next actions

Updated 2026-10-12. Client: Gyile / GyLiber.

## Current delivery

v0.2.0-alpha.1 adds exact cumulative main-match probabilities across Rust,
CLI and browser, plus probability CSV and checksum-indexed release packages.
Previous live revision: 25b8e342c3102c19d57e847c9b0a4b2c0857a67e
(0.1.0-alpha.3). See verification/LIVE-ALPHA-3-2026-10-11.md.
New-version publication is in progress; consult GitHub release state.

Gyile authorized autonomous development, deployment and the next minor release
on 2026-10-12, suspending stop/continue checkpoints for this run. This is release
authorization, not evidence of manual client acceptance. Preserve the mint/slate
palette and keep unsupported official profiles disabled.

## Finish this release

1. Pass required PR CI and audits; merge without bypassing protections.
2. Deploy gated main to the existing Render site in My Workspace.
3. Verify live identity, thresholds, CSV and existing browser workflows.
4. Commit live evidence and release-intent.json through a checked PR.
5. Main CI publishes tested packages at the exact CI commit as a prerelease.
   Verify public release, tag and asset inventory.

## Following work

1. Collect client feedback using the demonstration in the release notes.
2. Obtain authoritative complete game rules. The prepared request remains unsent.
   Do not retry the blocked host or weaken TLS. Record provenance, digest,
   edition, applicability and section references for every new source.
3. Implement one evidenced game family with conformance fixtures, then expand.
   Do not label generic models as official games or claim all-game support.
4. Review dependency PRs separately from the release marker.
5. Remove the old release intent before a new version; update compatibility
   documentation. Never replace published tags or assets.

Resume by inspecting main, PRs, releases, live identity and local changes.
Preserve interrupted work. Normal bounded checkpoints resume after this release
unless Gyile directs otherwise. Stable gates still require verified rules,
conformance, independent review and explicit client acceptance.
