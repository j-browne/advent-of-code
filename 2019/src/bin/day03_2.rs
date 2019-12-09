use std::{
    borrow::Borrow,
    collections::HashMap,
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

#[allow(clippy::suspicious_arithmetic_impl)]
impl<Rhs: Borrow<Move>> Add<Rhs> for &mut Position {
    type Output = Position;

    fn add(self, rhs: Rhs) -> Self::Output {
        use Direction::*;
        let rhs = rhs.borrow();

        let mut out = self;
        match rhs.direction {
            Up => out.1 += rhs.distance,
            Right => out.0 += rhs.distance,
            Down => out.1 -= rhs.distance,
            Left => out.0 -= rhs.distance,
        };

        *out
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<Rhs: AsRef<Move>> Add<Rhs> for &Position {
    type Output = Position;

    fn add(self, rhs: Rhs) -> Self::Output {
        use Direction::*;
        let rhs = rhs.as_ref();

        match rhs.direction {
            Up => self.1 + rhs.distance,
            Right => self.0 + rhs.distance,
            Down => self.1 - rhs.distance,
            Left => self.0 - rhs.distance,
        };
        *self
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<Rhs: AsRef<Move>> Add<Rhs> for Position {
    type Output = Position;

    fn add(self, rhs: Rhs) -> Self::Output {
        use Direction::*;
        let rhs = rhs.as_ref();

        match rhs.direction {
            Up => self.1 + rhs.distance,
            Right => self.0 + rhs.distance,
            Down => self.1 - rhs.distance,
            Left => self.0 - rhs.distance,
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
        Ok(Move {
            direction,
            distance,
        })
    }
}

fn main() {
    println!("{}", day03_2(stdin().lock()));
}

fn day03_2(input: impl BufRead) -> usize {
    let pos = input
        .lines()
        .take(2)
        .map(|line| {
            line.unwrap()
                .trim()
                .split(',')
                .map(|x| {
                    let Move {
                        direction,
                        distance,
                    } = Move::try_from(x).unwrap();
                    repeat(Move {
                        direction,
                        distance: 1,
                    })
                    .take(distance as usize)
                })
                .flatten()
                .scan(Position(0, 0), |p, m| Some(p + m))
                .enumerate()
                .map(|(a, b)| (b, a + 1))
                .collect::<HashMap<Position, usize>>()
        })
        .collect::<Vec<_>>();

    pos[0]
        .iter()
        .filter_map(|(k, v1)| pos[1].get(k).map(|v2| v1 + v2))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day03_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day03_2(BufReader::new(File::open("input/input_day03.txt").unwrap())),
            101386
        );
    }
}
