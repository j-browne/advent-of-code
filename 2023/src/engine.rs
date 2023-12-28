use crate::array_2d::{Array2d, Dimensions, Indices};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Item {
    Symbol(char),
    Number(u32),
    Empty,
}

pub struct Engine {
    schematic: Array2d<usize, Item>,
}

impl Engine {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines().enumerate();
        let dims = Dimensions {
            cols: 0..it.clone().count(),
            rows: 0..it.clone().next().unwrap().1.len(),
        };
        let mut items = Vec::with_capacity(dims.col_diff() * dims.row_diff());

        for line in s.lines() {
            assert_eq!(line.chars().count(), dims.col_diff());

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
                let num = line[start..(dims.cols.end)].parse::<u32>().unwrap();
                for _ in start..(dims.cols.end) {
                    items.push(Item::Number(num));
                }
            }
        }

        let schematic = Array2d::new(dims, items);
        Self { schematic }
    }

    #[must_use]
    pub fn part_numbers(&self) -> Vec<u32> {
        let dims = self.schematic.dims();
        let mut numbers = Vec::new();

        for row in dims.rows.clone() {
            for col in dims.cols.clone() {
                let indices = Indices { row, col };
                if let Item::Symbol(_) = self.schematic[indices] {
                    let set = self
                        .schematic
                        .neighbors_diag(indices)
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
        let dims = self.schematic.dims();
        let mut ratios = Vec::new();

        for row in dims.rows.clone() {
            for col in dims.cols.clone() {
                let indices = Indices { row, col };
                if let Item::Symbol('*') = self.schematic[indices] {
                    let nums = self
                        .schematic
                        .neighbors_diag(indices)
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
