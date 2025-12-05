use crate::array_2d::{IndexCompatible, Indices};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir8 {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Neg for Dir8 {
    type Output = Dir8;

    fn neg(self) -> Self::Output {
        match self {
            Dir8::Up => Dir8::Down,
            Dir8::UpRight => Dir8::DownLeft,
            Dir8::Right => Dir8::Left,
            Dir8::RightDown => Dir8::UpLeft,
            Dir8::Down => Dir8::Up,
            Dir8::DownLeft => Dir8::UpRight,
            Dir8::Left => Dir8::Right,
            Dir8::UpLeft => Dir8::RightDown,
        }
    }
}

impl<I> Add<Dir8> for Indices<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Output = Option<Indices<I>>;

    fn add(self, rhs: Dir8) -> Self::Output {
        match rhs {
            Dir8::Up => self
                .row
                .checked_sub(&1.try_into().unwrap())
                .map(|row| Indices { row, col: self.col }),
            Dir8::UpRight => self
                .row
                .checked_sub(&1.try_into().unwrap())
                .map(|row| Indices {
                    row,
                    col: self.col + 1.try_into().unwrap(),
                }),
            Dir8::Right => Some(Indices {
                row: self.row,
                col: self.col + 1.try_into().unwrap(),
            }),
            Dir8::RightDown => Some(Indices {
                row: self.row + 1.try_into().unwrap(),
                col: self.col + 1.try_into().unwrap(),
            }),
            Dir8::Down => Some(Indices {
                row: self.row + 1.try_into().unwrap(),
                col: self.col,
            }),
            Dir8::DownLeft => self
                .col
                .checked_sub(&1.try_into().unwrap())
                .map(|col| Indices {
                    row: self.row + 1.try_into().unwrap(),
                    col,
                }),
            Dir8::Left => self
                .col
                .checked_sub(&1.try_into().unwrap())
                .map(|col| Indices { row: self.row, col }),
            Dir8::UpLeft => self
                .row
                .checked_sub(&1.try_into().unwrap())
                .and_then(|row| {
                    self.col
                        .checked_sub(&1.try_into().unwrap())
                        .map(|col| Indices { row, col })
                }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dir8Dist<T>(pub Dir8, pub T);

impl<I> Add<Dir8Dist<I>> for Indices<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Output = Option<Indices<I>>;

    fn add(self, rhs: Dir8Dist<I>) -> Self::Output {
        match rhs {
            Dir8Dist(Dir8::Up, dist) => self
                .row
                .checked_sub(&dist)
                .map(|row| Indices { row, col: self.col }),
            Dir8Dist(Dir8::UpRight, dist) => self.row.checked_sub(&dist).map(|row| Indices {
                row,
                col: self.col + dist,
            }),
            Dir8Dist(Dir8::Right, dist) => Some(Indices {
                row: self.row,
                col: self.col + dist,
            }),
            Dir8Dist(Dir8::RightDown, dist) => Some(Indices {
                row: self.row + dist,
                col: self.col + dist,
            }),
            Dir8Dist(Dir8::Down, dist) => Some(Indices {
                row: self.row + dist,
                col: self.col,
            }),
            Dir8Dist(Dir8::DownLeft, dist) => self.col.checked_sub(&dist).map(|col| Indices {
                row: self.row + dist,
                col,
            }),
            Dir8Dist(Dir8::Left, dist) => self
                .col
                .checked_sub(&dist)
                .map(|col| Indices { row: self.row, col }),
            Dir8Dist(Dir8::UpLeft, dist) => self
                .row
                .checked_sub(&dist)
                .and_then(|row| self.col.checked_sub(&dist).map(|col| Indices { row, col })),
        }
    }
}

impl<T> Mul<T> for Dir8 {
    type Output = Dir8Dist<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Dir8Dist(self, rhs)
    }
}
