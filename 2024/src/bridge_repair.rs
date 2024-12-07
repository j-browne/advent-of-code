use std::{collections::HashSet, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calibration {
    input: Vec<u64>,
    output: u64,
    ops: Vec<fn(u64, u64) -> Option<u64>>,
}

impl Calibration {
    #[must_use]
    pub fn new(s: &str, ops: Vec<fn(u64, u64) -> Option<u64>>) -> Self {
        let mut it = s.split(": ");
        let output = it.next().unwrap().parse().unwrap();
        let input = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { input, output, ops }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        let mut it = self.input.iter();
        let mut set = HashSet::new();
        let mut old_set = HashSet::new();
        old_set.insert(*it.next().unwrap());
        for input in it {
            for val in old_set.drain() {
                for op in &self.ops {
                    if let Some(new_val) = op(val, *input) {
                        // We can do this because the result of add, mul, and cat
                        // are always larger than the inputs
                        if new_val <= self.output {
                            set.insert(new_val);
                        }
                    }
                }
            }
            mem::swap(&mut old_set, &mut set);
        }
        old_set.contains(&self.output)
    }

    #[must_use]
    pub fn output(&self) -> u64 {
        self.output
    }
}

#[must_use]
pub fn checked_cat(a: u64, b: u64) -> Option<u64> {
    let exp = b.ilog10() + 1;
    Some(a * 10_u64.pow(exp) + b)
}
