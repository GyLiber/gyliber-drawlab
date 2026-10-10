# Hosted alpha.2: desktop browser verification

Date: 2026-10-10. Scope: one bounded live-verification/documentation batch.
URL: https://gyliber-drawlab.onrender.com/
Version: 0.1.0-alpha.2.
Deployed SHA: 0a970a046adca28f8d9ce8ecf4c9080a80d11bae.
Render deployment: dep-db483mjncjis73c4rik0 (reported live).
Repository baseline: 61102bfe95e9a131c553191b90de17f660103b2b.
No application change or redeployment was necessary.

## Observed checks

| Check | Observed result |
|---|---|
| HTTPS GET / | HTTP 200; HTML content type |
| HTTPS GET /build-info.json | HTTP 200; exact deployed SHA, alpha.2, laboratory-preview |
| CSP response header | self-only script/assets; wasm-unsafe-eval; frame-ancestors none; no unsafe-inline or unsafe-eval |
| Other response headers | nosniff, X-Frame-Options DENY, no-referrer, device permissions denied, Cache-Control no-cache and HSTS present |
| Live browser engine | Loaded; version displayed and controls enabled |
| 5-of-36 selection | Two boards generated |
| JSON export/import | Download completed; re-import showed Valid: 2 board(s), lab-5of36 |
| Invalid record | Export modified to duplicate a main number; live import rejected with INVALID_RECORD |
| 6-of-58 simulated draw | Two draws displayed six distinct mains and a bonus absent from each main set |
| 5-of-50 plus 1-of-20 draw | Two draws displayed independent extra balls; odds showed 1 in 42,375,200 |
| Unsupported official rules | Three disabled candidate notices remained visible; only laboratory models offered |

HTTP checks used certificate-verifying HTTPS requests. Browser checks used the
actual hosted interface in the cloud Chrome browser, including file download and
file-picker import. No internal application function was called to simulate a
successful UI action. A viewport screenshot was captured and inspected.

## Limits and remaining acceptance

This is observed developer smoke-test evidence, not client approval, proof of
randomness, independent security review or complete official-game conformance.
Only the three models above were exercised in this live session; exhaustive
model/mode and mobile/keyboard checks remain recorded separately in CI evidence.
No mobile live session or fresh all-model live sweep is claimed. A full-page
screenshot timed out; the viewport capture succeeded. The export/import tool call
was slow but returned a valid imported record; this does not measure app latency.

Next: Gyile reviews the live preview and accepts it or reports a specific defect.
If a mobile live acceptance check is needed, perform it as a bounded follow-up.
Official rule acquisition, independently reviewed fixtures and the stable-release
gates remain in ../NEXT-STEPS.md. Do not label this stable 0.1.0.
