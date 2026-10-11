# Alpha.3 candidate: main-match probability explorer

Prepared 2026-10-11 for Gyile / GyLiber. Candidate implementation, not a deployment
or client-acceptance record. Hosted alpha.2 remains unchanged until deployment.

## New client-visible outcome

Change the laboratory model and see the probabilities of exactly 0 through k main
matches against one independent uniform draw. A visual bar, rounded percentage,
exact fraction and exact total make the underlying calculation inspectable.
The existing generation → export → validation workflow remains available.

## Two-minute demonstration after deployment

1. Verify alpha.3 build identity. Select 5/36 and find “How many main numbers might match?”.
2. Read exactly 0 matches as 169911 / 376992, exactly 5 as 1 / 376992,
   and the exact total as 376992 / 376992 = 1. Expand “Why these probabilities?”.
3. Switch to 6/58: seven rows replace six; the all-main fraction is 1 / 40475358.
4. Compare 5/50+1/20 with 5/50+1/16. Main-match distributions agree, while the
   full-match odds elsewhere differ because they include the independent extra.
5. Change operation and board count: neither changes this one-selection distribution.
6. Generate two selections, export JSON and import it. Alpha.3 records need the
   alpha.3 verifier; keep earlier records with their corresponding earlier version.

## Mathematical contract

For one fixed k-subset S of an n-element pool and a uniformly sampled independent
k-subset D, R = |S ∩ D|. Exactly r matches can occur in C(k,r)C(n−k,k−r) draws,
from C(n,k) possible draws. Summing the numerators counts every possible D once.
This is the hypergeometric distribution, not a simulation estimate or a prize table.
Bonus and independent extra balls are excluded from this event.

Rust supplies decimal-string counts. The browser keeps these exact for the fractions
and sum using BigInt; only bars and percentages use approximate floating point.
Positive percentages below 0.0001% display “<0.0001%” instead of rounding to zero.
No added network request, external chart dependency or changed random generator.

## Verification and release boundaries

Core reference fixtures cover the six 5/36 numerators and refusal of unverified
profiles. Existing tests cover normalization across all enabled profiles.
Browser regression covers the exact rows, extra independence, model changes,
mode/count invariance and mobile page overflow, alongside prior workflow tests.
CI must pass on the exact candidate revision before merge; inspect the PR checks.
Implementation and test presence are not assertions that those checks passed.

Remaining: gated merge, existing-service deployment, live browser/visual evidence,
client acceptance and release artifacts/tag. No stable release or new official
profile is included. See [next actions](../NEXT-STEPS.md).
