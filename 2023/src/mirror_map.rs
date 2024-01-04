use crate::array_2d::{Array2d, Dimensions, Dir, Indices};
use std::collections::HashSet;

pub struct MirrorMap {
    map: Array2d<usize, Tile>,
}

impl MirrorMap {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let map = Array2d::from_grid(s, Tile::new);

        Self { map }
    }

    #[must_use]
    pub fn num_energized(&self, start: Indices<usize>, dir: Dir) -> usize {
        let data = vec![HashSet::new(); self.map.n_cells()];
        let mut light = Array2d::new(self.map.dims().clone(), data);
        let mut currs = vec![(start, dir)];
        while !currs.is_empty() {
            let mut new_currs = Vec::new();
            for curr in currs {
                if !light[curr.0].contains(&curr.1) {
                    light[curr.0].insert(curr.1);
                    new_currs.extend(self.map[curr.0].next_dir(curr.1).into_iter().filter_map(
                        |dir| {
                            (curr.0 + dir).and_then(|indices| {
                                (self.map.dims().cols.contains(&indices.col)
                                    && self.map.dims().rows.contains(&indices.row))
                                .then_some((indices, dir))
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
    pub fn dims(&self) -> Dimensions<usize> {
        self.map.dims().clone()
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
    fn new(b: u8) -> Self {
        match b {
            b'.' => Self::Empty,
            b'/' => Self::MirrorUpRight,
            b'\\' => Self::MirrorDownRight,
            b'|' => Self::SplitterUpDown,
            b'-' => Self::SplitterRightLeft,
            _ => panic!("unknown tile: {b}"),
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
            _ => panic!("invalid direction: {from:?}"),
        }
    }
}
