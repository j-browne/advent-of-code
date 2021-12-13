pub enum Direction {
    U,
    R,
    D,
    L,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::U,
            'R' => Direction::R,
            'D' => Direction::D,
            'L' => Direction::L,
            _ => {
                panic!();
            }
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub loc: u8,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> State {
        State { loc: 5 }
    }

    pub fn do_move_std(&mut self, dir: Direction) {
        let mut new_loc = match (self.loc, dir) {
            (l, Direction::U) if (l - 1) / 3 == 0 => l,
            (l, Direction::R) if (l - 1) % 3 == 2 => l,
            (l, Direction::D) if (l - 1) / 3 == 2 => l,
            (l, Direction::L) if (l - 1) % 3 == 0 => l,
            (l, Direction::U) => l - 3,
            (l, Direction::R) => l + 1,
            (l, Direction::D) => l + 3,
            (l, Direction::L) => l - 1,
        };
        ::std::mem::swap(&mut new_loc, &mut self.loc);
    }

    pub fn do_move(&mut self, dir: Direction) {
        let mut new_loc = match (self.loc, dir) {
            (1, Direction::D) => 3,
            (1, _) => 1,
            (2, Direction::R) => 3,
            (2, Direction::D) => 6,
            (2, _) => 2,
            (3, Direction::U) => 1,
            (3, Direction::R) => 4,
            (3, Direction::D) => 7,
            (3, Direction::L) => 2,
            (4, Direction::D) => 8,
            (4, Direction::L) => 3,
            (4, _) => 4,
            (5, Direction::R) => 6,
            (5, _) => 5,
            (6, Direction::U) => 2,
            (6, Direction::R) => 7,
            (6, Direction::D) => 10,
            (6, Direction::L) => 5,
            (7, Direction::U) => 3,
            (7, Direction::R) => 8,
            (7, Direction::D) => 11,
            (7, Direction::L) => 6,
            (8, Direction::U) => 4,
            (8, Direction::R) => 9,
            (8, Direction::D) => 12,
            (8, Direction::L) => 7,
            (9, Direction::L) => 8,
            (9, _) => 9,
            (10, Direction::U) => 6,
            (10, Direction::R) => 11,
            (10, _) => 10,
            (11, Direction::U) => 7,
            (11, Direction::R) => 12,
            (11, Direction::D) => 13,
            (11, Direction::L) => 10,
            (12, Direction::U) => 8,
            (12, Direction::L) => 11,
            (12, _) => 12,
            (13, Direction::U) => 11,
            (13, _) => 13,
            (_, _) => panic!(),
        };
        ::std::mem::swap(&mut new_loc, &mut self.loc);
    }
}
