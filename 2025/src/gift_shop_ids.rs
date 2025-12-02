#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let (min, max) = s.split_once('-').unwrap();
        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GiftShopIds {
    ranges: Vec<Range>,
}

impl GiftShopIds {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let ranges = s.trim().split(',').map(Range::new).collect();
        Self { ranges }
    }

    #[must_use]
    pub fn sum_invalid(self) -> u64 {
        let mut sum = 0;
        for Range { min, max } in self.ranges {
            let pow_min = min.ilog10() / 2 + 1;
            let pow_max = max.ilog10() / 2 + 1;

            let mul1_min = u64::pow(10, pow_min) + 1;
            let mul1_max = u64::pow(10, pow_max) + 1;

            for pow in pow_min..=pow_max {
                let mul1 = u64::pow(10, pow) + 1;

                let mul2_min = if pow == pow_min {
                    min.div_ceil(mul1_min).max(u64::pow(10, pow - 1))
                } else {
                    u64::pow(10, pow - 1) + 1
                };
                let mul2_max = if pow == pow_max {
                    max / mul1_max
                } else {
                    u64::pow(10, pow) - 1
                };

                for mul2 in mul2_min..=mul2_max {
                    sum += mul1 * mul2;
                }
            }
        }
        sum
    }
}
