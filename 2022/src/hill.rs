use crate::array_2d::Array2d;
use priority_queue::PriorityQueue;
use std::{cell::Cell, cmp::Reverse, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hill {
    start: (usize, usize),
    end: (usize, usize),
    heights: Array2d<u8>,
}

impl Hill {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let start = Cell::new(None);
        let end = Cell::new(None);

        let it = s.lines();
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);

        let mut data = Vec::with_capacity(rows * cols);
        for (j, r) in it.enumerate() {
            for (i, c) in r.chars().enumerate() {
                let c = c as u8;
                let pos = Some((i, j));
                data.push(match c {
                    b'a'..=b'z' => c - b'a',
                    b'S' => {
                        start.set(pos);
                        0
                    }
                    b'E' => {
                        end.set(pos);
                        b'z' - b'a'
                    }
                    c => panic!("unknown height '{c}'"),
                });
            }
        }
        let heights = Array2d::new(dim, data);

        let start = start.into_inner().unwrap();
        let end = end.into_inner().unwrap();

        Self {
            start,
            end,
            heights,
        }
    }

    #[must_use]
    fn dijkstra<F>(&self, end_cond: F) -> usize
    where
        F: Fn((usize, usize)) -> bool,
    {
        // go backwards from the end
        let mut dist = HashMap::new();
        dist.insert(self.end, 0);
        let mut queue = PriorityQueue::new();
        queue.push(self.end, Reverse(0));

        let mut last = None;
        while !queue.is_empty() {
            let (curr_idx, curr_dist) = queue.pop().unwrap();
            if end_cond(curr_idx) {
                last = Some(curr_idx);
                break;
            }

            for next_idx in self.heights.neighbor_indices(curr_idx.0, curr_idx.1) {
                if self.heights[next_idx] + 1 >= self.heights[curr_idx] {
                    let new_dist = curr_dist.0 + 1;

                    dist.entry(next_idx).or_insert_with(|| {
                        queue.push(next_idx, Reverse(new_dist));
                        new_dist
                    });
                    let next_dist = dist.get_mut(&next_idx).unwrap();
                    if new_dist < *next_dist {
                        queue.push(next_idx, Reverse(new_dist));
                        *next_dist = new_dist;
                    }
                }
            }
        }

        dist[&last.unwrap()]
    }

    #[must_use]
    pub fn shortest_path_dist(&self) -> usize {
        self.dijkstra(|pos| pos == self.start)
    }

    #[must_use]
    pub fn best_start_dist(&self) -> usize {
        self.dijkstra(|pos| self.heights[pos] == 0)
    }
}
