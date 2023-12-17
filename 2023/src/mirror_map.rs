use crate::array_2d::Array2d;
use std::{
    collections::HashSet,
    ops::{Add, Neg},
};

pub struct MirrorMap {
    map: Array2d<Tile>,
}

impl MirrorMap {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines();
        let dim = (
            it.clone().count(),
            it.clone().peekable().peek().unwrap().len(),
        );
        let data = it.flat_map(|l| l.chars().map(Tile::new)).collect();
        let map = Array2d::new(dim, data);

        Self { map }
    }

    #[must_use]
    pub fn num_energized(&self, start: (usize, usize), dir: Dir) -> usize {
        let data = vec![HashSet::new(); self.map.dim().0 * self.map.dim().1];
        let mut light = Array2d::new(self.map.dim(), data);
        let mut currs = vec![(start, dir)];
        while !currs.is_empty() {
            let mut new_currs = Vec::new();
            for curr in currs {
                if !light[curr.0].contains(&curr.1) {
                    light[curr.0].insert(curr.1);
                    new_currs.extend(self.map[curr.0].next_dir(curr.1).into_iter().filter_map(
                        |dir| {
                            (curr.0 + dir).and_then(|x| {
                                (x.0 < self.map.dim().1 && x.1 < self.map.dim().0)
                                    .then_some((x, dir))
                            })
                        },
                    ));
                }
            }
            currs = new_currs;
        }
        light.into_iter().filter(|x| !x.is_empty()).count()
    }

    #[must_use]
    pub fn dim(&self) -> (usize, usize) {
        self.map.dim()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorUpRight,
    MirrorDownRight,
    SplitterUpDown,
    SplitterRightLeft,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::MirrorUpRight,
            '\\' => Self::MirrorDownRight,
            '|' => Self::SplitterUpDown,
            '-' => Self::SplitterRightLeft,
            _ => panic!("unknown tile: {c}"),
        }
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    fn next_dir(self, from: Dir) -> Vec<Dir> {
        match (self, from) {
            (Self::Empty, _) => vec![from],
            (Self::MirrorUpRight, Dir::Up) => vec![Dir::Right],
            (Self::MirrorUpRight, Dir::Right) => vec![Dir::Up],
            (Self::MirrorUpRight, Dir::Down) => vec![Dir::Left],
            (Self::MirrorUpRight, Dir::Left) => vec![Dir::Down],
            (Self::MirrorDownRight, Dir::Up) => vec![Dir::Left],
            (Self::MirrorDownRight, Dir::Right) => vec![Dir::Down],
            (Self::MirrorDownRight, Dir::Down) => vec![Dir::Right],
            (Self::MirrorDownRight, Dir::Left) => vec![Dir::Up],
            (Self::SplitterUpDown, Dir::Right | Dir::Left) => vec![Dir::Up, Dir::Down],
            (Self::SplitterUpDown, Dir::Up | Dir::Down) => vec![from],
            (Self::SplitterRightLeft, Dir::Right | Dir::Left) => vec![from],
            (Self::SplitterRightLeft, Dir::Up | Dir::Down) => vec![Dir::Right, Dir::Left],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Neg for Dir {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

impl Add<Dir> for (usize, usize) {
    type Output = Option<(usize, usize)>;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => usize::checked_sub(self.1, 1).map(|y| (self.0, y)),
            Dir::Right => usize::checked_add(self.0, 1).map(|x| (x, self.1)),
            Dir::Down => usize::checked_add(self.1, 1).map(|y| (self.0, y)),
            Dir::Left => usize::checked_sub(self.0, 1).map(|x| (x, self.1)),
        }
    }
}
