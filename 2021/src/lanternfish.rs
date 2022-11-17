use std::{collections::HashMap, str::FromStr};

pub struct Population {
    ages: HashMap<u32, u128>,
}

impl Population {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut ages = HashMap::new();
        for age in s.trim().split(',').map(u32::from_str) {
            let age = age.unwrap();
            *ages.entry(age).or_insert(0) += 1;
        }

        Self { ages }
    }

    pub fn step(&mut self) {
        let mut old_ages = HashMap::with_capacity(self.ages.len());
        std::mem::swap(&mut old_ages, &mut self.ages);
        for (old_age, n) in old_ages {
            let new_age = if old_age == 0 {
                *self.ages.entry(8).or_insert(0) += n;
                6
            } else {
                old_age - 1
            };
            *self.ages.entry(new_age).or_insert(0) += n;
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    #[must_use]
    pub fn total(&self) -> u128 {
        self.ages.values().sum()
    }
}
