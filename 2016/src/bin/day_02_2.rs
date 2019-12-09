use std::io::BufRead;
use Direction::*;

pub enum Direction {
    U,
    R,
    D,
    L,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => U,
            'R' => R,
            'D' => D,
            'L' => L,
            _ => { panic!(); }
        }
    }
}

pub struct State {
    loc: u8,
}

impl State {
    pub fn new() -> State {
        State {
            loc: 5,
        }
    }

    pub fn do_move(&mut self, dir: Direction) {
        let mut new_loc = match (self.loc, dir) {
            (1, D) => 3,
            (1, _) => 1,
            (2, R) => 3,
            (2, D) => 6,
            (2, _) => 2,
            (3, U) => 1,
            (3, R) => 4,
            (3, D) => 7,
            (3, L) => 2,
            (4, D) => 8,
            (4, L) => 3,
            (4, _) => 4,
            (5, R) => 6,
            (5, _) => 5,
            (6, U) => 2,
            (6, R) => 7,
            (6, D) => 10,
            (6, L) => 5,
            (7, U) => 3,
            (7, R) => 8,
            (7, D) => 11,
            (7, L) => 6,
            (8, U) => 4,
            (8, R) => 9,
            (8, D) => 12,
            (8, L) => 7,
            (9, L) => 8,
            (9, _) => 9,
            (10, U) => 6,
            (10, R) => 11,
            (10, _) => 10,
            (11, U) => 7,
            (11, R) => 12,
            (11, D) => 13,
            (11, L) => 10,
            (12, U) => 8,
            (12, L) => 11,
            (12, _) => 12,
            (13, U) => 11,
            (13, _) => 13,
            (_, _) => { panic!() },
        };
        ::std::mem::swap(&mut new_loc, &mut self.loc);
    }
}

fn main() {
    let mut state = State::new();
    let stdin = ::std::io::stdin();
    let mut nums = Vec::<u8>::new();

    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            state.do_move(c.into());
        }
        nums.push(state.loc);
    }

    for n in nums {
        print!("{:X}", n);
    }
    println!("");
}
