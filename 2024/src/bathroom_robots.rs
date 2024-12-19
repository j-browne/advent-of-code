use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Range},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BathroomRobots {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl BathroomRobots {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.lines();
        let (width, height);
        {
            let mut it = it.next().unwrap().split_whitespace();
            width = it.next().unwrap().parse().unwrap();
            height = it.next().unwrap().parse().unwrap();
        }

        let robots = it.map(Robot::new).collect();
        Self {
            width,
            height,
            robots,
        }
    }

    pub fn step(&mut self) {
        for robot in &mut self.robots {
            robot.pos = (robot.pos + robot.vel).wrap(self.width, self.height);
        }
    }

    #[must_use]
    pub fn safety_factor(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.step();
        }

        let mut quads = [0, 0, 0, 0];
        let regions = [
            Region {
                x: 0..self.width / 2,
                y: 0..self.height / 2,
            },
            Region {
                x: 0..self.width / 2,
                y: (self.height + 1) / 2..self.height,
            },
            Region {
                x: (self.width + 1) / 2..self.width,
                y: 0..self.height / 2,
            },
            Region {
                x: (self.width + 1) / 2..self.width,
                y: (self.height + 1) / 2..self.height,
            },
        ];
        for robot in &self.robots {
            if let Some(idx) = regions.iter().position(|region| region.contains(robot.pos)) {
                quads[idx] += 1;
            }
        }

        quads.into_iter().product()
    }

    #[must_use]
    pub fn easter_egg(&mut self) -> usize {
        let mut i = 0;

        // apparently, the easter egg coincides with the first occurrence of none of the robots
        // overlapping. That doesn't seem like that should be the case, but checking for that is a
        // lot faster than checking the max consecutive robots
        let mut map = HashMap::new();
        loop {
            if self.max_overlap(&mut map) == 1 {
                return i;
            }
            //if self.max_consecutive() > 10 {
            //    return i;
            //}
            self.step();
            i += 1;
        }
    }

    #[must_use]
    fn max_overlap(&self, map: &mut HashMap<Vec2, usize>) -> usize {
        map.clear();
        for robot in &self.robots {
            *map.entry(robot.pos).or_insert(0) += 1;
        }
        *map.values().max().unwrap()
    }

    #[must_use]
    #[allow(dead_code)]
    fn max_consecutive(&self) -> usize {
        let mut max_line = 0;

        for y in 0..self.height {
            let mut curr_line = 0;
            for x in 0..self.width {
                if self.robots.iter().any(|r| r.pos == Vec2 { x, y }) {
                    curr_line += 1;
                    max_line = max_line.max(curr_line);
                } else {
                    curr_line = 0;
                }
            }
        }

        max_line
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let n = self
                    .robots
                    .iter()
                    .filter(|r| r.pos == Vec2 { x, y })
                    .count();
                match n {
                    0 => print!("\u{2591}"),
                    1 => print!("\u{2592}"),
                    2 => print!("\u{2593}"),
                    _ => print!("\u{2588}"),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        let pos = {
            let mut it = it.next().unwrap().strip_prefix("p=").unwrap().split(',');
            Vec2 {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
            }
        };
        let vel = {
            let mut it = it.next().unwrap().strip_prefix("v=").unwrap().split(',');
            Vec2 {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
            }
        };

        Self { pos, vel }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn wrap(self, width: i32, height: i32) -> Self {
        Self {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    x: Range<i32>,
    y: Range<i32>,
}

impl Region {
    fn contains(&self, pos: Vec2) -> bool {
        self.x.contains(&pos.x) && self.y.contains(&pos.y)
    }
}
