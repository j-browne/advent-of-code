use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignments {
    assignments: Vec<Pair>,
}

impl Assignments {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let assignments = s.lines().map(Pair::new).collect();
        Self { assignments }
    }

    #[must_use]
    pub fn subset_count(&self) -> usize {
        self.assignments.iter().filter(|p| p.has_subset()).count()
    }

    #[must_use]
    pub fn overlap_count(&self) -> usize {
        self.assignments.iter().filter(|p| p.has_overlap()).count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl Pair {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut it_1 = s.split(',');
        let mut it_2 = it_1.next().unwrap().split('-');
        let left = RangeInclusive::new(
            it_2.next().unwrap().parse().unwrap(),
            it_2.next().unwrap().parse().unwrap(),
        );
        let mut it_2 = it_1.next().unwrap().split('-');
        let right = RangeInclusive::new(
            it_2.next().unwrap().parse().unwrap(),
            it_2.next().unwrap().parse().unwrap(),
        );

        Self { left, right }
    }

    #[must_use]
    fn has_subset(&self) -> bool {
        let set_1 = self.left.clone().collect::<HashSet<_>>();
        let set_2 = self.right.clone().collect::<HashSet<_>>();

        set_1.is_subset(&set_2) || set_2.is_subset(&set_1)
    }

    #[must_use]
    fn has_overlap(&self) -> bool {
        let set_1 = self.left.clone().collect::<HashSet<_>>();
        let set_2 = self.right.clone().collect::<HashSet<_>>();

        !set_1.is_disjoint(&set_2)
    }
}
