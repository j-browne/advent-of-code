use crate::array_2d::Array2d;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Item {
    Symbol(char),
    Number(u32),
    Empty,
}

pub struct Engine {
    schematic: Array2d<Item>,
}

impl Engine {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let dim = {
            let x = s.lines().count();
            (x, x)
        };
        let mut items = Vec::with_capacity(dim.0 * dim.1);

        for line in s.lines() {
            assert_eq!(line.chars().count(), dim.0);

            let mut num_start = None;
            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    if num_start.is_none() {
                        num_start = Some(i);
                    }
                } else {
                    if let Some(start) = num_start {
                        let num = line[start..i].parse::<u32>().unwrap();
                        for _ in start..i {
                            items.push(Item::Number(num));
                        }
                        num_start = None;
                    }

                    if c == '.' {
                        items.push(Item::Empty);
                    } else {
                        items.push(Item::Symbol(c));
                    }
                }
            }

            // If the line ends in a number
            if let Some(start) = num_start {
                let num = line[start..(dim.1)].parse::<u32>().unwrap();
                for _ in start..(dim.1) {
                    items.push(Item::Number(num));
                }
            }
        }

        let schematic = Array2d::new(dim, items);
        Self { schematic }
    }

    #[must_use]
    pub fn part_numbers(&self) -> Vec<u32> {
        let dim = self.schematic.dim();
        let mut numbers = Vec::new();

        for i in 0..(dim.0) {
            for j in 0..(dim.1) {
                if let Item::Symbol(_) = self.schematic[(i, j)] {
                    let set = self
                        .schematic
                        .neighbors_diag(i, j)
                        .filter_map(|x| {
                            if let Item::Number(p) = x {
                                Some(p)
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>();
                    numbers.extend(set.into_iter());
                }
            }
        }

        numbers
    }

    #[must_use]
    pub fn gear_ratios(&self) -> Vec<u32> {
        let dim = self.schematic.dim();
        let mut ratios = Vec::new();

        for i in 0..(dim.0) {
            for j in 0..(dim.1) {
                if let Item::Symbol('*') = self.schematic[(i, j)] {
                    let nums = self
                        .schematic
                        .neighbors_diag(i, j)
                        .filter_map(|x| {
                            if let Item::Number(p) = x {
                                Some(p)
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>();
                    if nums.len() == 2 {
                        ratios.push(nums.into_iter().product());
                    }
                }
            }
        }

        ratios
    }
}
