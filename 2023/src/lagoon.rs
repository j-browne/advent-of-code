use crate::{array_2d::Indices, dir::Dir4};
use std::iter::zip;

pub struct Lagoon {
    border: Vec<Indices<i128>>,
}

impl Lagoon {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut border = Vec::new();
        let mut pos = Indices { col: 0, row: 0 };

        for l in s.lines() {
            let mut it = l.split_whitespace();
            let dir = match it.next().unwrap() {
                "U" => Dir4::Up,
                "R" => Dir4::Right,
                "D" => Dir4::Down,
                "L" => Dir4::Left,
                d => panic!("invalid direction: {d}"),
            };
            let dist = it.next().unwrap().parse::<i128>().unwrap();

            border.push(pos);
            pos = (pos + dir * dist).unwrap();
        }

        Self { border }
    }

    #[must_use]
    pub fn from_swapped(s: &str) -> Self {
        let mut border = Vec::new();
        let mut pos = Indices { col: 0, row: 0 };

        for l in s.lines() {
            let s = l
                .split_whitespace()
                .nth(2)
                .unwrap()
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            let (dist, dir) = s.split_at(5);
            let dist = i128::from_str_radix(dist, 16).unwrap();
            let dir = match dir {
                "3" => Dir4::Up,
                "0" => Dir4::Right,
                "1" => Dir4::Down,
                "2" => Dir4::Left,
                d => panic!("invalid direction: {d}"),
            };

            border.push(pos);
            pos = (pos + dir * dist).unwrap();
        }

        Self { border }
    }

    #[must_use]
    pub fn internal_area(&self) -> i128 {
        // This is the shoelace formula, but slightly altered.
        // The shoelace formula gives the area of the shape through the center of each block.
        // We need to add half of the border blocks.
        // For the corners we need to add or subtract a quarter of a block. However, all of the
        // positives and negatives cancel out, except for 4 positives (which sums up to 1).
        let mut area = 0;
        for (i1, i2) in zip(self.border.iter(), self.border.iter().cycle().skip(1)) {
            area += i1.col * i2.row - i1.row * i2.col;
            // these account for the extra from the borders
            area += (i1.col - i2.col).abs() + (i1.row - i2.row).abs();
        }
        area /= 2;
        // this accounts for the extra from the corners
        area.abs() + 1
    }
}
