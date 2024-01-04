use crate::dir::{Dir4, Dir8};
use num::CheckedSub;
use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Index, IndexMut, Mul, Range, Rem, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array2d<I, T> {
    dims: Dimensions<I>,
    data: Vec<T>,
}

impl<I, T> Array2d<I, T>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    #[must_use]
    pub fn new(dims: Dimensions<I>, data: Vec<T>) -> Self {
        assert_eq!(data.len(), (dims.n_cells()).try_into().unwrap());
        Self { dims, data }
    }

    #[must_use]
    pub fn from_grid<F>(s: &str, f: F) -> Self
    where
        F: FnMut(u8) -> T + Copy,
    {
        let it = s.lines();
        let dims = Dimensions {
            rows: I::try_from(0).unwrap()..I::try_from(it.clone().count()).unwrap(),
            cols: I::try_from(0).unwrap()..I::try_from(it.clone().next().unwrap().len()).unwrap(),
        };

        let data = it.flat_map(|l| l.bytes().map(f)).collect();
        Self { dims, data }
    }

    #[must_use]
    pub const fn dims(&self) -> &Dimensions<I> {
        &self.dims
    }

    #[must_use]
    pub fn n_cells(&self) -> I {
        self.dims.n_cells()
    }

    #[must_use]
    pub fn n_rows(&self) -> I {
        self.dims.n_rows()
    }

    #[must_use]
    pub fn n_cols(&self) -> I {
        self.dims.n_cols()
    }

    #[must_use]
    pub fn contains(&self, indices: Indices<I>) -> bool {
        self.dims.contains(indices)
    }

    #[must_use]
    pub const fn data(&self) -> &Vec<T> {
        &self.data
    }

    #[must_use]
    pub fn get(&self, indices: Indices<I>) -> Option<&T> {
        self.idx(indices).and_then(|idx| self.data.get(idx))
    }

    #[must_use]
    pub fn get_mut(&mut self, indices: Indices<I>) -> Option<&mut T> {
        self.idx(indices).and_then(|idx| self.data.get_mut(idx))
    }

    #[must_use]
    pub fn get_opt(&self, indices: Option<Indices<I>>) -> Option<&T> {
        self.idx(indices?).and_then(|idx| self.data.get(idx))
    }

    #[must_use]
    pub fn get_opt_mut(&mut self, indices: Option<Indices<I>>) -> Option<&mut T> {
        self.idx(indices?).and_then(|idx| self.data.get_mut(idx))
    }

    #[must_use]
    pub fn idx(&self, indices: Indices<I>) -> Option<usize> {
        self.dims.idx(indices)
    }

    #[must_use]
    pub fn indices(&self, idx: usize) -> Option<Indices<I>> {
        self.dims.indices(idx)
    }

    #[must_use]
    pub fn position<F>(&self, predicate: F) -> Option<Indices<I>>
    where
        F: FnMut(&T) -> bool,
    {
        self.data
            .iter()
            .position(predicate)
            .map(|idx| self.indices(idx).unwrap())
    }

    #[must_use]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    #[must_use]
    pub fn iter_indices_2d(&self, minor_dir: Dir4) -> IterIndices2d<I> {
        let major = match minor_dir {
            Dir4::Up => self.dims.cols.start,
            Dir4::Right => self.dims.rows.start,
            Dir4::Down => self.dims.cols.end,
            Dir4::Left => self.dims.rows.end,
        };
        IterIndices2d {
            dims: self.dims.clone(),
            major,
            minor_dir,
        }
    }

    pub fn neighbors_4(&self, indices: Indices<I>) -> impl Iterator<Item = &T> {
        [Dir4::Up, Dir4::Right, Dir4::Down, Dir4::Left]
            .into_iter()
            .filter_map(move |dir| (indices + dir).and_then(|indices| self.get(indices)))
    }

    pub fn neighbors_8(&self, indices: Indices<I>) -> impl Iterator<Item = &T> {
        [
            Dir8::Up,
            Dir8::UpRight,
            Dir8::Right,
            Dir8::RightDown,
            Dir8::Down,
            Dir8::DownLeft,
            Dir8::Left,
            Dir8::UpLeft,
        ]
        .into_iter()
        .filter_map(move |dir| (indices + dir).and_then(|indices| self.get(indices)))
    }

    pub fn neighbors_enumerated_4(
        &self,
        indices: Indices<I>,
    ) -> impl Iterator<Item = (Indices<I>, &T)> {
        [Dir4::Up, Dir4::Right, Dir4::Down, Dir4::Left]
            .into_iter()
            .filter_map(move |dir| {
                (indices + dir).and_then(|indices| self.get(indices).map(|t| (indices, t)))
            })
    }

    pub fn neighbors_enumerated_8(
        &self,
        indices: Indices<I>,
    ) -> impl Iterator<Item = (Indices<I>, &T)> {
        [
            Dir8::Up,
            Dir8::UpRight,
            Dir8::Right,
            Dir8::RightDown,
            Dir8::Down,
            Dir8::DownLeft,
            Dir8::Left,
            Dir8::UpLeft,
        ]
        .into_iter()
        .filter_map(move |dir| {
            (indices + dir).and_then(|indices| self.get(indices).map(|t| (indices, t)))
        })
    }
}

