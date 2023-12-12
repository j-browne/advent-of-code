use std::cmp::{max, min};

pub struct Galaxy {
    expansion: usize,
    dim: (usize, usize),
    locations: Vec<(usize, usize)>,
}

impl Galaxy {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines();
        let dim = (
            it.clone().count(),
            it.clone().peekable().peek().unwrap().len(),
        );

        let locations = it
            .enumerate()
            .flat_map(|(j, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(i, c)| (c == '#').then_some((i, j)))
            })
            .collect();

        Self {
            dim,
            locations,
            expansion: 0,
        }
    }

    pub fn set_expansion(&mut self, expansion: usize) {
        self.expansion = expansion;
    }

    #[must_use]
    pub fn sum_of_distances(&self) -> usize {
        let mut sum = 0;

        let empty_cols = (0..self.dim.0)
            .filter(|i| !self.locations.iter().any(|(x, _y)| x == i))
            .collect::<Vec<_>>();
        let empty_rows = (0..self.dim.1)
            .filter(|i| !self.locations.iter().any(|(_x, y)| y == i))
            .collect::<Vec<_>>();

        for i1 in 0..self.locations.len() {
            for i2 in (i1 + 1)..self.locations.len() {
                let (x1, y1) = self.locations[i1];
                let (x2, y2) = self.locations[i2];
                let (x1, x2) = (min(x1, x2), max(x1, x2));
                let (y1, y2) = (min(y1, y2), max(y1, y2));

                let diff = (x2 - x1)
                    + (y2 - y1)
                    + empty_cols.iter().filter(|i| **i > x1 && **i < x2).count()
                        * (self.expansion - 1)
                    + empty_rows.iter().filter(|i| **i > y1 && **i < y2).count()
                        * (self.expansion - 1);

                sum += diff;
            }
        }

        sum
    }
}
