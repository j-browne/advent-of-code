use std::{
    iter::zip,
    ops::{Add, Neg},
};

use crate::array_2d::Array2d;

pub struct Maze {
    start: (usize, usize),
    map: Array2d<Tile>,
}

impl Maze {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.lines();
        let dim = (
            it.clone().count(),
            it.clone().peekable().peek().unwrap().len(),
        );

        let data = it.flat_map(|l| l.chars().map(Tile::new)).collect();
        let mut map = Array2d::new(dim, data);

        let start = map.position(|x| *x == Tile::Start).unwrap();
        let new_tile = {
            let mut it = map.neighbor_indices(start.0, start.1).filter_map(|dest| {
                let direction = Dir::from_diff(start, dest);
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
    fn calculate_distances(&self) -> Array2d<Option<u32>> {
        let mut distances = Array2d::new(self.map.dim(), vec![None; self.map.data().len()]);

        distances[self.start] = Some(0);
        let mut curr_distance = 1;
        let start_dirs = self.map[self.start].dirs();
        let mut curr_dirs = [
            (self.start + start_dirs.0, start_dirs.0),
            (self.start + start_dirs.1, start_dirs.1),
        ];

        while curr_dirs[0].0 != curr_dirs[1].0 {
            for (curr, dir) in &mut curr_dirs {
                distances[*curr] = Some(curr_distance);
                *dir = self.map[*curr].next_dir(-*dir);
                *curr = *curr + *dir;
            }
            curr_distance += 1;
        }
        distances[curr_dirs[0].0] = Some(curr_distance);

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
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'S' => Self::Start,
            'L' => Self::UpRight,
            '|' => Self::UpDown,
            'J' => Self::UpLeft,
            'F' => Self::RightDown,
            '-' => Self::RightLeft,
            '7' => Self::DownLeft,
            _ => panic!("unknown tile: {c}"),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    /// Which direction is `dest` from `src`?
    ///
    /// `src` and `dest` must be one away in a cardinal direction
    fn from_diff(src: (usize, usize), dest: (usize, usize)) -> Self {
        match (
            isize::try_from(dest.0).unwrap() - isize::try_from(src.0).unwrap(),
            isize::try_from(dest.1).unwrap() - isize::try_from(src.1).unwrap(),
        ) {
            (0, -1) => Self::Up,
            (1, 0) => Self::Right,
            (0, 1) => Self::Down,
            (-1, 0) => Self::Left,
            _ => panic!("src and dest aren't 1 away in a cardinal direction"),
        }
    }
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
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::Up => (self.0, usize::checked_sub(self.1, 1).unwrap()),
            Dir::Right => (usize::checked_add(self.0, 1).unwrap(), self.1),
            Dir::Down => (self.0, usize::checked_add(self.1, 1).unwrap()),
            Dir::Left => (usize::checked_sub(self.0, 1).unwrap(), self.1),
        }
    }
}
