use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

#[derive(Debug)]
pub struct Grid {
    energies: [u8; 100],
    flashes: u32,
}

impl Grid {
    const DIM: (usize, usize) = (10, 10);

    #[must_use]
    pub fn new(s: &str) -> Self {
        let energies = s
            .trim()
            .split_whitespace()
            .flat_map(|l| {
                l.bytes().map(|c| {
                    assert!(c >= b'0');
                    assert!(c <= b'9');
                    c - b'0'
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            energies,
            flashes: 0,
        }
    }

    #[must_use]
    pub fn energy_at(&self, x: usize, y: usize) -> u8 {
        assert!(x < Self::DIM.1);
        assert!(y < Self::DIM.0);
        self.energies[idx(x, y)]
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        neighbor_indices(x, y).map(|(x, y)| self.energy_at(x, y))
    }

    pub fn step(&mut self) {
        let mut to_visit = (0..(Self::DIM.0))
            .flat_map(move |y| (0..(Self::DIM.1)).map(move |x| (x, y)))
            .collect::<Vec<_>>();
        while !to_visit.is_empty() {
            let (x, y) = to_visit.pop().unwrap();
            self.energies[idx(x, y)] += 1;
            if self.energies[idx(x, y)] == 10 {
                self.flashes += 1;
                for (new_x, new_y) in neighbor_indices(x, y) {
                    to_visit.push((new_x, new_y));
                }
            }
        }
        for e in &mut self.energies {
            if *e > 9 {
                *e = 0;
            }
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    #[must_use]
    pub fn flashes(&self) -> u32 {
        self.flashes
    }
}

#[inline]
fn idx<T>(x: T, y: T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    T::try_from(Grid::DIM.1).unwrap() * y + x
}

fn neighbor_indices(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let diffs = [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    diffs.into_iter().filter_map(move |(dx, dy)| {
        match (
            usize::try_from(isize::try_from(x).unwrap() + dx),
            usize::try_from(isize::try_from(y).unwrap() + dy),
        ) {
            (Ok(new_x), Ok(new_y)) if new_x >= Grid::DIM.1 || new_y >= Grid::DIM.0 => None,
            (Ok(new_x), Ok(new_y)) => Some((new_x, new_y)),
            _ => None,
        }
    })
}
