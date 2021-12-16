use std::{
    fmt::Debug,
    ops::{Add, Index, IndexMut, Mul},
};

// TODO: should data be a Box<[u8]>?
// TODO: should it be called something else? (Not really a Vec)
#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    /// Dimensionality of the data
    dim: (usize, usize),
    /// The data itself
    data: Vec<T>,
}

impl<T> Vec2d<T> {
    #[must_use]
    pub fn new(dim: (usize, usize), data: Vec<T>) -> Self {
        assert_eq!(data.len(), dim.0 * dim.1);
        Self { dim, data }
    }

    #[must_use]
    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    #[must_use]
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    #[inline]
    fn idx<I>(&self, x: I, y: I) -> I
    where
        I: Mul<Output = I> + Add<Output = I> + TryFrom<usize>,
        <I as TryFrom<usize>>::Error: Debug,
    {
        I::try_from(self.dim.1).unwrap() * y + x
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = &T> + '_ {
        self.neighbor_indices(x, y).map(|(x, y)| &self[(x, y)])
    }

    pub fn neighbors_diag(&self, x: usize, y: usize) -> impl Iterator<Item = &T> + '_ {
        self.neighbor_diag_indices(x, y).map(|(x, y)| &self[(x, y)])
    }

    pub fn neighbor_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let diffs = [(-1isize, 0isize), (0, -1), (1, 0), (0, 1)];
        diffs.into_iter().filter_map(move |(dx, dy)| {
            match (
                usize::try_from(isize::try_from(x).unwrap() + dx),
                usize::try_from(isize::try_from(y).unwrap() + dy),
            ) {
                (Ok(new_x), Ok(new_y)) if new_x >= self.dim.1 || new_y >= self.dim.0 => None,
                (Ok(new_x), Ok(new_y)) => Some((new_x, new_y)),
                _ => None,
            }
        })
    }
    pub fn neighbor_diag_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let diffs = [
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        diffs.into_iter().filter_map(move |(dx, dy)| {
            match (
                usize::try_from(isize::try_from(x).unwrap() + dx),
                usize::try_from(isize::try_from(y).unwrap() + dy),
            ) {
                (Ok(new_x), Ok(new_y)) if new_x >= self.dim.1 || new_y >= self.dim.0 => None,
                (Ok(new_x), Ok(new_y)) => Some((new_x, new_y)),
                _ => None,
            }
        })
    }

    #[must_use]
    #[allow(clippy::iter_not_returning_iterator)]
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    #[must_use]
    #[allow(clippy::iter_not_returning_iterator)]
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<T> Index<(usize, usize)> for Vec2d<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.dim.1);
        assert!(y < self.dim.0);
        let idx = self.idx(x, y);
        &self.data[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2d<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.dim.1);
        assert!(y < self.dim.0);
        let idx = self.idx(x, y);
        &mut self.data[idx]
    }
}

impl<T> IntoIterator for Vec2d<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Vec2d<T> {
    type Item = &'a T;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec2d<T> {
    type Item = &'a mut T;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
