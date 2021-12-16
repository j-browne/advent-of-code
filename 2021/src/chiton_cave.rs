use crate::Vec2d;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Grid {
    risks: Vec2d<u32>,
}

impl Grid {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.split('\n').filter(|x| !x.is_empty());
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let data = it
            .flat_map(|r| {
                r.chars().map(|c| {
                    let c = c as u32;
                    assert!(c >= 48);
                    assert!(c <= 57);
                    c - 48
                })
            })
            .collect();
        let risks = Vec2d::new(dim, data);

        Self { risks }
    }

    #[must_use]
    pub fn new_expanded(s: &str) -> Self {
        let it = s.split('\n').filter(|x| !x.is_empty());
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let data = it
            .flat_map(|r| {
                r.chars().map(|c| {
                    let c = c as u32;
                    assert!(c >= 48);
                    assert!(c <= 57);
                    c - 48
                })
            })
            .collect();
        let old_risks = Vec2d::new(dim, data);

        let dim = (5 * old_risks.dim().0, 5 * old_risks.dim().1);
        let data = vec![0; dim.0 * dim.1];
        let mut risks = Vec2d::new(dim, data);

        for y in 0..5 {
            for x in 0..5 {
                let dim = old_risks.dim();
                copy(
                    &old_risks,
                    &mut risks,
                    (x * dim.1, y * dim.0),
                    (x + y).try_into().unwrap(),
                );
            }
        }

        Self { risks }
    }

    #[must_use]
    pub fn min_path(&self) -> u32 {
        let dim = self.risks.dim();
        let mut shorts = Vec2d::new(dim, vec![u32::MAX; dim.0 * dim.1]);
        let mut to_visit = HashSet::new();

        shorts[(0, 0)] = 0;
        to_visit.insert((0, 0));

        loop {
            let curr = *to_visit.iter().min_by_key(|x| shorts[**x]).unwrap();
            to_visit.remove(&curr);

            // since curr is the shortest path so far, if it at the end, it is also the shortest
            // path to the end
            if curr == (dim.0 - 1, dim.1 - 1) {
                break;
            }

            let neighbors = shorts.neighbor_indices(curr.0, curr.1).collect::<Vec<_>>();
            for n in neighbors {
                if shorts[curr] + self.risks[n] < shorts[n] {
                    shorts[n] = shorts[curr] + self.risks[n];
                    to_visit.insert(n);
                }
            }
        }

        shorts[(dim.0 - 1, dim.1 - 1)]
    }
}

fn copy(from: &Vec2d<u32>, to: &mut Vec2d<u32>, start: (usize, usize), diff: u32) {
    assert!(to.dim().0 >= from.dim().0 + start.0);
    assert!(to.dim().1 >= from.dim().1 + start.1);

    for y in 0..(from.dim().0) {
        for x in 0..(from.dim().1) {
            let old = from[(x, y)];
            let new = ((old - 1 + diff) % 9) + 1;
            to[(x + start.0, y + start.1)] = new;
        }
    }
}
