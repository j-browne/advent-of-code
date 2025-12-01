use crate::{
    array_2d::{Array2d, Indices},
    dir::Dir4,
};
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Boxes,
    Robot,
}

impl Tile {
    fn new(c: u8) -> Self {
        match c {
            b'.' => Self::Empty,
            b'#' => Self::Wall,
            b'O' => Self::Boxes,
            b'@' => Self::Robot,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warehouse {
    map: Array2d<usize, Tile>,
    moves: Vec<Dir4>,
    pos: Indices<usize>,
}

impl Warehouse {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");
        let map = Array2d::from_grid(it.next().unwrap(), Tile::new);
        let moves = it
            .next()
            .unwrap()
            .bytes()
            .filter_map(|b| match b {
                b'^' => Some(Dir4::Up),
                b'>' => Some(Dir4::Right),
                b'v' => Some(Dir4::Down),
                b'<' => Some(Dir4::Left),
                _ => None,
            })
            .collect();
        let pos = map.position(|tile| *tile == Tile::Robot).unwrap();

        Self { map, moves, pos }
    }

    #[must_use]
    pub fn new_doubled(s: &str) -> Self {
        /*
        let mut it = s.split("\n\n");
        let map = todo!();
        let moves = it
            .next()
            .unwrap()
            .bytes()
            .filter_map(|b| match b {
                b'^' => Some(Dir4::Up),
                b'>' => Some(Dir4::Right),
                b'v' => Some(Dir4::Down),
                b'<' => Some(Dir4::Left),
                _ => None,
            })
            .collect();
        let pos = map.position(|tile| *tile == Tile::Robot).unwrap();

        Self { map, moves, pos }
        */
        todo!()
    }

    fn step(&mut self, m: Dir4) {
        for i in 1.. {
            //println!("{i}");
            if let Some(tile) = self.map.get_opt(self.pos + m * i) {
                //println!("* {:?} {:?} {:?}", self.pos, self.pos + m * i, tile);
                let first = (self.pos + m).unwrap();
                let last = (self.pos + m * i).unwrap();
                match *tile {
                    Tile::Wall => {
                        //println!(" 1");
                        break;
                    }
                    Tile::Empty => {
                        //println!(" 2");
                        self.map.swap(first, last);
                        self.map.swap(first, self.pos);
                        self.pos = first;
                        break;
                    }
                    _ => {
                        //println!(" 3");
                    }
                }
            } else {
                //println!("#");
                break;
            }
        }
    }

    pub fn run(&mut self) {
        let moves = mem::take(&mut self.moves);
        for m in moves {
            self.step(m);
            //println!("{:?}", m);
            //self.map.print_with(print_tile);
            //println!();
        }
    }

    #[must_use]
    pub fn gps_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter(|(_idx, tile)| **tile == Tile::Boxes)
            .map(|(idx, _tile)| {
                let indices = self.map.indices(idx).unwrap();
                100 * indices.row + indices.col
            })
            .sum()
    }
}

#[allow(dead_code, clippy::trivially_copy_pass_by_ref)]
fn print_tile(t: &Tile) -> String {
    match t {
        Tile::Empty => String::from("."),
        Tile::Wall => String::from("#"),
        Tile::Boxes => String::from("O"),
        Tile::Robot => String::from("@"),
    }
}
