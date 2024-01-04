use crate::{
    array_2d::{Array2d, Dimensions, Indices},
    dir::Dir4,
};
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
    pub fn num_energized(&self, start: Indices<usize>, dir: Dir4) -> usize {
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
    fn next_dir(self, from: Dir4) -> Vec<Dir4> {
        match (self, from) {
            (Self::Empty, _) => vec![from],
            (Self::MirrorUpRight, Dir4::Up) => vec![Dir4::Right],
            (Self::MirrorUpRight, Dir4::Right) => vec![Dir4::Up],
            (Self::MirrorUpRight, Dir4::Down) => vec![Dir4::Left],
            (Self::MirrorUpRight, Dir4::Left) => vec![Dir4::Down],
            (Self::MirrorDownRight, Dir4::Up) => vec![Dir4::Left],
            (Self::MirrorDownRight, Dir4::Right) => vec![Dir4::Down],
            (Self::MirrorDownRight, Dir4::Down) => vec![Dir4::Right],
            (Self::MirrorDownRight, Dir4::Left) => vec![Dir4::Up],
            (Self::SplitterUpDown, Dir4::Right | Dir4::Left) => vec![Dir4::Up, Dir4::Down],
            (Self::SplitterUpDown, Dir4::Up | Dir4::Down) => vec![from],
            (Self::SplitterRightLeft, Dir4::Right | Dir4::Left) => vec![from],
            (Self::SplitterRightLeft, Dir4::Up | Dir4::Down) => vec![Dir4::Right, Dir4::Left],
        }
    }
}
