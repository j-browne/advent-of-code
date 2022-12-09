use crate::array_2d::Array2d;
use std::{
    cmp,
    iter::{repeat, zip},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trees {
    heights: Array2d<u8>,
}

impl Trees {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines();
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let data = it
            .flat_map(|r| {
                r.chars().map(|c| {
                    let c = c as u8;
                    assert!(c >= b'0');
                    assert!(c <= b'9');
                    c - b'0'
                })
            })
            .collect();
        let heights = Array2d::new(dim, data);

        Self { heights }
    }

    #[must_use]
    pub fn visible_outside(&self) -> usize {
        let dim = self.heights.dim();
        let mut visibility = {
            let data = vec![false; dim.0 * dim.1];
            Array2d::new(dim, data)
        };

        // edges are always visible
        for i in 0..dim.0 {
            visibility[(i, 0)] = true;
            visibility[(i, dim.1 - 1)] = true;
        }
        for j in 0..dim.1 {
            visibility[(0, j)] = true;
            visibility[(dim.0 - 1, j)] = true;
        }

        for i in 0..dim.0 {
            // do both left and right at the same time
            let mut max_0 = 0;
            let mut max_1 = 0;
            for j in 0..dim.1 {
                let idx_0 = (i, j);
                let idx_1 = (i, dim.1 - j - 1);
                if self.heights[idx_0] > max_0 {
                    visibility[idx_0] = true;
                    max_0 = self.heights[idx_0];
                }
                if self.heights[idx_1] > max_1 {
                    visibility[idx_1] = true;
                    max_1 = self.heights[idx_1];
                }
            }
        }

        for j in 0..dim.1 {
            // do both up and down at the same time
            let mut max_0 = 0;
            let mut max_1 = 0;
            for i in 0..dim.0 {
                let idx_0 = (i, j);
                let idx_1 = (dim.0 - i - 1, j);
                if self.heights[idx_0] > max_0 {
                    visibility[idx_0] = true;
                    max_0 = self.heights[idx_0];
                }
                if self.heights[idx_1] > max_1 {
                    visibility[idx_1] = true;
                    max_1 = self.heights[idx_1];
                }
            }
        }

        visibility.into_iter().filter(|x| *x).count()
    }

    #[must_use]
    pub fn max_scenic_score(&self) -> usize {
        let mut max_scenic_score = 0;
        let dim = self.heights.dim();

        for i in 1..(dim.0 - 1) {
            for j in 1..(dim.1 - 1) {
                let new_pos: [Box<dyn Iterator<Item = (usize, usize)>>; 4] = [
                    Box::new(zip((0..i).rev(), repeat(j))),     // left
                    Box::new(zip((i + 1)..(dim.0), repeat(j))), // right
                    Box::new(zip(repeat(i), (0..j).rev())),     // up
                    Box::new(zip(repeat(i), (j + 1)..(dim.1))), // down
                ];

                let curr_height = self.heights[(i, j)];

                let scenic_score = new_pos
                    .into_iter()
                    .map(|it| {
                        let mut max_dist = 0;
                        for pos in it {
                            max_dist += 1;
                            if self.heights[dbg!(pos)] >= curr_height {
                                break;
                            }
                        }
                        max_dist
                    })
                    .product();
                max_scenic_score = cmp::max(scenic_score, max_scenic_score);
            }
        }

        max_scenic_score
    }
}
