use std::collections::HashSet;

pub struct Rucksacks<'a> {
    rucksacks: Vec<Pair<'a>>,
}

impl<'a> Rucksacks<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let rucksacks = s.lines().map(Pair::new).collect();
        Rucksacks { rucksacks }
    }

    #[must_use]
    pub fn mixed_priority_sum(&self) -> u32 {
        self.rucksacks.iter().map(Pair::mixed_priority).sum()
    }

    #[must_use]
    pub fn badge_priority_sum(&self) -> u32 {
        self.rucksacks
            .chunks(3)
            .map(|x| {
                let a = x[0].inner.iter().collect::<HashSet<_>>();
                let b = x[1].inner.iter().collect::<HashSet<_>>();
                let c = x[2].inner.iter().collect::<HashSet<_>>();
                let ab = a.intersection(&b).copied().collect::<HashSet<_>>();
                item_priority(**ab.intersection(&c).next().unwrap())
            })
            .sum()
    }
}

struct Pair<'a> {
    inner: &'a [u8],
}

impl<'a> Pair<'a> {
    #[must_use]
    const fn new(s: &'a str) -> Self {
        Self {
            inner: s.as_bytes(),
        }
    }

    #[must_use]
    fn mixed_priority(&self) -> u32 {
        let (left, right) = self.inner.split_at(self.inner.len() / 2);
        assert_eq!(left.len(), right.len());

        let left = left.iter().collect::<HashSet<_>>();
        let right = right.iter().collect::<HashSet<_>>();

        item_priority(**left.intersection(&right).next().unwrap())
    }
}

#[must_use]
fn item_priority(item: u8) -> u32 {
    match item {
        x @ b'a'..=b'z' => x - b'a' + 1,
        x @ b'A'..=b'Z' => x - b'A' + 27,
        x => panic!("invalid rucksack item '{}'", x as char),
    }
    .into()
}
