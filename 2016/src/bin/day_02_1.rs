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
            (l, U) if (l - 1) / 3 == 0 => l,
            (l, R) if (l - 1) % 3 == 2 => l,
            (l, D) if (l - 1) / 3 == 2 => l,
            (l, L) if (l - 1) % 3 == 0 => l,
            (l, U) => l - 3,
            (l, R) => l + 1,
            (l, D) => l + 3,
            (l, L) => l - 1,
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
        print!("{}", n);
    }
    println!("");
}
