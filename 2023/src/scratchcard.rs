use std::collections::HashSet;

pub struct Scratchcard {
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Scratchcard {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split(": ").skip(1);
        let mut it = it.next().unwrap().split(" | ");
        let winning_numbers = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let numbers = it
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Self {
            numbers,
            winning_numbers,
        }
    }

    #[must_use]
    pub fn wins(&self) -> usize {
        HashSet::intersection(&self.numbers, &self.winning_numbers).count()
    }
}
