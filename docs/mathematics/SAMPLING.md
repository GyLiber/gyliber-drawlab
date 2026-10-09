# Sampling and exact probability

## Assumptions

The platform supplies independent, computationally unpredictable 32-bit words.
Our transformation is exactly uniform conditional on ideal uniform input.
This is a conditional mathematical proof, not a claim that hardware entropy has
been measured or that the software is certified.

## Bounded integers

For positive bound b, let L = floor(2^32/b)b, calculated in u64.
Accept x only when x < L, then return x mod b. Every residue has L/b accepted
preimages. The output is uniform conditional on success. After 128 rejected
words, return an error; never substitute a value. Entropy errors propagate.

## Subsets

At step i of partial Fisher–Yates, select uniformly from the n-i unused entries.
Each ordered k-tuple has probability 1/(n(n-1)...(n-k+1)).
Each unordered subset has k! orderings, so each has probability 1/C(n,k).
Sort the main subset only after sampling. A remaining-pool bonus is the next
sampled entry, not the largest number in a sorted combined set.
An independent extra ball uses a separate bounded sample and may equal a main.

Selections have no remaining-pool bonus. Simulated draws do. Independent extras
exist in both modes. Boards are independent draws from the combination space:
duplicate boards are possible and are not filtered.

## Exact arithmetic

C(n,k) uses a multiplicative integer recurrence in u128 and checked conversion
to u64; supported n <= 64. C(n,k)=0 when k>n; C(0,0)=1.
An independent m-ball field multiplies the complete-match denominator by m.
The main-match probability is C(k,r)C(n-k,k-r)/C(n,k).
Export integer numerators/denominators as decimal strings to avoid JavaScript
floating-point truncation. Main-match fractions are exact but not reduced.

Reference counts:
- C(58,6) = 40,475,358
- C(52,6) = 20,358,520
- C(50,5) * 20 = 42,375,200
- C(50,5) * 16 = 33,900,160
- C(36,5) = 376,992

## Evidence

Core tests enumerate every 8-bit input for each bound 1..256, and enumerate the
full ordered choice space for a 2-of-4 model against an independent unordered
combination oracle. Tests also exercise rejection tails, empty entropy streams,
retry exhaustion, bonus ordering, overlapping independent pools, exact counts
and hypergeometric normalization. Real entropy smoke tests check invariants, not
statistical significance. Passing frequency tests would not prove security.
