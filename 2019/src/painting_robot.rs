use crate::intcode::{self, Machine, Return};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Left => self.rotate_left(),
            Rotation::Right => self.rotate_right(),
        }
    }

    fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub machine: Machine,
    pub facing: Direction,
    pub position: Position,
    pub painted_tiles: HashMap<Position, Color>,
}

impl Robot {
    pub fn with_machine(machine: Machine) -> Self {
        Self {
            machine,
            facing: Direction::Up,
            position: Default::default(),
            painted_tiles: Default::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), intcode::Error> {
        loop {
            match self.machine.run()? {
                Return::Stopped => break,
                Return::EmptyInput => {
                    self.advance();
                    self.camera_input();
                }
            }
        }

        Ok(())
    }

    fn advance(&mut self) {
        while !self.machine.output.is_empty() {
            let color = match self.machine.output.pop_front().unwrap() {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("invalid color code"),
            };
            let rotation = match self.machine.output.pop_front().unwrap() {
                0 => Rotation::Left,
                1 => Rotation::Right,
                _ => panic!("invalid rotation code"),
            };

            let _ = self.painted_tiles.insert(self.position, color);
            self.facing = self.facing.rotate(rotation);

            match self.facing {
                Direction::Up => self.position.y += 1,
                Direction::Right => self.position.x += 1,
                Direction::Down => self.position.y -= 1,
                Direction::Left => self.position.x -= 1,
            }
        }
    }

    fn camera_input(&mut self) {
        self.machine.input.push_back(
            match self
                .painted_tiles
                .get(&self.position)
                .unwrap_or(&Color::Black)
            {
                Color::Black => 0,
                Color::White => 1,
            },
        );
    }

    pub fn num_painted(&self) -> u32 {
        self.painted_tiles.len() as u32
    }

    pub fn painted_image(&self) -> String {
        let mut output = String::new();

        let xmin = self.painted_tiles.iter().map(|(x, _)| x.x).min().unwrap();
        let ymin = self.painted_tiles.iter().map(|(x, _)| x.y).min().unwrap();
        let xmax = self.painted_tiles.iter().map(|(x, _)| x.x).max().unwrap();
        let ymax = self.painted_tiles.iter().map(|(x, _)| x.y).max().unwrap();

        output.push('╔');
        for _ in xmin..=xmax {
            output.push('═');
        }
        output.push('╗');
        output.push('\n');

        for y in (ymin..=ymax).rev() {
            output.push('║');
            for x in xmin..=xmax {
                match self
                    .painted_tiles
                    .get(&Position { x, y })
                    .unwrap_or(&Color::Black)
                {
                    Color::Black => output.push(' '),
                    Color::White => output.push('█'),
                };
            }
            output.push('║');
            output.push('\n');
        }

        output.push('╚');
        for _ in xmin..=xmax {
            output.push('═');
        }
        output.push('╝');

        output
    }
}
