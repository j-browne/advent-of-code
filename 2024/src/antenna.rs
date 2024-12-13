use crate::array_2d::{Dimensions, Indices};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    dims: Dimensions<isize>,
    antennae: HashMap<u8, Vec<Indices<isize>>>,
}

impl Map {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let dims = Dimensions {
            rows: 0..s.lines().clone().count().try_into().unwrap(),
            cols: 0..s.lines().clone().next().unwrap().len().try_into().unwrap(),
        };
        let mut antennae = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            let row = row.try_into().unwrap();
            for (col, b) in line.bytes().enumerate() {
                let col = col.try_into().unwrap();
                if b.is_ascii_alphanumeric() {
                    antennae
                        .entry(b)
                        .or_insert_with(Vec::new)
                        .push(Indices { row, col });
                }
            }
        }
        Self { dims, antennae }
    }

    #[must_use]
    pub fn num_antinodes_one(&self) -> usize {
        let mut antinodes = HashSet::new();
        for indices_list in self.antennae.values() {
            for i in 0..indices_list.len() {
                for j in (i + 1)..indices_list.len() {
                    let d_row = indices_list[j].row - indices_list[i].row;
                    let d_col = indices_list[j].col - indices_list[i].col;

                    let indices = Indices {
                        row: indices_list[j].row + d_row,
                        col: indices_list[j].col + d_col,
                    };
                    if self.dims.contains(indices) {
                        antinodes.insert(indices);
                    }
                    let indices = Indices {
                        row: indices_list[i].row - d_row,
                        col: indices_list[i].col - d_col,
                    };
                    if self.dims.contains(indices) {
                        antinodes.insert(indices);
                    }
                }
            }
        }
        antinodes.len()
    }

    #[must_use]
    pub fn num_antinodes_many(&self) -> usize {
        let mut antinodes = HashSet::new();
        for indices_list in self.antennae.values() {
            for i in 0..indices_list.len() {
                for j in (i + 1)..indices_list.len() {
                    let d_row = indices_list[j].row - indices_list[i].row;
                    let d_col = indices_list[j].col - indices_list[i].col;

                    let mut base = Indices {
                        row: indices_list[j].row,
                        col: indices_list[j].col,
                    };
                    loop {
                        if self.dims.contains(base) {
                            antinodes.insert(base);
                        } else {
                            break;
                        }
                        base = Indices {
                            row: base.row + d_row,
                            col: base.col + d_col,
                        };
                    }

                    base = Indices {
                        row: indices_list[i].row,
                        col: indices_list[i].col,
                    };
                    loop {
                        if self.dims.contains(base) {
                            antinodes.insert(base);
                        } else {
                            break;
                        }
                        base = Indices {
                            row: base.row - d_row,
                            col: base.col - d_col,
                        };
                    }
                }
            }
        }
        antinodes.len()
    }
}
