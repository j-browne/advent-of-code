use std::collections::HashMap;

#[derive(Debug)]
pub struct Polymer {
    /// pairs in polymer sequence
    pairs: HashMap<(char, char), u128>,
    first: char,
    last: char,
    /// Insertion rules
    ins_rules: HashMap<(char, char), char>,
}

impl Polymer {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.trim().split("\n\n");

        let seq = it.next().unwrap().chars().collect::<Vec<_>>();
        let first = seq[0];
        let last = seq[seq.len() - 1];

        let mut pairs = HashMap::new();
        for p in seq.windows(2) {
            *pairs.entry((p[0], p[1])).or_insert(0) += 1;
        }

        let ins_rules = it
            .next()
            .unwrap()
            .split('\n')
            .map(|s| {
                let mut it1 = s.split(" -> ");
                let mut it2 = it1.next().unwrap().chars();
                let a = it2.next().unwrap();
                let b = it2.next().unwrap();
                let mut it2 = it1.next().unwrap().chars();
                let c = it2.next().unwrap();
                ((a, b), c)
            })
            .collect();

        Self {
            pairs,
            first,
            last,
            ins_rules,
        }
    }

    pub fn insert(&mut self) {
        let mut old_pairs = HashMap::new();
        std::mem::swap(&mut old_pairs, &mut self.pairs);
        for ((a, c), n) in old_pairs {
            let b = self.ins_rules[&(a, c)];
            *self.pairs.entry((a, b)).or_insert(0) += n;
            *self.pairs.entry((b, c)).or_insert(0) += n;
        }
    }

    pub fn insert_n(&mut self, n: u32) {
        for _ in 0..n {
            self.insert();
        }
    }

    #[must_use]
    pub fn counts(&self) -> HashMap<char, u128> {
        let mut m = HashMap::new();
        for ((a, b), n) in &self.pairs {
            *m.entry(*a).or_insert(0) += n;
            *m.entry(*b).or_insert(0) += n;
        }

        // why +1 and not -1?
        *m.get_mut(&self.first).unwrap() += 1;
        *m.get_mut(&self.last).unwrap() += 1;

        for n in m.values_mut() {
            *n /= 2;
        }

        m
    }
}
