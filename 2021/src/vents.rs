use std::{collections::HashMap, ops::Not, str::FromStr};

#[derive(Debug)]
pub struct Floor {
    lines: Vec<Line>,
}

impl Floor {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let lines = s
            .split('\n')
            .filter_map(|s| s.is_empty().not().then(|| Line::new(s)))
            .collect();

        Self { lines }
    }

    #[must_use]
    pub fn overlap(&self) -> usize {
        overlap_inner(self.lines.iter())
    }

    #[must_use]
    pub fn overlap_hv(&self) -> usize {
        overlap_inner(
            self.lines
                .iter()
                .filter(|l| (l.0).0 == (l.1).0 || (l.0).1 == (l.1).1),
        )
    }
}

fn overlap_inner<'a>(it: impl Iterator<Item = &'a Line>) -> usize {
    let mut vent_pos = HashMap::new();

    for p in it.flat_map(|l| l.iter()) {
        *vent_pos.entry(p).or_insert(0) += 1;
    }

    vent_pos.into_iter().filter(|(_, c)| c > &1).count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Line(Pos, Pos);

impl Line {
    fn new(s: &str) -> Self {
        let mut it = s.split(" -> ").map(Pos::new);
        Self(it.next().unwrap(), it.next().unwrap())
    }

    fn iter(self) -> impl Iterator<Item = Pos> {
        let mut dx = (self.1).0 as i32 - (self.0).0 as i32;
        let mut dy = (self.1).1 as i32 - (self.0).1 as i32;
        let steps = i32::max(dx.abs(), dy.abs());
        if steps != 0 {
            dx /= steps;
            dy /= steps;
        }

        (0..=steps).map(move |x| Pos((self.0).0 + x * dx, (self.0).1 + x * dy))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

impl Pos {
    fn new(s: &str) -> Self {
        let mut it = s.split(',').map(i32::from_str);
        Self(it.next().unwrap().unwrap(), it.next().unwrap().unwrap())
    }
}
