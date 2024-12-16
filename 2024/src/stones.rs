use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stones {
    stones: Vec<u64>,
}

impl Stones {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let stones = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Self { stones }
    }

    #[must_use]
    pub fn num_stones_after(&self, num_blinks: usize) -> usize {
        let mut cache = HashMap::new();

        self.stones
            .iter()
            .map(|stone| num_stones_after_inner(*stone, num_blinks, &mut cache))
            .sum()
    }
}

fn num_stones_after_inner(
    stone: u64,
    num_blinks_left: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if num_blinks_left == 0 {
        1
    } else if let Some(cache_entry) = cache.get(&(stone, num_blinks_left)) {
        *cache_entry
    } else {
        let num_stones = if stone == 0 {
            num_stones_after_inner(1, num_blinks_left - 1, cache)
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let midpoint = 10u64.pow((stone.ilog10() + 1) / 2);
            let left = stone / midpoint;
            let right = stone % midpoint;

            num_stones_after_inner(left, num_blinks_left - 1, cache)
                + num_stones_after_inner(right, num_blinks_left - 1, cache)
        } else {
            num_stones_after_inner(stone * 2024, num_blinks_left - 1, cache)
        };
        cache.insert((stone, num_blinks_left), num_stones);
        num_stones
    }
}
