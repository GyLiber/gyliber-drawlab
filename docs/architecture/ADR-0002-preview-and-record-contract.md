# ADR-0002: Preview scope and record contract

Status: Accepted, 2026-10-05.

Official source locations were found, but full PDF retrieval returned 403, 502
or timeout. Search snippets do not satisfy the design's rule-verification gate.
Five explicit laboratory models are enabled. Three ITHUBA candidate families
are visible but disabled in the core, not merely hidden by the UI.
Current operator profiles, retired games, sports pools, instant games and
raffles are unimplemented. Version remains 0.1.0-alpha.1.

Records use schema 1 and strict unknown-field rejection. They contain the exact
profile, a SHA-256 digest of compact serde JSON in declared struct field order,
software version, mode, entropy-backend label and boards. Verification compares
against the bundled registry, never against an imported profile's authority.

This alpha accepts only its exact software version; cross-release compatibility
must be specified before a stable format. It does not preserve draw order,
timestamps, build commit, signatures or add-on relationships yet. These are
explicit gaps against the full design, not claimed features.

No production deterministic-seed entry point exists. Test random streams remain
inside the private sampler module. A mathematically valid edited record can
still pass verification: the verifier does not authenticate or prove randomness.
