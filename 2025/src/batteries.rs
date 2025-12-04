use std::cmp::Reverse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batteries {
    banks: Vec<Bank>,
}

impl Batteries {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let banks = s.lines().map(Bank::new).collect();
        Self { banks }
    }

    #[must_use]
    pub fn sum_max_joltage(&self, size: usize) -> u64 {
        self.banks.iter().map(|bank| bank.max_joltage(size)).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bank {
    vals: Vec<u8>,
}

impl Bank {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let vals = s.bytes().map(|c| c - b'0').collect();
        Self { vals }
    }

    #[must_use]
    fn max_joltage(&self, size: usize) -> u64 {
        let mut sum = 0;
        let mut vals = self.vals.as_slice();

        for curr_size in (1..=size).rev() {
            // The use of Reverse and flipping the order of the index and value are so that `max`
            // prefers the highest value, but if there are multiple entries with the same maximum
            // value, it will prefer the one that comes first
            let (curr_val, Reverse(idx)) = vals[..vals.len() - (curr_size - 1)]
                .iter()
                .enumerate()
                .map(|(a, b)| (b, Reverse(a)))
                .max()
                .unwrap();

            vals = &vals[idx + 1..];
            sum *= 10;
            sum += u64::from(*curr_val);
        }

        sum
    }
}
