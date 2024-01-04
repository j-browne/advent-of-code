use crate::{
    array_2d::{Array2d, Indices},
    dir::Dir4,
};
use std::iter::zip;

pub struct Maze {
    start: Indices<usize>,
    map: Array2d<usize, Tile>,
}

impl Maze {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut map = Array2d::from_grid(s, Tile::new);

        let start = map.position(|x| *x == Tile::Start).unwrap();
        let new_tile = {
            let mut it = map.neighbors_enumerated_4(start).filter_map(|(dest, _)| {
                let direction = dir_from_diff(start, dest);
                match (direction, map[dest]) {
                    (Dir4::Up | Dir4::Down, Tile::UpDown)
                    | (Dir4::Up | Dir4::Left, Tile::RightDown)
                    | (Dir4::Up | Dir4::Right, Tile::DownLeft)
                    | (Dir4::Right | Dir4::Down, Tile::UpLeft)
                    | (Dir4::Right | Dir4::Left, Tile::RightLeft)
                    | (Dir4::Down | Dir4::Left, Tile::UpRight) => Some(direction),
                    _ => None,
                }
            });
            Tile::from_dirs(it.next().unwrap(), it.next().unwrap())
        };
        map[start] = new_tile;

        Maze { start, map }
    }

    #[must_use]
    pub fn furthest_in_loop(&self) -> u32 {
        self.calculate_distances()
            .data()
            .iter()
            .filter_map(|x| *x)
            .max()
            .unwrap()
    }

    #[must_use]
    pub fn num_enclosed(&self) -> u32 {
        let distances = self.calculate_distances();

        let mut inside = false;
        let mut last_turn = None;
        let mut num_inside = 0;

        // Each row should start and end with `inside` being false.
        // Because of this, we can use `iter`
        // It would be more "correct" to iterate over the rows, since it would not depend on the
        // internal representation the data.
        for (map, dist) in zip(self.map.data().iter(), distances.data().iter()) {
            match (dist, last_turn, map) {
                (None, _, _) => {
                    if inside {
                        num_inside += 1;
                    }
                }
                (Some(_), None, Tile::UpRight | Tile::RightDown) => {
                    last_turn = Some(*map);
                }
                (Some(_), Some(Tile::UpRight), Tile::UpLeft)
                | (Some(_), Some(Tile::RightDown), Tile::DownLeft) => {
                    last_turn = None;
                }
                (Some(_), Some(Tile::UpRight), Tile::DownLeft)
                | (Some(_), Some(Tile::RightDown), Tile::UpLeft) => {
                    inside = !inside;
                    last_turn = None;
                }
                (Some(_), Some(_), Tile::RightLeft) => {}
                (Some(_), None, Tile::UpDown) => {
                    inside = !inside;
                }
                _ => panic!("invalid state: ({dist:?} {last_turn:?} {map:?})"),
            }
        }

        num_inside
    }

    #[must_use]
    fn calculate_distances(&self) -> Array2d<usize, Option<u32>> {
        let mut distances =
            Array2d::new(self.map.dims().clone(), vec![None; self.map.data().len()]);

        distances[self.start] = Some(0);
        let mut curr_distance = 1;
        let start_dirs = self.map[self.start].dirs();
        let mut curr_dirs = [
            (self.start + start_dirs.0, start_dirs.0),
            (self.start + start_dirs.1, start_dirs.1),
        ];

        while curr_dirs[0].0 != curr_dirs[1].0 {
            for (curr, dir) in &mut curr_dirs {
                distances[curr.unwrap()] = Some(curr_distance);
                *dir = self.map[curr.unwrap()].next_dir(-*dir);
                *curr = curr.unwrap() + *dir;
            }
            curr_distance += 1;
        }
        distances[curr_dirs[0].0.unwrap()] = Some(curr_distance);

        distances
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    UpRight,
    UpDown,
    UpLeft,
    RightDown,
    RightLeft,
    DownLeft,
}

impl Tile {
    #[must_use]
    fn new(b: u8) -> Self {
        match b {
            b'.' => Self::Empty,
            b'S' => Self::Start,
            b'L' => Self::UpRight,
            b'|' => Self::UpDown,
            b'J' => Self::UpLeft,
            b'F' => Self::RightDown,
            b'-' => Self::RightLeft,
            b'7' => Self::DownLeft,
            _ => panic!("unknown tile: {b}"),
        }
    }

    #[must_use]
    fn from_dirs(d1: Dir4, d2: Dir4) -> Self {
        match (d1, d2) {
            (Dir4::Up, Dir4::Right) | (Dir4::Right, Dir4::Up) => Self::UpRight,
            (Dir4::Up, Dir4::Down) | (Dir4::Down, Dir4::Up) => Self::UpDown,
            (Dir4::Up, Dir4::Left) | (Dir4::Left, Dir4::Up) => Self::UpLeft,
            (Dir4::Right, Dir4::Down) | (Dir4::Down, Dir4::Right) => Self::RightDown,
            (Dir4::Right, Dir4::Left) | (Dir4::Left, Dir4::Right) => Self::RightLeft,
            (Dir4::Down, Dir4::Left) | (Dir4::Left, Dir4::Down) => Self::DownLeft,
            _ => panic!("invalid dir combination: {d1:?} {d2:?}"),
        }
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    fn next_dir(self, from: Dir4) -> Dir4 {
        match (self, from) {
            (Self::UpRight, Dir4::Up) => Dir4::Right,
            (Self::UpRight, Dir4::Right) => Dir4::Up,
            (Self::UpDown, Dir4::Up) => Dir4::Down,
            (Self::UpDown, Dir4::Down) => Dir4::Up,
            (Self::UpLeft, Dir4::Up) => Dir4::Left,
            (Self::UpLeft, Dir4::Left) => Dir4::Up,
            (Self::RightDown, Dir4::Right) => Dir4::Down,
            (Self::RightDown, Dir4::Down) => Dir4::Right,
            (Self::RightLeft, Dir4::Right) => Dir4::Left,
            (Self::RightLeft, Dir4::Left) => Dir4::Right,
            (Self::DownLeft, Dir4::Down) => Dir4::Left,
            (Self::DownLeft, Dir4::Left) => Dir4::Down,
            _ => panic!("can't navigate {self:?} {from:?}"),
        }
    }

    #[must_use]
    fn dirs(self) -> (Dir4, Dir4) {
        match self {
            Self::UpRight => (Dir4::Up, Dir4::Right),
            Self::UpDown => (Dir4::Up, Dir4::Down),
            Self::UpLeft => (Dir4::Up, Dir4::Left),
            Self::RightDown => (Dir4::Right, Dir4::Down),
            Self::RightLeft => (Dir4::Right, Dir4::Left),
            Self::DownLeft => (Dir4::Down, Dir4::Left),
            _ => panic!("{self:?} doesn't have dirs"),
        }
    }
}

/// Which direction is `dest` from `src`?
///
/// `src` and `dest` must be one away in a cardinal direction
fn dir_from_diff(src: Indices<usize>, dest: Indices<usize>) -> Dir4 {
    match (
        isize::try_from(dest.col).unwrap() - isize::try_from(src.col).unwrap(),
        isize::try_from(dest.row).unwrap() - isize::try_from(src.row).unwrap(),
    ) {
        (0, -1) => Dir4::Up,
        (1, 0) => Dir4::Right,
        (0, 1) => Dir4::Down,
        (-1, 0) => Dir4::Left,
        _ => panic!("src and dest aren't 1 away in a cardinal direction"),
    }
}
