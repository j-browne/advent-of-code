use std::{
    borrow::Borrow,
    collections::HashSet,
    convert::TryFrom,
    io::{stdin, BufRead},
    iter::repeat,
    num::ParseIntError,
    ops::Add,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownDirection,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<&str> for Direction {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(Self::Error::UnknownDirection),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl<Rhs: Borrow<Move>> Add<Rhs> for &mut Position {
    type Output = Position;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.borrow();

        let out = self;
        match rhs.direction {
            Direction::Up => out.1 += rhs.distance,
            Direction::Right => out.0 += rhs.distance,
            Direction::Down => out.1 -= rhs.distance,
            Direction::Left => out.0 -= rhs.distance,
        };

        *out
    }
}

impl<Rhs: AsRef<Move>> Add<Rhs> for &Position {
    type Output = Position;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.as_ref();

        match rhs.direction {
            Direction::Up => self.1 + rhs.distance,
            Direction::Right => self.0 + rhs.distance,
            Direction::Down => self.1 - rhs.distance,
            Direction::Left => self.0 - rhs.distance,
        };
        *self
    }
}

impl<Rhs: AsRef<Move>> Add<Rhs> for Position {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.as_ref();

        match rhs.direction {
            Direction::Up => self.1 + rhs.distance,
            Direction::Right => self.0 + rhs.distance,
            Direction::Down => self.1 - rhs.distance,
            Direction::Left => self.0 - rhs.distance,
        };
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    direction: Direction,
    distance: i32,
}

impl TryFrom<&str> for Move {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = Direction::try_from(&value[..=0])?;
        let distance = value[1..].parse()?;
        Ok(Self {
            direction,
            distance,
        })
    }
}

fn main() {
    println!("{}", day03_1(stdin().lock()));
}

fn day03_1(input: impl BufRead) -> i32 {
    let pos = input
        .lines()
        .take(2)
        .map(|line| {
            line.unwrap()
                .trim()
                .split(',')
                .flat_map(|x| {
                    let Move {
                        direction,
                        distance,
                    } = Move::try_from(x).unwrap();
                    repeat(Move {
                        direction,
                        distance: 1,
                    })
                    .take(usize::try_from(distance).unwrap())
                })
                .scan(Position(0, 0), |p, m| Some(p + m))
                .collect::<HashSet<Position>>()
        })
        .collect::<Vec<_>>();

    pos[0]
        .intersection(&pos[1])
        .map(|a| a.0.abs() + a.1.abs())
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day03_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day03_1(BufReader::new(File::open("input/input_day03.txt").unwrap())),
            1211
        );
    }
}
