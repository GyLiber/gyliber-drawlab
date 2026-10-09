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
