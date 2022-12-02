#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyGuide {
    throws: Vec<(Throw, Throw)>,
}

impl StrategyGuide {
    #[must_use]
    pub fn with_throw(s: &str) -> Self {
        let throws = s
            .lines()
            .map(|s| {
                let mut x = s.split_whitespace();
                let throw_1 = Throw::new(x.next().unwrap());
                let throw_2 = Throw::with_mistaken(x.next().unwrap());
                (throw_1, throw_2)
            })
            .collect();
        Self { throws }
    }

    #[must_use]
    pub fn with_res(s: &str) -> Self {
        let throws = s
            .lines()
            .map(|s| {
                let mut x = s.split_whitespace();
                let throw_1 = Throw::new(x.next().unwrap());
                let res = Res::new(x.next().unwrap());
                let throw_2 = throw_1.get_res(res);
                (throw_1, throw_2)
            })
            .collect();
        Self { throws }
    }

    #[must_use]
    pub fn score(&self) -> u32 {
        self.throws.iter().map(|(a, b)| b.match_score(*a)).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn new(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("invalid throw '{s}'"),
        }
    }

    fn with_mistaken(s: &str) -> Self {
        match s {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("invalid throw '{s}'"),
        }
    }

    const fn get_res(self, res: Res) -> Self {
        match (self, res) {
            (Self::Rock, Res::Draw) | (Self::Scissors, Res::Win) | (Self::Paper, Res::Loss) => {
                Self::Rock
            }
            (Self::Rock, Res::Win) | (Self::Scissors, Res::Loss) | (Self::Paper, Res::Draw) => {
                Self::Paper
            }
            (Self::Rock, Res::Loss) | (Self::Scissors, Res::Draw) | (Self::Paper, Res::Win) => {
                Self::Scissors
            }
        }
    }

    const fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    const fn match_score(self, other: Self) -> u32 {
        self.score() + self.res(other).score()
    }

    const fn res(self, other: Self) -> Res {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Res::Win,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Res::Loss,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => Res::Draw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Res {
    Win,
    Loss,
    Draw,
}

impl Res {
    fn new(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid throw '{s}'"),
        }
    }

    const fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}
