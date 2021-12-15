use std::collections::HashMap;

#[derive(Debug)]
pub struct Polymer {
    /// Element sequence
    seq: Vec<char>,
    /// Insertion rules
    ins_rules: HashMap<(char, char), char>,
}

impl Polymer {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.trim().split("\n\n");

        let seq = it.next().unwrap().chars().collect();

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

        Self { seq, ins_rules }
    }

    pub fn insert(&mut self) {
        let mut old_seq = Vec::new();
        std::mem::swap(&mut old_seq, &mut self.seq);
        for x in old_seq.windows(2) {
            self.seq.push(x[0]);
            self.seq.push(self.ins_rules[&(x[0], x[1])]);
        }
        self.seq.push(*old_seq.last().unwrap());
    }

    pub fn insert_n(&mut self, n: u32) {
        for _ in 0..n {
            self.insert();
        }
    }

    pub fn counts(&self) -> HashMap<char, u128> {
        let mut m = HashMap::new();
        for c in &self.seq {
            *m.entry(*c).or_insert(0) += 1;
        }
        m
    }
}
