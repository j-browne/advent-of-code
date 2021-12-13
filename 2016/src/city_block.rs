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

impl Facing {
    pub fn turn(&mut self, t: Turning) {
        let mut new_dir = match (*self, t) {
            (Facing::N, Turning::R) => Facing::E,
            (Facing::N, Turning::L) => Facing::W,
            (Facing::E, Turning::R) => Facing::S,
            (Facing::E, Turning::L) => Facing::N,
            (Facing::S, Turning::R) => Facing::W,
            (Facing::S, Turning::L) => Facing::E,
            (Facing::W, Turning::R) => Facing::N,
            (Facing::W, Turning::L) => Facing::S,
        };
        ::std::mem::swap(self, &mut new_dir);
    }
}

pub struct State {
    pub loc: (i32, i32),
    pub dir: Facing,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> State {
        State {
            loc: (0, 0),
            dir: Facing::N,
        }
    }

    pub fn do_command(&mut self, t: Turning, l: i32) {
        self.dir.turn(t);
        match self.dir {
            Facing::N => {
                self.loc.1 += l;
            }
            Facing::E => {
                self.loc.0 += l;
            }
            Facing::S => {
                self.loc.1 -= l;
            }
            Facing::W => {
                self.loc.0 -= l;
            }
        }
    }

    pub fn distance(&self) -> i32 {
        self.loc.0.abs() + self.loc.1.abs()
    }

    pub fn turn(&mut self, t: Turning) {
        self.dir.turn(t);
    }

    pub fn forward(&mut self, l: i32) {
        match self.dir {
            Facing::N => {
                self.loc.1 += l;
            }
            Facing::E => {
                self.loc.0 += l;
            }
            Facing::S => {
                self.loc.1 -= l;
            }
            Facing::W => {
                self.loc.0 -= l;
            }
        }
    }
}

pub fn parse_command(c: &str) -> (Turning, i32) {
    let (c1, c2) = c.split_at(1);
    let t = match c1 {
        "R" => Turning::R,
        "L" => Turning::L,
        _ => {
            panic!("");
        }
    };
    let l = c2.parse::<i32>().unwrap();
    (t, l)
}
