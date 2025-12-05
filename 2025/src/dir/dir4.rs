use crate::array_2d::{IndexCompatible, Indices};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

impl Neg for Dir4 {
    type Output = Dir4;

    fn neg(self) -> Self::Output {
        match self {
            Dir4::Up => Dir4::Down,
            Dir4::Right => Dir4::Left,
            Dir4::Down => Dir4::Up,
            Dir4::Left => Dir4::Right,
        }
    }
}

impl<I> Add<Dir4> for Indices<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Output = Option<Indices<I>>;

    fn add(self, rhs: Dir4) -> Self::Output {
        match rhs {
            Dir4::Up => self
                .row
                .checked_sub(&1.try_into().unwrap())
                .map(|row| Indices { row, col: self.col }),
            Dir4::Right => Some(Indices {
                row: self.row,
                col: self.col + 1.try_into().unwrap(),
            }),
            Dir4::Down => Some(Indices {
                row: self.row + 1.try_into().unwrap(),
                col: self.col,
            }),
            Dir4::Left => self
                .col
                .checked_sub(&1.try_into().unwrap())
                .map(|col| Indices { row: self.row, col }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dir4Dist<T>(pub Dir4, pub T);

impl<I> Add<Dir4Dist<I>> for Indices<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Output = Option<Indices<I>>;

    fn add(self, rhs: Dir4Dist<I>) -> Self::Output {
        match rhs {
            Dir4Dist(Dir4::Up, dist) => self
                .row
                .checked_sub(&dist)
                .map(|row| Indices { row, col: self.col }),
            Dir4Dist(Dir4::Right, dist) => Some(Indices {
                row: self.row,
                col: self.col + dist,
            }),
            Dir4Dist(Dir4::Down, dist) => Some(Indices {
                row: self.row + dist,
                col: self.col,
            }),
            Dir4Dist(Dir4::Left, dist) => self
                .col
                .checked_sub(&dist)
                .map(|col| Indices { row: self.row, col }),
        }
    }
}

impl<T> Mul<T> for Dir4 {
    type Output = Dir4Dist<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Dir4Dist(self, rhs)
    }
}
