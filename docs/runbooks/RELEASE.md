# Release and client demonstration

Every minor release must deliver a demonstrable capability, not only scaffolding.
Keep minor versions prerelease until all promised scope gates pass.

1. Create a focused PR with Conventional Commits and a current coverage table.
2. Run Rust format, lint, locked tests, browser production build, browser tests
   and both dependency scans. Attach the actual evidence and limitations.
3. Demonstrate CLI generation → export → verification and browser generation →
   download → import. Demonstrate invalid-record rejection and disabled rules.
4. Review source/rules/security changes; obtain owner/client acceptance.
5. Tag the exact accepted commit and publish checksum-indexed artifacts with
   versions, limitations and build instructions. Do not retag released history.
6. For hosting, review provider policy, headers and HTTPS, then publish the
   reviewed static artifact. There is no need for a VM in this version.

For alpha.1, full official rule verification and scope completion remain blockers
to stable 0.1.0. Do not upgrade the version merely because generic models work.

CI produces a static preview artifact, not a public deployment. Extract it and
serve over localhost using a trusted static server. file:// is unsupported.
The source build route is documented in README. Keep older releases and their
rule registry versions together; verification compatibility is version-specific.

## Automated laboratory prereleases (from v0.2.0-alpha.1)

Owner authorization on 2026-10-12 permits autonomous laboratory publication;
it does not substitute for client testing or stable official-game gates.
CI packages the tested static site and Linux x86_64 CLI with notes, LICENSE
and SHA256SUMS. The main-only publisher depends on verification and advisory
audit. Only this job has contents:write, using the ephemeral repository token;
PR jobs remain read-only. No personal deployment token is needed.

After live checks, a reviewed release-intent.json specifies version,
prerelease:true and deployed_commit. The publisher checks live HTTPS identity,
downloads this run's bundle, verifies hashes, creates a draft at the CI commit,
and publishes it as a prerelease. No intent means no publication. Public releases
are never overwritten; an existing draft fails for investigation.

The live commit may precede the tag by documentation/intent-only commits; record
both. Checksums detect changed bytes, not authenticity or certification. Source
maintainers and GitHub/Render administrators remain trust boundaries. Remove the
old intent when beginning a future version. The publisher does not deploy Render
or enable official profiles.
