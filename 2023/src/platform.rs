use crate::array_2d::Array2d;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    layout: Array2d<Tile>,
}

impl Platform {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines();
        let dim = (
            it.clone().count(),
            it.clone().peekable().peek().unwrap().len(),
        );

        let data = it.flat_map(|l| l.chars().map(Tile::new)).collect();
        let layout = Array2d::new(dim, data);
        Self { layout }
    }

    pub fn tilt_up(&mut self) {
        let dim = self.layout.dim();
        for col in 0..dim.1 {
            let mut first = 0;
            let mut count = 0;
            for row in 0..dim.0 {
                match self.layout[(col, row)] {
                    Tile::CubeRock => {
                        for i in 0..count {
                            self.layout[(col, first + i)] = Tile::RoundRock;
                        }
                        first = row + 1;
                        count = 0;
                    }
                    Tile::RoundRock => {
                        count += 1;
                        self.layout[(col, row)] = Tile::Empty;
                    }
                    Tile::Empty => {}
                }
            }
            for i in 0..count {
                self.layout[(col, first + i)] = Tile::RoundRock;
            }
        }
    }

    pub fn tilt_left(&mut self) {
        let dim = self.layout.dim();
        for row in 0..dim.0 {
            let mut first = 0;
            let mut count = 0;
            for col in 0..dim.1 {
                match self.layout[(col, row)] {
                    Tile::CubeRock => {
                        for i in 0..count {
                            self.layout[(first + i, row)] = Tile::RoundRock;
                        }
                        first = col + 1;
                        count = 0;
                    }
                    Tile::RoundRock => {
                        count += 1;
                        self.layout[(col, row)] = Tile::Empty;
                    }
                    Tile::Empty => {}
                }
            }
            for i in 0..count {
                self.layout[(first + i, row)] = Tile::RoundRock;
            }
        }
    }

    pub fn tilt_down(&mut self) {
        let dim = self.layout.dim();
        for col in 0..dim.1 {
            let mut first = dim.0 - 1;
            let mut count = 0;
            for row in (0..dim.0).rev() {
                match self.layout[(col, row)] {
                    Tile::CubeRock => {
                        for i in 0..count {
                            self.layout[(col, first - i)] = Tile::RoundRock;
                        }
                        first = usize::saturating_sub(row, 1);
                        count = 0;
                    }
                    Tile::RoundRock => {
                        count += 1;
                        self.layout[(col, row)] = Tile::Empty;
                    }
                    Tile::Empty => {}
                }
            }
            for i in 0..count {
                self.layout[(col, first - i)] = Tile::RoundRock;
            }
        }
    }

    pub fn tilt_right(&mut self) {
        let dim = self.layout.dim();
        for row in 0..dim.0 {
            let mut first = dim.1 - 1;
            let mut count = 0;
            for col in (0..dim.1).rev() {
                match self.layout[(col, row)] {
                    Tile::CubeRock => {
                        for i in 0..count {
                            self.layout[(first - i, row)] = Tile::RoundRock;
                        }
                        first = usize::saturating_sub(col, 1);
                        count = 0;
                    }
                    Tile::RoundRock => {
                        count += 1;
                        self.layout[(col, row)] = Tile::Empty;
                    }
                    Tile::Empty => {}
                }
            }
            for i in 0..count {
                self.layout[(first - i, row)] = Tile::RoundRock;
            }
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }

    #[must_use]
    pub fn load(&self) -> u32 {
        let dim = self.layout.dim();
        u32::try_from(
            (0..dim.0)
                .map(|i| {
                    (dim.0 - i)
                        * (0..dim.1)
                            .filter(|j| self.layout[(*j, i)] == Tile::RoundRock)
                            .count()
                })
                .sum::<usize>(),
        )
        .unwrap()
    }

    #[must_use]
    pub fn load_after(&self, n: usize) -> u32 {
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

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim = self.layout.dim();
        for row in 0..dim.0 {
            for col in 0..dim.1 {
                match self.layout[(col, row)] {
                    Tile::RoundRock => write!(f, "O")?,
                    Tile::CubeRock => write!(f, "#")?,
                    Tile::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            'O' => Self::RoundRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => panic!("unknown tile: {c}"),
        }
    }
}
