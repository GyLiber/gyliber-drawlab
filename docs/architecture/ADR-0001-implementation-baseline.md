# ADR-0001: Preserve the Rust design baseline

Status: Accepted for initial implementation, 2026-10-05.

The client said no revision was needed and supplied the repository after a
comparison of Rust and TypeScript. Implementation retains the saved Rust design;
this interpretation was stated before work began. Rust core and CLI, with a
narrow WASM browser binding, preserve one implementation of sampling and odds.
This entails two toolchains and does not itself guarantee security. A later
change needs measured justification and a new ADR, not a rewritten history.

The design v1.0.0 is an immutable proposal snapshot, not an assertion that all
features already exist. README and verification records describe actual status.
