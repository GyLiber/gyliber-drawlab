# Alpha.1 verification and client acceptance

Client: Gyile / GyLiber. Work started 2026-10-05; browser verification 2026-10-06.
Scope: laboratory preview. Stable 0.1.0 and client acceptance remain outstanding.

## Observed checks

| Check | Observed result |
|---|---|
| Rust format / Clippy with warnings denied | Passed |
| Core tests | 8 passed: bounded sampler, exhaustive toy subset space, failure injection, bonus behavior, exact arithmetic and records |
| CLI integration tests | 2 passed: generation/verification and fail-closed requests |
| WASM and TypeScript production build | Passed; about 205 KB total uncompressed |
| Browser integration tests | 4 passed locally on Chromium 143.0.7499.0 |
| Native/browser interoperability | Browser-generated record accepted by native CLI |
| Failure paths | Duplicate-number import rejected; entropy exception produces no result/export |
| Mobile/keyboard behavior | Passed at 390px width; desktop/mobile screenshots inspected |
| Cargo advisory scan | Passed against 1,290 loaded advisories on 2026-10-05 |
| npm audit | No vulnerabilities reported on 2026-10-05; CI repeats against locked dependencies |
| GitHub CI | See PR checks for the exact current commit; do not infer remote success from local results |
| Independent review / client acceptance | Not performed |

## Browser environment recovery

Standard Playwright Chromium downloads returned invalid archives in the local
execution environment. Local browser tests used Chromium 143.0.7499.0 from
@sparticuz/chromium 143.0.4 installed outside the repository. The binary and
graphics libraries were extracted without modifying the application. Test launch
used no-sandbox/no-zygote for the container and software graphics; web security
was not disabled. A first single-process launch failed when a second browser
context was created; removing single-process resolved the infrastructure fault.
No test assertions were weakened. CI uses the normal Playwright browser install.
No alternate browser dependency ships in the product.

## Acceptance demonstration

1. Generate two lab-5of36 selections in the CLI; export JSON and verify it.
2. Generate a remaining-bonus draw using lab-6of58.
3. Show exact 1/42,375,200 match probability for lab-5of50-20.
4. In the browser, generate, export and re-import a record.
5. Alter a number to duplicate another main number and demonstrate rejection.
6. Show disabled official candidates and explain the outstanding rule evidence.

These are client-reviewable laboratory capabilities. They do not constitute
delivery of all official game coverage or client acceptance of stable 0.1.0.

## Remaining release gates

Full official sources, reviewed conformance fixtures and catalogue scope;
add-on semantics; schema compatibility and complete metadata; further property/
fuzz/mutation evidence; independent review; owner-enabled branch protections;
production hosting/header review if deployment is requested.
