use crate::{
    array_2d::{Array2d, Indices},
    dir::Dir4,
};
use std::{
    collections::BTreeSet,
    convert::identity,
    iter::{repeat, zip},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Garden {
    map: Array2d<usize, u8>,
}

impl Garden {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let map = Array2d::from_grid(s, identity);
        Self { map }
    }

    #[must_use]
    pub fn fencing_cost(&self) -> usize {
        self.regions().iter().map(Region::cost).sum()
    }

    #[must_use]
    pub fn discounted_fencing_cost(&self) -> usize {
        self.regions().iter().map(Region::discounted_cost).sum()
    }

    #[must_use]
    fn regions(&self) -> Vec<Region> {
        let mut visited = {
            let dims = self.map.dims().clone();
            let size = dims.n_cells();
            Array2d::new(dims, vec![false; size])
        };
        let mut regions = Vec::new();

        while let Some(start_idx) = visited.iter().position(|x| !*x) {
            let mut to_visit = BTreeSet::new();
            to_visit.insert(self.map.indices(start_idx).unwrap());

            let mut plots = BTreeSet::new();
            while let Some(curr_indices) = to_visit.pop_last() {
                visited[curr_indices] = true;

                let curr = self.map[curr_indices];
                to_visit.extend(
                    self.map
                        .neighbors_enumerated_4(curr_indices)
                        .filter(|(next_indices, next)| curr == **next && !visited[*next_indices])
                        .map(|(next_indices, _)| next_indices),
                );
                plots.insert(curr_indices);
            }
            regions.push(Region {
                garden: self,
                plots,
            });
        }

        regions
    }
}

struct Region<'a> {
    garden: &'a Garden,
    plots: BTreeSet<Indices<usize>>,
}

impl Region<'_> {
    #[must_use]
    fn cost(&self) -> usize {
        let area = self.plots.len();
        let perimeter = self
            .plots
            .iter()
            .map(|p| {
                self.garden
                    .map
                    .neighbors_4_opt(*p)
                    .filter(|n| n.is_none_or(|n| self.garden.map[*p] != *n))
                    .count()
            })
            .sum::<usize>();
        area * perimeter
    }

    #[must_use]
    fn discounted_cost(&self) -> usize {
        let area = self.plots.len();
        let mut sides = 0;

        let minors = zip(
            self.garden.map.iter_indices_2d(Dir4::Up),
            repeat(Dir4::Right),
        )
        .chain(zip(
            self.garden.map.iter_indices_2d(Dir4::Right),
            repeat(Dir4::Down),
        ))
        .chain(zip(
            self.garden.map.iter_indices_2d(Dir4::Down),
            repeat(Dir4::Left),
        ))
        .chain(zip(
            self.garden.map.iter_indices_2d(Dir4::Left),
            repeat(Dir4::Up),
        ));
        for (minor, side_dir) in minors {
            let mut is_fence = false;
            for indices in minor {
                if is_fence {
                    if self.plots.contains(&indices) {
                        if (indices + side_dir)
                            .is_some_and(|side_indices| self.plots.contains(&side_indices))
                        {
                            sides += 1;
                            is_fence = false;
                        }
                    } else {
                        sides += 1;
                        is_fence = false;
                    }
                } else if self.plots.contains(&indices)
                    && (indices + side_dir)
                        .is_none_or(|side_indices| !self.plots.contains(&side_indices))
                {
                    is_fence = true;
                }
            }

            if is_fence {
                sides += 1;
            }
        }

        area * sides
    }
}
