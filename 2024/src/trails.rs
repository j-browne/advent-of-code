use crate::array_2d::{Array2d, Indices};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    map: Array2d<usize, u32>,
}

impl Map {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let map = Array2d::from_grid(s, |c| u32::from(c - b'0'));
        Self { map }
    }

    #[must_use]
    pub fn trailhead_scores(&self) -> Vec<usize> {
        self.map
            .data()
            .iter()
            .enumerate()
            .filter(|(_idx, height)| **height == 0)
            .map(|(idx, _height)| {
                let mut peaks = HashSet::new();
                self.calculate_score_from(self.map.indices(idx).unwrap(), &mut peaks);
                peaks.len()
            })
            .collect()
    }

    fn calculate_score_from(&self, indices: Indices<usize>, peaks: &mut HashSet<Indices<usize>>) {
        let curr_height = self.map[indices];
        if curr_height == 9 {
            peaks.insert(indices);
        } else {
            for (neigh_indices, neigh_height) in self.map.neighbors_enumerated_4(indices) {
                if *neigh_height == curr_height + 1 {
                    self.calculate_score_from(neigh_indices, peaks);
                }
            }
        }
    }

    #[must_use]
    pub fn trailhead_ratings(&self) -> Vec<usize> {
        self.map
            .data()
            .iter()
            .enumerate()
            .filter(|(_idx, height)| **height == 0)
            .map(|(idx, _height)| self.calculate_ratings_from(self.map.indices(idx).unwrap()))
            .collect()
    }

    fn calculate_ratings_from(&self, indices: Indices<usize>) -> usize {
        let curr_height = self.map[indices];
        if curr_height == 9 {
            return 1;
        }

        self.map
            .neighbors_enumerated_4(indices)
            .filter(|(_neigh_indices, neigh_height)| **neigh_height == curr_height + 1)
            .map(|(neigh_indices, _neigh_height)| self.calculate_ratings_from(neigh_indices))
            .sum()
    }
}
