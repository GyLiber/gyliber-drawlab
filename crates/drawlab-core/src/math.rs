use crate::{Error, Extra, Profile};
use serde::Serialize;

/// Exact count within the supported domain, with checked intermediates.
pub fn combinations(n: u8, k: u8) -> Result<u64, Error> {
    if n > 64 {
        return Err(Error("INVALID_BOUND: n exceeds 64"));
    }
    if k > n {
        return Ok(0);
    }
    let k = k.min(n - k);
    let mut value = 1u128;
    for i in 1..=k {
        value = value
            .checked_mul(u128::from(n - k + i))
            .ok_or(Error("ARITHMETIC_OVERFLOW"))?
            / u128::from(i);
    }
    u64::try_from(value).map_err(|_| Error("ARITHMETIC_OVERFLOW"))
}
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Odds {
    pub numerator: String,
    pub denominator: String,
    pub explanation: &'static str,
}
pub fn odds(p: &Profile) -> Result<Odds, Error> {
    p.enabled()?;
    let main = u128::from(combinations(p.pool, p.pick)?);
    let multiple = match p.extra {
        Extra::Independent { pool } => u128::from(pool),
        _ => 1,
    };
    Ok(Odds {
        numerator: "1".into(),
        denominator: (main * multiple).to_string(),
        explanation: "Exact match of all main selections and any independent extra ball. A remaining-pool bonus does not change this event.",
    })
}
#[derive(Debug, Serialize)]
pub struct MatchProbability {
    pub matches: u8,
    pub numerator: String,
    pub denominator: String,
}
/// Main-match hypergeometric distribution; not an official prize table.
pub fn match_distribution(p: &Profile) -> Result<Vec<MatchProbability>, Error> {
    p.enabled()?;
    let denominator = combinations(p.pool, p.pick)?;
    (0..=p.pick)
        .map(|r| {
            let numerator = u128::from(combinations(p.pick, r)?)
                * u128::from(combinations(p.pool - p.pick, p.pick - r)?);
            Ok(MatchProbability {
                matches: r,
                numerator: numerator.to_string(),
                denominator: denominator.to_string(),
            })
        })
        .collect()
}
/// Exact main-only tail probability for one fixed selection and independent draw.
pub fn at_least_matches(p: &Profile, minimum: u8) -> Result<Odds, Error> {
    p.enabled()?;
    if minimum > p.pick {
        return Err(Error("INVALID_THRESHOLD: exceeds selection size"));
    }
    let mut numerator = 0u128;
    for r in minimum..=p.pick {
        numerator += u128::from(combinations(p.pick, r)?)
            * u128::from(combinations(p.pool - p.pick, p.pick - r)?);
    }
    Ok(Odds {
        numerator: numerator.to_string(),
        denominator: combinations(p.pool, p.pick)?.to_string(),
        explanation: "At least the requested number of main matches in one independent draw. Extra and bonus balls excluded; not a prize probability.",
    })
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct MatchMoments {
    pub expected_numerator: String,
    pub expected_denominator: String,
    pub variance_numerator: String,
    pub variance_denominator: String,
    pub explanation: &'static str,
}

/// Hypergeometric moments for X = number of main matches against a fixed pick.
/// Fractions remain exact and unreduced. No floating point enters the authority.
pub fn match_moments(p: &Profile) -> Result<MatchMoments, Error> {
    p.enabled()?;
    let n = u128::from(p.pool);
    let k = u128::from(p.pick);
    // If n = 1, the only legal pick is 1 and X is identically 1.
    let (variance_numerator, variance_denominator) = if n == 1 {
        (0, 1)
    } else {
        (k * k * (n - k) * (n - k), n * n * (n - 1))
    };
    Ok(MatchMoments {
        expected_numerator: (k * k).to_string(),
        expected_denominator: n.to_string(),
        variance_numerator: variance_numerator.to_string(),
        variance_denominator: variance_denominator.to_string(),
        explanation: "For a fixed k-number selection and independent uniform k-of-n draw, X is hypergeometric: E[X]=k²/n and Var(X)=k²(n-k)²/[n²(n-1)]. Main numbers only; no predictions, payouts, bonus or extra-ball events.",
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moments_match_independent_enumeration_and_degenerate_boundary() {
        let mut p = crate::profiles()
            .unwrap()
            .into_iter()
            .find(|p| p.id == "lab-5of36")
            .unwrap();
        let moments = match_moments(&p).unwrap();
        assert_eq!(moments.expected_numerator, "25");
        assert_eq!(moments.expected_denominator, "36");
        assert_eq!(moments.variance_numerator, "24025");
        assert_eq!(moments.variance_denominator, "45360");
        p.pool = 5;
        p.pick = 3;
        let mut sum = 0u128;
        let mut sum_squares = 0u128;
        let mut count = 0u128;
        for a in 1..=5 {
            for b in a + 1..=5 {
                for c in b + 1..=5 {
                    let x = [a, b, c].iter().filter(|&&i| i <= 3).count() as u128;
                    sum += x;
                    sum_squares += x * x;
                    count += 1;
                }
            }
        }
        let m = match_moments(&p).unwrap();
        let en: u128 = m.expected_numerator.parse().unwrap();
        let ed: u128 = m.expected_denominator.parse().unwrap();
        let vn: u128 = m.variance_numerator.parse().unwrap();
        let vd: u128 = m.variance_denominator.parse().unwrap();
        assert_eq!(sum * ed, count * en);
        assert_eq!((sum_squares * count - sum * sum) * vd, vn * count * count);
        p.pool = 1;
        p.pick = 1;
        assert_eq!(match_moments(&p).unwrap().variance_numerator, "0");
        p.status = crate::Status::Unverified;
        assert!(match_moments(&p).is_err());
    }
    #[test]
    fn tail_probabilities_match_small_exhaustive_oracle_and_boundaries() {
        let mut p = crate::profiles()
            .unwrap()
            .into_iter()
            .find(|p| p.id == "lab-5of36")
            .unwrap();
        assert_eq!(at_least_matches(&p, 3).unwrap().numerator, "4806");
        assert_eq!(at_least_matches(&p, 0).unwrap().numerator, "376992");
        assert_eq!(at_least_matches(&p, 5).unwrap().numerator, "1");
        assert!(at_least_matches(&p, 6).is_err());
        p.pool = 5;
        p.pick = 3;
        // Independently enumerate every 3-subset of 1..=5 against {1,2,3}.
        let mut counts = [0u64; 4];
        for a in 1..=5 {
            for b in a + 1..=5 {
                for c in b + 1..=5 {
                    let matched = [a, b, c].iter().filter(|&&n| n <= 3).count();
                    for (threshold, count) in counts.iter_mut().enumerate() {
                        if matched >= threshold {
                            *count += 1;
                        }
                    }
                }
            }
        }
        for (minimum, count) in counts.into_iter().enumerate() {
            let result = at_least_matches(&p, minimum as u8).unwrap();
            assert_eq!(result.numerator, count.to_string());
            assert_eq!(result.denominator, "10");
        }
        p.status = crate::Status::Unverified;
        assert!(at_least_matches(&p, 0).is_err());
    }
    #[test]
    fn five_of_36_match_reference_counts_and_disabled_profiles() {
        let profiles = crate::profiles().unwrap();
        let p = profiles.iter().find(|p| p.id == "lab-5of36").unwrap();
        let rows = match_distribution(p).unwrap();
        let expected = [169_911u64, 157_325, 44_950, 4_650, 155, 1];
        for (r, (row, count)) in rows.iter().zip(expected).enumerate() {
            assert_eq!(usize::from(row.matches), r);
            assert_eq!(row.numerator, count.to_string());
            assert_eq!(row.denominator, "376992");
        }
        for p in profiles
            .iter()
            .filter(|p| p.status == crate::Status::Unverified)
        {
            assert!(match_distribution(p).is_err());
        }
    }
    #[test]
    fn exact_reference_counts_and_distribution_normalization() {
        for (n, k, v) in [
            (58, 6, 40_475_358),
            (52, 6, 20_358_520),
            (50, 5, 2_118_760),
            (36, 5, 376_992),
        ] {
            assert_eq!(combinations(n, k).unwrap(), v);
        }
        for p in crate::profiles()
            .unwrap()
            .into_iter()
            .filter(|p| p.enabled().is_ok())
        {
            let rows = match_distribution(&p).unwrap();
            let sum: u64 = rows
                .iter()
                .map(|r| r.numerator.parse::<u64>().unwrap())
                .sum();
            assert_eq!(sum, combinations(p.pool, p.pick).unwrap());
        }
        assert_eq!(combinations(0, 0).unwrap(), 1);
        assert_eq!(combinations(5, 6).unwrap(), 0);
        assert_eq!(combinations(64, 32).unwrap(), 1_832_624_140_942_590_534);
        assert!(combinations(65, 1).is_err());
    }
}
