# Alpha.2 hosting preparation

Date: 2026-10-09. Baseline: 80a4047239315b222c4ee3c54c9ccbfee6b7464a.
The baseline GitHub run 37840399323 passed both verification and advisory jobs.

This packet adds static hosting configuration, version/commit identity, one browser
integration test and the shared Render/CI build path. Mathematical models and
sampling algorithms are unchanged. The version bump intentionally preserves the
existing strict record-version policy; earlier records need the earlier verifier.

Local checks: shell syntax, whitespace, package/version consistency and formatting.
Validate render.yaml against the official Render JSON schema before publication.
Application/native/browser and advisory results for this head must be read from
PR #1's checks, not inferred from the baseline. CI runs five browser tests.

Provider-specific bootstrap, HTTPS response headers, live browser behavior and
client acceptance remain pending owner Blueprint setup. No deployed URL, stable
release, independent security audit or completed official-game support is claimed.
Follow ../runbooks/HOSTING.md and ../NEXT-STEPS.md for the exact remaining actions.
