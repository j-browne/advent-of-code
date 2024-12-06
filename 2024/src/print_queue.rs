use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintQueue {
    rules: HashSet<(u32, u32)>,
    updates: Vec<Update>,
}

impl PrintQueue {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");

        let rules = it
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut it = l.split('|');
                (
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        let updates = it.next().unwrap().lines().map(Update::new).collect();

        Self { rules, updates }
    }

    pub fn valid_updates(&self) -> impl Iterator<Item = &Update> {
        self.updates.iter().filter(|u| u.is_valid(&self.rules))
    }

    pub fn invalid_updates_corrected(&self) -> impl Iterator<Item = Update> + use<'_> {
        self.updates
            .iter()
            .filter(|u| !u.is_valid(&self.rules))
            .map(|u| u.correct(&self.rules))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pages: Vec<u32>,
}

impl Update {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let pages = s.split(',').map(|s| s.parse().unwrap()).collect();

        Self { pages }
    }

    #[must_use]
    pub fn correct(&self, rules: &HashSet<(u32, u32)>) -> Self {
        let mut new = self.clone();

        for i in 0..self.pages.len() {
            for j in (i + 1)..self.pages.len() {
                if rules.contains(&(new.pages[j], new.pages[i])) {
                    new.pages.swap(i, j);
                }
            }
        }

        new
    }

    #[must_use]
    pub fn is_valid(&self, rules: &HashSet<(u32, u32)>) -> bool {
        for i in 0..self.pages.len() {
            for j in (i + 1)..self.pages.len() {
                if rules.contains(&(self.pages[j], self.pages[i])) {
                    return false;
                }
            }
        }
        true
    }

    #[must_use]
    pub fn middle_page(&self) -> u32 {
        let idx = self.pages.len() / 2;
        self.pages[idx]
    }
}
