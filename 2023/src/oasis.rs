use std::iter::zip;

pub struct Oasis {
    numbers: Vec<i32>,
}

impl Oasis {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let numbers = s
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        Self { numbers }
    }

    #[must_use]
    pub fn next(&self) -> i32 {
        calculate_next(&self.numbers)
    }

    #[must_use]
    pub fn prev(&self) -> i32 {
        let mut rev = self.numbers.clone();
        rev.reverse();
        calculate_next(&rev)
    }
}

fn calculate_next(nums: &[i32]) -> i32 {
    if nums.iter().all(|x| *x == 0) {
        0
    } else {
        let v = zip(nums.iter(), nums.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        nums.iter().last().unwrap() + calculate_next(&v)
    }
}
