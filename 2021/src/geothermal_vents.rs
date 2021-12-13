use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Floor {
    /// Dimensionality of the data
    dim: (usize, usize),
    heights: Vec<u32>,
}

impl Floor {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.split('\n').filter(|x| !x.is_empty());
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let heights = it
            .flat_map(|r| {
                r.chars().map(|c| {
                    let c = c as u32;
                    assert!(c >= 48);
                    assert!(c <= 57);
                    c - 48
                })
            })
            .collect();

        Self { dim, heights }
    }

    #[inline]
    fn idx<T>(&self, x: T, y: T) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + TryFrom<usize>,
        <T as TryFrom<usize>>::Error: Debug,
    {
        T::try_from(self.dim.1).unwrap() * y + x
    }

    #[must_use]
    pub fn height_at(&self, x: usize, y: usize) -> u32 {
        assert!(x < self.dim.1);
        assert!(y < self.dim.0);
        self.heights[self.idx(x, y)]
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = u32> + '_ {
        self.neighbor_indices(x, y)
            .map(|(x, y)| self.height_at(x, y))
    }

    pub fn neighbor_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let diffs = [(-1isize, 0isize), (0, -1), (1, 0), (0, 1)];
        diffs.into_iter().filter_map(move |(dx, dy)| {
            match (
                usize::try_from(isize::try_from(x).unwrap() + dx),
                usize::try_from(isize::try_from(y).unwrap() + dy),
            ) {
                (Ok(new_x), Ok(new_y)) if new_x >= self.dim.1 || new_y >= self.dim.0 => None,
                (Ok(new_x), Ok(new_y)) => Some((new_x, new_y)),
                _ => None,
            }
        })
    }

    pub fn low_points(&self) -> impl Iterator<Item = u32> + '_ {
        self.low_point_indices().map(|(x, y)| self.height_at(x, y))
    }

    pub fn low_point_indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..(self.dim.0))
            .flat_map(move |y| (0..(self.dim.1)).map(move |x| (x, y)))
            .filter(|(x, y)| self.is_low_point((*x, *y)))
    }

    fn is_low_point(&self, (x, y): (usize, usize)) -> bool {
        let height = self.height_at(x, y);

        self.neighbors(x, y).all(|h| h > height)
    }

    #[must_use]
    pub fn basins(&self) -> Vec<Basin> {
        let mut basins = Vec::new();

        for lp in self.low_point_indices() {
            let mut points = Vec::new();
            let mut to_visit = vec![lp];

            while !to_visit.is_empty() {
                let curr = to_visit.pop().unwrap();
                for next in self.neighbor_indices(curr.0, curr.1) {
                    if !points.contains(&next)
                        && !to_visit.contains(&next)
                        && self.height_at(next.0, next.1) != 9
                    {
                        to_visit.push(next);
                    }
                }
                points.push(curr);
            }

            basins.push(Basin { points });
        }

        basins
    }
}

pub struct Basin {
    points: Vec<(usize, usize)>,
}

impl Basin {
    #[must_use]
    pub fn size(&self) -> usize {
        self.points.len()
    }
}
