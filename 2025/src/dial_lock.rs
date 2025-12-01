const START: i16 = 50;
const NUM_SPOTS: i16 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Left(i16),
    Right(i16),
}

impl Turn {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s.split_at(1) {
            ("L", num) => Self::Left(num.parse().unwrap()),
            ("R", num) => Self::Right(num.parse().unwrap()),
            _ => panic!("unknown Turn pattern"),
        }
    }
}

#[must_use]
fn do_move(start: i16, turn: Turn) -> (i16, i16) {
    let end = match turn {
        Turn::Left(num) => start - num,
        Turn::Right(num) => start + num,
    };

    (end.div_euclid(NUM_SPOTS), end.rem_euclid(NUM_SPOTS))
}

#[must_use]
fn zero_correction(last_turn: Option<Turn>, this_turn: Turn) -> i16 {
    match (last_turn, this_turn) {
        (Some(Turn::Left(_)), Turn::Right(_)) => 1,
        (Some(Turn::Right(_)), Turn::Left(_)) => -1,
        _ => 0,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialLock {
    turns: Vec<Turn>,
}

impl DialLock {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let turns = s.lines().map(Turn::new).collect();
        Self { turns }
    }

    #[must_use]
    pub fn get_end_zeroes(self) -> i16 {
        let mut zeroes = 0;
        let mut curr = START;
        let Self { turns } = self;
        for turn in turns {
            (_, curr) = do_move(curr, turn);
            if curr == 0 {
                zeroes += 1;
            }
        }
        zeroes
    }

    #[must_use]
    pub fn get_all_zeroes(self) -> i16 {
        let mut last_turn = None;
        let mut zeroes = 0;
        let mut curr = START;
        let Self { turns } = self;
        for turn in turns {
            if curr == 0 {
                zeroes += zero_correction(last_turn, turn);
            }

            let full_rot;
            (full_rot, curr) = do_move(curr, turn);
            zeroes += full_rot.abs();
            last_turn = Some(turn);
        }
        zeroes
    }
}
