use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Turning {
    R,
    L,
}

#[derive(Clone, Copy)]
pub enum Facing {
    N,
    E,
    S,
    W,
}

use Facing::*;
use Turning::*;

impl Facing {
    fn turn(&mut self, t: Turning) {
        let mut new_dir = match (*self, t) {
            (N, R) => E,
            (N, L) => W,
            (E, R) => S,
            (E, L) => N,
            (S, R) => W,
            (S, L) => E,
            (W, R) => N,
            (W, L) => S,
        };
        ::std::mem::swap(self, &mut new_dir);
    }
}

pub struct State {
    loc: (i32, i32),
    dir: Facing,
}

impl State {
    pub fn new() -> State {
        State{
            loc: (0, 0),
            dir: Facing::N,
        }
    }

    pub fn turn(&mut self, t: Turning) {
        self.dir.turn(t);
    }

    pub fn forward(&mut self, l: i32) {
        match self.dir {
            N => { self.loc.1 += l; }
            E => { self.loc.0 += l; }
            S => { self.loc.1 -= l; }
            W => { self.loc.0 -= l; }
        }
    }

    pub fn do_command(&mut self, t: Turning, l: i32) {
        self.turn(t);
        self.forward(l);
    }

    pub fn distance(&self) -> i32 {
        self.loc.0.abs() + self.loc.1.abs()
    }
}

fn parse_command(c: &str) -> (Turning, i32) {
    let (c1, c2) = c.split_at(1);
    let t = match c1 {
        "R" => Turning::R,
        "L" => Turning::L,
        _ =>{ panic!(""); }
    };
    let l = c2.parse::<i32>().unwrap();
    (t, l)
}

fn main() {
    let mut state = State::new();
    let mut visited = HashSet::<(i32, i32)>::new();

    let stdin = ::std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let line = line.trim();

    for c in line.split(", ").collect::<Vec<&str>>() {
        let (t, l) = parse_command(c);
        state.turn(t);
        for _ in 0..l {
            if !visited.insert(state.loc) {
                break;
            }
            state.forward(1);
        }
    }
    println!("{}", state.distance());
}
