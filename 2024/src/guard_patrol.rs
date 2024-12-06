use crate::{
    array_2d::{Array2d, Indices},
    dir::Dir4,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unvisited,
    Visited([bool; 4]),
    Blocked,
    Guard,
}

impl Tile {
    fn new(c: u8) -> Self {
        match c {
            b'#' => Self::Blocked,
            b'^' => Self::Guard,
            _ => Self::Unvisited,
        }
    }

    fn visit(&mut self, dir: Dir4) {
        if let Self::Visited(mut dirs) = self {
            dirs[dir_to_i(dir)] = true;
        } else {
            let mut dirs = [false; 4];
            dirs[dir_to_i(dir)] = true;
            *self = Self::Visited(dirs);
        }
    }

    fn has_visited(self, dir: Dir4) -> bool {
        if let Self::Visited(dirs) = self {
            dirs[dir_to_i(dir)]
        } else {
            false
        }
    }
}

fn dir_to_i(dir: Dir4) -> usize {
    match dir {
        Dir4::Up => 0,
        Dir4::Right => 1,
        Dir4::Down => 2,
        Dir4::Left => 3,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    map: Array2d<usize, Tile>,
    guard_pos: Option<Indices<usize>>,
    guard_dir: Dir4,
}

impl Map {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let map = Array2d::from_grid(s, Tile::new);
        let guard_pos = map.position(|t| *t == Tile::Guard);
        Self {
            map,
            guard_pos,
            guard_dir: Dir4::Up,
        }
    }

    fn do_patrol(&mut self) {
        while let Some(pos) = self.guard_pos {
            self.map[pos].visit(self.guard_dir);

            // It's important not to move forward after you turn because the path might be blocked
            if self.map.get_opt(pos + self.guard_dir) == Some(&Tile::Blocked) {
                self.guard_dir = match self.guard_dir {
                    Dir4::Up => Dir4::Right,
                    Dir4::Right => Dir4::Down,
                    Dir4::Down => Dir4::Left,
                    Dir4::Left => Dir4::Up,
                };
            } else {
                self.guard_pos = pos + self.guard_dir;
            }

            // If the guard has left the map
            if self.guard_pos.is_none_or(|p| !self.map.contains(p)) {
                self.guard_pos = None;
                break;
            }

            // If the guard has entered a loop
            if self
                .guard_pos
                .is_some_and(|p| self.map[p].has_visited(self.guard_dir))
            {
                break;
            }
        }
    }

    #[must_use]
    pub fn num_visited(mut self) -> usize {
        self.do_patrol();
        self.map
            .data()
            .iter()
            .filter(|t| matches!(**t, Tile::Visited(_)))
            .count()
    }

    #[must_use]
    pub fn num_loop_candidates(self) -> usize {
        let to_check = {
            let mut map = self.clone();
            map.do_patrol();
            map.map
                .data()
                .iter()
                .enumerate()
                .filter(|(_idx, t)| matches!(*t, Tile::Visited(_)))
                .map(|(idx, _t)| map.map.indices(idx).unwrap())
                .collect::<Vec<_>>()
        };

        to_check
            .iter()
            .map(|new_block| {
                let mut map = self.clone();
                map.map[*new_block] = Tile::Blocked;
                map.do_patrol();
                usize::from(map.guard_pos.is_some())
            })
            .sum()
    }
}

#[allow(dead_code, clippy::trivially_copy_pass_by_ref)]
fn print_tile(t: &Tile) -> String {
    match t {
        Tile::Unvisited => String::from("."),
        Tile::Visited(_) => String::from("X"),
        Tile::Blocked => String::from("O"),
        Tile::Guard => String::from("^"),
    }
}
