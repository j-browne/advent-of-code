use std::{convert::TryInto, str::FromStr};

#[derive(Debug)]
pub struct Game {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");

        let draws = it
            .next()
            .unwrap()
            .split(',')
            .map(u32::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let boards = it.map(Board::new).collect::<Vec<_>>();

        Self { draws, boards }
    }

    pub fn first_winner(&mut self) -> (Board, u32) {
        for d in &self.draws {
            for b in &mut self.boards {
                if let Some(x) = b.grid.iter_mut().find(|x| **x == Some(*d)) {
                    *x = None;
                }
                if b.won() {
                    return (b.clone(), *d);
                }
            }
        }
        panic!("no winner")
    }

    pub fn last_winner(&mut self) -> (Board, u32) {
        let mut candidates = (0..(self.boards.len())).collect::<Vec<_>>();

        for d in &self.draws {
            for b in &mut self.boards {
                if let Some(x) = b.grid.iter_mut().find(|x| **x == Some(*d)) {
                    *x = None;
                }
            }

            // This assumes that multiple boards do not tie for last
            let winner = candidates[0];
            candidates.retain(|x| !self.boards[*x].won());
            if candidates.is_empty() {
                return (self.boards[winner].clone(), *d);
            }
        }
        panic!("no last winner")
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: [Option<u32>; 25],
}

impl Board {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let grid = s
            .split_whitespace()
            .map(|x| u32::from_str(x).map(Option::Some))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .try_into()
            .unwrap();
        Self { grid }
    }

    #[must_use]
    pub fn won(&self) -> bool {
        for i in 0..5 {
            if (0..5).map(|j| self.grid[5 * i + j]).all(|x| x.is_none()) {
                return true;
            }
            if (0..5).map(|j| self.grid[5 * j + i]).all(|x| x.is_none()) {
                return true;
            }
        }
        if (0..5).map(|i| self.grid[5 * i + i]).all(|x| x.is_none()) {
            return true;
        }
        if (0..5)
            .map(|i| self.grid[5 * i + (5 - i)])
            .all(|x| x.is_none())
        {
            return true;
        }

        false
    }

    #[must_use]
    pub fn score(&self, winning_number: u32) -> u32 {
        self.grid.iter().filter_map(|x| *x).sum::<u32>() * winning_number
    }
}
