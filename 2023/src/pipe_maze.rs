use std::iter::zip;

use crate::array_2d::{Array2d, Dir, Indices};

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
            let mut it = map.neighbors_enumerated(start).filter_map(|(dest, _)| {
                let direction = dir_from_diff(start, dest);
                match (direction, map[dest]) {
                    (Dir::Up | Dir::Down, Tile::UpDown)
                    | (Dir::Up | Dir::Left, Tile::RightDown)
                    | (Dir::Up | Dir::Right, Tile::DownLeft)
                    | (Dir::Right | Dir::Down, Tile::UpLeft)
                    | (Dir::Right | Dir::Left, Tile::RightLeft)
                    | (Dir::Down | Dir::Left, Tile::UpRight) => Some(direction),
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
    fn from_dirs(d1: Dir, d2: Dir) -> Self {
        match (d1, d2) {
            (Dir::Up, Dir::Right) | (Dir::Right, Dir::Up) => Self::UpRight,
            (Dir::Up, Dir::Down) | (Dir::Down, Dir::Up) => Self::UpDown,
            (Dir::Up, Dir::Left) | (Dir::Left, Dir::Up) => Self::UpLeft,
            (Dir::Right, Dir::Down) | (Dir::Down, Dir::Right) => Self::RightDown,
            (Dir::Right, Dir::Left) | (Dir::Left, Dir::Right) => Self::RightLeft,
            (Dir::Down, Dir::Left) | (Dir::Left, Dir::Down) => Self::DownLeft,
            _ => panic!("invalid dir combination: {d1:?} {d2:?}"),
        }
    }

    #[must_use]
    #[allow(clippy::match_same_arms)]
    fn next_dir(self, from: Dir) -> Dir {
        match (self, from) {
            (Self::UpRight, Dir::Up) => Dir::Right,
            (Self::UpRight, Dir::Right) => Dir::Up,
            (Self::UpDown, Dir::Up) => Dir::Down,
            (Self::UpDown, Dir::Down) => Dir::Up,
            (Self::UpLeft, Dir::Up) => Dir::Left,
            (Self::UpLeft, Dir::Left) => Dir::Up,
            (Self::RightDown, Dir::Right) => Dir::Down,
            (Self::RightDown, Dir::Down) => Dir::Right,
            (Self::RightLeft, Dir::Right) => Dir::Left,
            (Self::RightLeft, Dir::Left) => Dir::Right,
            (Self::DownLeft, Dir::Down) => Dir::Left,
            (Self::DownLeft, Dir::Left) => Dir::Down,
            _ => panic!("can't navigate {self:?} {from:?}"),
        }
    }

    #[must_use]
    fn dirs(self) -> (Dir, Dir) {
        match self {
            Self::UpRight => (Dir::Up, Dir::Right),
            Self::UpDown => (Dir::Up, Dir::Down),
            Self::UpLeft => (Dir::Up, Dir::Left),
            Self::RightDown => (Dir::Right, Dir::Down),
            Self::RightLeft => (Dir::Right, Dir::Left),
            Self::DownLeft => (Dir::Down, Dir::Left),
            _ => panic!("{self:?} doesn't have dirs"),
        }
    }
}

/// Which direction is `dest` from `src`?
///
/// `src` and `dest` must be one away in a cardinal direction
fn dir_from_diff(src: Indices<usize>, dest: Indices<usize>) -> Dir {
    match (
        isize::try_from(dest.col).unwrap() - isize::try_from(src.col).unwrap(),
        isize::try_from(dest.row).unwrap() - isize::try_from(src.row).unwrap(),
    ) {
        (0, -1) => Dir::Up,
        (1, 0) => Dir::Right,
        (0, 1) => Dir::Down,
        (-1, 0) => Dir::Left,
        _ => panic!("src and dest aren't 1 away in a cardinal direction"),
    }
}
