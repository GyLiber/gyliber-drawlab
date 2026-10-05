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
#[cfg(test)]
mod tests {
    use super::*;
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