impl<I, T> Display for Array2d<I, T>
where
    T: Display,
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_indices_2d(Dir4::Right) {
            for indices in row {
                write!(f, "{}", self[indices])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<I, T> Index<Indices<I>> for Array2d<I, T>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Output = T;

    fn index(&self, indices: Indices<I>) -> &Self::Output {
        self.get(indices).unwrap()
    }
}

impl<I, T> IndexMut<Indices<I>> for Array2d<I, T>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    fn index_mut(&mut self, indices: Indices<I>) -> &mut Self::Output {
        self.get_mut(indices).unwrap()
    }
}

impl<I, T> IntoIterator for Array2d<I, T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, I, T> IntoIterator for &'a Array2d<I, T> {
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, I, T> IntoIterator for &'a mut Array2d<I, T> {
    type Item = &'a mut T;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IterIndices2d<I> {
    dims: Dimensions<I>,
    major: I,
    minor_dir: Dir4,
}

impl<I> Iterator for IterIndices2d<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
{
    type Item = IterIndices2dInner<I>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.minor_dir {
            Dir4::Up => {
                let next = self
                    .dims
                    .cols
                    .contains(&self.major)
                    .then(|| IterIndices2dInner {
                        dims: self.dims.clone(),
                        major: self.major,
                        minor: Some(self.dims.rows.end - 1.try_into().unwrap()),
                        minor_dir: self.minor_dir,
                    });
                self.major = self.major + 1.try_into().unwrap();
                next
            }
            Dir4::Right => {
                let next = self
                    .dims
                    .rows
                    .contains(&self.major)
                    .then(|| IterIndices2dInner {
                        dims: self.dims.clone(),
                        major: self.major,
                        minor: Some(self.dims.cols.start),
                        minor_dir: self.minor_dir,
                    });
                self.major = self.major + 1.try_into().unwrap();
                next
            }
            Dir4::Down => {
                self.major = I::checked_sub(&self.major, &I::try_from(1).unwrap())?;
                self.dims
                    .cols
                    .contains(&self.major)
                    .then(|| IterIndices2dInner {
                        dims: self.dims.clone(),
                        major: self.major,
                        minor: Some(self.dims.rows.start),
                        minor_dir: self.minor_dir,
                    })
            }
            Dir4::Left => {
                self.major = I::checked_sub(&self.major, &I::try_from(1).unwrap())?;
                self.dims
                    .rows
                    .contains(&self.major)
                    .then(|| IterIndices2dInner {
                        dims: self.dims.clone(),
                        major: self.major,
                        minor: Some(self.dims.cols.end - 1.try_into().unwrap()),
                        minor_dir: self.minor_dir,
                    })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IterIndices2dInner<I> {
    dims: Dimensions<I>,
    major: I,
    minor: Option<I>,
    minor_dir: Dir4,
}

impl<I> Iterator for IterIndices2dInner<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    type Item = Indices<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let minor = self.minor?;
        let next = match self.minor_dir {
            Dir4::Up | Dir4::Down => self.dims.rows.contains(&minor).then_some(Indices {
                row: minor,
                col: self.major,
            }),
            Dir4::Right | Dir4::Left => self.dims.cols.contains(&minor).then_some(Indices {
                row: self.major,
                col: minor,
            }),
        };
        match self.minor_dir {
            Dir4::Up | Dir4::Left => self.minor = I::checked_sub(&minor, &1.try_into().unwrap()),
            Dir4::Right | Dir4::Down => self.minor = Some(minor + 1.try_into().unwrap()),
        }
        next
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions<I> {
    pub rows: Range<I>,
    pub cols: Range<I>,
}

impl<I> Dimensions<I>
where
    I: IndexCompatible,
    <I as TryFrom<usize>>::Error: Debug,
    <I as TryInto<usize>>::Error: Debug,
{
    #[must_use]
    pub fn n_cells(&self) -> I {
        self.n_cols() * self.n_rows()
    }

    #[must_use]
    pub fn n_rows(&self) -> I {
        self.rows.end - self.rows.start
    }

    #[must_use]
    pub fn n_cols(&self) -> I {
        self.cols.end - self.cols.start
    }

    #[must_use]
    pub fn contains(&self, indices: Indices<I>) -> bool {
        self.rows.contains(&indices.row) && self.cols.contains(&indices.col)
    }

    #[must_use]
    pub fn idx(&self, indices: Indices<I>) -> Option<usize> {
        (self.contains(indices)).then_some(
            (self.n_cols() * (indices.row - self.rows.start) + (indices.col - self.cols.start))
                .try_into()
                .ok()?,
        )
    }

    #[must_use]
    pub fn indices(&self, idx: usize) -> Option<Indices<I>> {
        (idx < (self.n_cells()).try_into().unwrap()).then(|| {
            let row = I::try_from(idx).unwrap() / self.n_cols() + self.rows.start;
            let col = I::try_from(idx).unwrap() % self.n_cols() + self.cols.start;
            Indices { row, col }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Indices<I>
where
    I: IndexCompatible,
{
    pub row: I,
    pub col: I,
}

pub trait IndexCompatible:
    Debug
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + CheckedSub
    + TryInto<usize>
    + TryFrom<usize>
{
}

impl<T> IndexCompatible for T where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + CheckedSub
        + TryInto<usize>
        + TryFrom<usize>
{
}
