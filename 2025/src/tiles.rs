use std::{cmp::Ordering, iter, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(u64, u64);

impl Point {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut coords = s.split(',').map(|x| x.parse().unwrap());
        Self(coords.next().unwrap(), coords.next().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tiles {
    red_tiles: Vec<Point>,
}

impl Tiles {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let red_tiles = s.lines().map(Point::new).collect();
        Self { red_tiles }
    }

    #[must_use]
    pub fn largest_area_rect(&self) -> u64 {
        self.red_tiles
            .iter()
            .enumerate()
            .flat_map(|(idx_0, p_0)| {
                self.red_tiles.iter().skip(idx_0 + 1).map(|p_1| {
                    (u64::abs_diff(p_0.0, p_1.0) + 1) * (u64::abs_diff(p_0.1, p_1.1) + 1)
                })
            })
            .max()
            .unwrap()
    }

    #[must_use]
    pub fn largest_area_rect_red_green(&self) -> u64 {
        self.red_tiles
            .iter()
            .enumerate()
            .flat_map(|(idx_0, p_0)| {
                self.red_tiles
                    .iter()
                    .skip(idx_0 + 1)
                    .filter(|p_1| self.is_rect_valid(&Rect::new(*p_0, **p_1)))
                    .map(|p_1| {
                        (u64::abs_diff(p_0.0, p_1.0) + 1) * (u64::abs_diff(p_0.1, p_1.1) + 1)
                    })
            })
            .max()
            .unwrap()
    }

    fn is_rect_valid(&self, rect: &Rect) -> bool {
        let mut last_zone = None;
        for p in iter::chain(&self.red_tiles, iter::once(&self.red_tiles[0])) {
            if *p == rect.p_0 || *p == rect.p_1 {
                continue;
            }

            let new_zone = rect.zone(*p);
            if rect.p_0.0 == 2 && rect.p_0.1 == 1 && rect.p_1.0 == 11 && rect.p_1.1 == 3 {
                println!("{p:?}: {new_zone:?}");
            }
            if new_zone == Zone::Inner {
                return false;
            }

            #[allow(clippy::collapsible_if, reason = "rustfmt won't format it if inlined")]
            if let Some(last_zone) = last_zone {
                if !valid_zone_crossing(last_zone, new_zone) {
                    return false;
                }
            }
            last_zone = Some(new_zone);
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rect {
    p_0: Point,
    p_1: Point,
}

impl Rect {
    fn new(mut p_0: Point, mut p_1: Point) -> Self {
        if p_0.0 > p_1.0 {
            mem::swap(&mut p_0.0, &mut p_1.0);
        }
        if p_0.1 > p_1.1 {
            mem::swap(&mut p_0.1, &mut p_1.1);
        }

        Self { p_0, p_1 }
    }

    fn zone(&self, p: Point) -> Zone {
        match (
            p.0.cmp(&self.p_0.0),
            p.0.cmp(&self.p_1.0),
            p.1.cmp(&self.p_0.1),
            p.1.cmp(&self.p_1.1),
        ) {
            (
                Ordering::Greater,
                Ordering::Less,
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
            ) => Zone::N,
            (
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
            ) => Zone::NE,
            (
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater,
                Ordering::Less,
            ) => Zone::E,
            (
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
            ) => Zone::SE,
            (
                Ordering::Greater,
                Ordering::Less,
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
            ) => Zone::S,
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
                Ordering::Greater,
                Ordering::Greater | Ordering::Equal,
            ) => Zone::SW,
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
                Ordering::Greater,
                Ordering::Less,
            ) => Zone::W,
            (
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
                Ordering::Less | Ordering::Equal,
                Ordering::Less,
            ) => Zone::NW,
            (Ordering::Greater, Ordering::Less, Ordering::Greater, Ordering::Less) => Zone::Inner,
            (Ordering::Greater, Ordering::Less, Ordering::Equal, Ordering::Equal) => Zone::NS,
            (Ordering::Equal, Ordering::Equal, Ordering::Greater, Ordering::Less) => Zone::EW,
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => Zone::NESW,
            _ => unreachable!("this doesn't describe a valid zone"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum Zone {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Inner,
    NS,
    EW,
    NESW,
}

fn valid_zone_crossing(z_0: Zone, z_1: Zone) -> bool {
    #[allow(clippy::enum_glob_use)]
    use Zone::*;
    match (z_0, z_1) {
        (N, NE | NW)
        | (NE, E | SE | N | NW)
        | (E, SE | NE)
        | (SE, S | SW | E | NE)
        | (S, SW | SE)
        | (SW, W | NW | S | SE)
        | (W, NW | SW)
        | (NW, N | NE | W | SW) => true,
        (z_0, z_1) if z_0 == z_1 => true,
        _ => false,
    }
}
