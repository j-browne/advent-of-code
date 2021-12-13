use crate::intcode::{self, Machine, Return};
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    mem,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    UnknownTile,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            0 => Ok(Empty),
            1 => Ok(Wall),
            2 => Ok(Block),
            3 => Ok(Paddle),
            4 => Ok(Ball),
            _ => Err(Self::Error::UnknownTile),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use Tile::*;
        match self {
            Empty => write!(f, " ")?,
            Wall => write!(f, "\u{2588}")?,
            Block => write!(f, "\u{2592}")?,
            Paddle => write!(f, "\u{2594}")?,
            Ball => write!(f, "\u{25CF}")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Arcade {
    pub machine: Machine,
    tiles: HashMap<(i64, i64), Tile>,
}

impl Arcade {
    pub fn with_machine(machine: Machine) -> Self {
        let tiles = HashMap::new();
        Self { machine, tiles }
    }

    pub fn run(&mut self) -> Result<(), intcode::Error> {
        loop {
            match self.machine.run()? {
                Return::Stopped => break,
                Return::EmptyInput => {}
            }
        }

        Ok(())
    }

    pub fn render(&mut self) -> (String, Option<i64>) {
        let mut data = VecDeque::new();
        mem::swap(&mut self.machine.output, &mut data);
        let data = Vec::from(data);

        let mut score = None;

        for d in data.chunks(3) {
            if d[0] == -1 && d[1] == 0 {
                score = Some(d[2]);
            } else {
                self.tiles.insert((d[0], d[1]), Tile::try_from(d[2]).unwrap());
            }
        }

        let mut output = String::new();

        let x_min = self.tiles.iter().map(|(x, _)| x.0).min().unwrap_or(0);
        let y_min = self.tiles.iter().map(|(x, _)| x.1).min().unwrap_or(0);
        let x_max = self.tiles.iter().map(|(x, _)| x.0).max().unwrap_or(0);
        let y_max = self.tiles.iter().map(|(x, _)| x.1).max().unwrap_or(0);

        output.push('╔');
        for _ in x_min..=x_max {
            output.push('═');
        }
        output.push('╗');
        output.push('\n');

        for y in y_min..=y_max {
            output.push('║');
            for x in x_min..=x_max {
                output.extend(format!("{}", self.tiles.get(&(x, y)).unwrap_or(&Tile::Empty)).chars());
            }
            output.push('║');
            output.push('\n');
        }

        output.push('╚');
        for _ in x_min..=x_max {
            output.push('═');
        }
        output.push('╝');

        (output, score)
    }
}
