use crate::array_2d::{Array2d, Dir};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    layout: Array2d<usize, Tile>,
}

impl Platform {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let layout = Array2d::from_grid(s, Tile::new);
        Self { layout }
    }

    pub fn tilt(&mut self, dir: Dir) {
        let dir = -dir;
        for it in self.layout.iter_indices_2d(dir) {
            let mut first = it.clone().next().unwrap();
            let mut count = 0;
            for indices in it {
                match self.layout[indices] {
                    Tile::CubeRock => {
                        for i in 0..count {
                            self.layout[(first + dir * i).unwrap()] = Tile::RoundRock;
                        }
                        if let Some(new_first) = indices + dir {
                            first = new_first;
                        }
                        count = 0;
                    }
                    Tile::RoundRock => {
                        count += 1;
                        self.layout[indices] = Tile::Empty;
                    }
                    Tile::Empty => {}
                }
            }
            for i in 0..count {
                self.layout[(first + dir * i).unwrap()] = Tile::RoundRock;
            }
        }
        println!("{}", self.layout);
    }

    pub fn cycle(&mut self) {
        self.tilt(Dir::Up);
        self.tilt(Dir::Left);
        self.tilt(Dir::Down);
        self.tilt(Dir::Right);
    }

    #[must_use]
    pub fn load(&self) -> usize {
        self.layout
            .iter_indices_2d(Dir::Right)
            .enumerate()
            .map(|(i, it)| {
                (self.layout.n_rows() - i)
                    * it.filter(|indices| self.layout[*indices] == Tile::RoundRock)
                        .count()
            })
            .sum()
    }

    #[must_use]
    pub fn load_after(&self, n: usize) -> usize {
        let mut visited = HashMap::new();
        let mut loads = Vec::new();
        let mut clone = self.clone();
        let offset;
        let mut curr = 0;
        loop {
            if let Some(i) = visited.get(clone.layout.data()) {
                offset = i;
                break;
            }
            visited.insert(clone.layout.data().clone(), curr);
            loads.push(clone.load());
            curr += 1;
            clone.cycle();
        }
        let cycle = curr - offset;

        let idx = (n - offset) % cycle + offset;
        loads[idx]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn new(b: u8) -> Self {
        match b {
            b'O' => Self::RoundRock,
            b'#' => Self::CubeRock,
            b'.' => Self::Empty,
            _ => panic!("unknown tile: {b}"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::RoundRock => write!(f, "O"),
            Tile::CubeRock => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}
