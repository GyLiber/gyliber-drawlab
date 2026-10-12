# DrawLab release status and next actions

Updated 2026-10-12. Client: Gyile / GyLiber.

## Current delivery

v0.2.0-alpha.1 adds exact cumulative main-match probabilities across Rust,
CLI and browser, plus probability CSV and checksum-indexed release packages.
Live source: ff89c98216e7ef90d506c6c9eff6583e35fa2c6f.
See [live evidence](verification/V0.2-LIVE-2026-10-12.md) and
[release notes](releases/v0.2.0-alpha.1.md).
The reviewed release marker enables gated publication; the public GitHub release
records the final tag SHA and package inventory. Do not republish this version.

Gyile authorized autonomous development, deployment and the next minor release
on 2026-10-12, suspending stop/continue checkpoints for this run. This is release
authorization, not evidence of manual client acceptance. Preserve the mint/slate
palette and keep unsupported official profiles disabled.

## Publication verification

The release-marker PR must pass both required checks before merge. Main CI then
publishes v0.2.0-alpha.1 automatically. Confirm the public prerelease, exact tag
commit and five assets (two archives, notes, license and checksums). If interrupted,
inspect the release and CI before retrying; never overwrite a tag or public asset.

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
