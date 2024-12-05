use crate::{
    array_2d::{Array2d, Indices},
    dir::Dir8,
};
use std::convert::identity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordSearch {
    letters: Array2d<usize, u8>,
}

impl WordSearch {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let letters = Array2d::from_grid(s, identity);
        Self { letters }
    }

    #[must_use]
    pub fn count_xmas(&self) -> usize {
        const GOAL: &[u8] = b"XMAS";

        (0..self.letters.n_cells())
            .flat_map(|idx| {
                [
                    (idx, Dir8::Up),
                    (idx, Dir8::UpRight),
                    (idx, Dir8::Right),
                    (idx, Dir8::RightDown),
                    (idx, Dir8::Down),
                    (idx, Dir8::DownLeft),
                    (idx, Dir8::Left),
                    (idx, Dir8::UpLeft),
                ]
                .into_iter()
            })
            .filter(|(idx, dir)| {
                let indices = self.letters.indices(*idx).unwrap();
                GOAL.iter().enumerate().all(|(i, goal_letter)| {
                    let letter = self.letters.get_opt(indices + *dir * i);
                    letter.is_some_and(|letter| letter == goal_letter)
                })
            })
            .count()
    }

    #[must_use]
    pub fn count_crossed_mas(&self) -> usize {
        (0..self.letters.n_cells())
            .filter(|idx| {
                let indices = self.letters.indices(*idx).unwrap();
                is_crossed_mas(&self.letters, indices)
            })
            .count()
    }
}

fn is_crossed_mas(letters: &Array2d<usize, u8>, indices: Indices<usize>) -> bool {
    const GOAL: &[u8; 3] = b"MAS";
    let center = letters[indices] == GOAL[1];
    let cross_1 = (letters.get_opt(indices + Dir8::UpLeft) == Some(&GOAL[0])
        && letters.get_opt(indices + Dir8::RightDown) == Some(&GOAL[2]))
        || (letters.get_opt(indices + Dir8::UpLeft) == Some(&GOAL[2])
            && letters.get_opt(indices + Dir8::RightDown) == Some(&GOAL[0]));
    let cross_2 = (letters.get_opt(indices + Dir8::DownLeft) == Some(&GOAL[0])
        && letters.get_opt(indices + Dir8::UpRight) == Some(&GOAL[2]))
        || (letters.get_opt(indices + Dir8::DownLeft) == Some(&GOAL[2])
            && letters.get_opt(indices + Dir8::UpRight) == Some(&GOAL[0]));

    center && cross_1 && cross_2
}
