use crate::{Board, Error, Extra, Mode, Profile};

pub(crate) trait Words {
    fn next(&mut self) -> Result<u32, Error>;
}
pub(crate) struct Platform;
impl Words for Platform {
    fn next(&mut self) -> Result<u32, Error> {
        let mut bytes = [0; 4];
        getrandom::fill(&mut bytes).map_err(|_| Error("ENTROPY_UNAVAILABLE"))?;
        Ok(u32::from_le_bytes(bytes))
    }
}

fn accept(x: u64, bound: u64, space: u64) -> Option<u64> {
    let limit = space / bound * bound;
    (x < limit).then_some(x % bound)
}
fn below(rng: &mut impl Words, bound: u32) -> Result<u32, Error> {
    if bound == 0 {
        return Err(Error("INVALID_BOUND"));
    }
    for _ in 0..128 {
        if let Some(v) = accept(u64::from(rng.next()?), u64::from(bound), 1u64 << 32) {
            return Ok(v as u32);
        }
    }
    Err(Error("ENTROPY_RETRY_LIMIT"))
}

pub(crate) fn sample(p: &Profile, mode: Mode, rng: &mut impl Words) -> Result<Board, Error> {
    p.enabled()?;
    let bonus = p.extra == Extra::Remaining && mode == Mode::Draw;
    let mut pool: Vec<u8> = (1..=p.pool).collect();
    for i in 0..usize::from(p.pick) + usize::from(bonus) {
        let j = i + below(rng, (pool.len() - i) as u32)? as usize;
        pool.swap(i, j);
    }
    let extra = match p.extra {
        Extra::Independent { pool } => Some((below(rng, u32::from(pool))? + 1) as u8),
        Extra::Remaining if bonus => Some(pool[usize::from(p.pick)]),
        _ => None,
    };
    let mut main = pool[..usize::from(p.pick)].to_vec();
    main.sort_unstable();
    let board = Board { main, extra };
    board.validate(p, mode)?;
    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Status;
    struct Sequence(std::collections::VecDeque<u32>);
    impl Words for Sequence {
        fn next(&mut self) -> Result<u32, Error> {
            self.0.pop_front().ok_or(Error("TEST_STREAM_EMPTY"))
        }
    }
    fn sequence(xs: &[u32]) -> Sequence {
        Sequence(xs.iter().copied().collect())
    }
    fn toy(extra: Extra) -> Profile {
        Profile {
            id: "toy".into(),
            version: "1".into(),
            name: "toy".into(),
            pool: 4,
            pick: 2,
            extra,
            status: Status::Laboratory,
            source: "test".into(),
        }
    }
    #[test]
    fn exhaustive_reduced_word_space_is_balanced() {
        for bound in 1..=256 {
            let mut counts = vec![0; bound as usize];
            for x in 0..256 {
                if let Some(v) = accept(x, bound, 256) {
                    counts[v as usize] += 1;
                }
            }
            assert!(counts.iter().all(|&c| c == 256 / bound));
        }
    }
    #[test]
    fn rejection_tail_and_failure_are_not_replaced() {
        assert_eq!(below(&mut sequence(&[u32::MAX, 7]), 3), Ok(1));
        assert!(below(&mut sequence(&[]), 3).is_err());
        assert_eq!(
            below(&mut sequence(&[u32::MAX; 128]), 3),
            Err(Error("ENTROPY_RETRY_LIMIT"))
        );
        assert!(below(&mut sequence(&[0]), 0).is_err());
        assert_eq!(below(&mut sequence(&[u32::MAX]), 1), Ok(0));
    }
    #[test]
    fn exhaustive_small_sample_matches_independent_combination_oracle() {
        let p = toy(Extra::None);
        let mut counts = std::collections::BTreeMap::new();
        for a in 0..4 {
            for b in 0..3 {
                let board = sample(&p, Mode::Selection, &mut sequence(&[a, b])).unwrap();
                *counts.entry(board.main).or_insert(0) += 1;
            }
        }
        for a in 1..=4 {
            for b in a + 1..=4 {
                assert_eq!(counts.get(&vec![a, b]), Some(&2));
            }
        }
        assert_eq!(counts.len(), 6);
    }
    #[test]
    fn remaining_bonus_is_selected_before_display_sort() {
        let b = sample(
            &toy(Extra::Remaining),
            Mode::Draw,
            &mut sequence(&[3, 1, 0]),
        )
        .unwrap();
        assert_eq!(b.main, vec![3, 4]);
        assert_eq!(b.extra, Some(2));
        assert_eq!(
            sample(
                &toy(Extra::Remaining),
                Mode::Selection,
                &mut sequence(&[0, 0])
            )
            .unwrap()
            .extra,
            None
        );
    }
    #[test]
    fn separate_pool_can_overlap_main() {
        let b = sample(
            &toy(Extra::Independent { pool: 4 }),
            Mode::Selection,
            &mut sequence(&[0, 0, 0]),
        )
        .unwrap();
        assert_eq!(b.extra, Some(1));
        assert!(b.main.contains(&1));
    }
}
