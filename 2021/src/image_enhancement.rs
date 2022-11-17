use crate::vec2d::Vec2d;
use std::iter::once;

#[derive(Debug)]
pub struct Image {
    bg: bool,
    enhancement: Vec<bool>,
    pixels: Vec2d<bool>,
}

impl Image {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");
        let enhancement = it
            .next()
            .unwrap()
            .split('\n')
            .flat_map(|s| {
                s.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unknown pixel"),
                })
            })
            .collect();

        let it = it.next().unwrap().trim().split('\n');
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let data = it
            .flat_map(|s| {
                s.chars().map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unknown pixel"),
                })
            })
            .collect();
        let pixels = Vec2d::new(dim, data);

        Self {
            bg: false,
            enhancement,
            pixels,
        }
    }

    pub fn enhance(&mut self) {
        let old_bg = self.bg;
        self.bg = self.enhancement[bits_to_idx(&[old_bg; 9])];

        let mut old_pixels = {
            let dim = (self.pixels.dim().0 + 2, self.pixels.dim().1 + 2);
            let data = vec![self.bg; dim.0 * dim.1];
            Vec2d::new(dim, data)
        };
        std::mem::swap(&mut old_pixels, &mut self.pixels);

        for y in 0..(self.pixels.dim().1) {
            for x in 0..(self.pixels.dim().0) {
                let mut bits = [old_bg; 9];
                for (x_old, y_old) in old_pixels
                    .neighbor_diag_indices_isize(
                        isize::try_from(x).unwrap() - 1,
                        isize::try_from(y).unwrap() - 1,
                    )
                    .chain(
                        once((x - 1, y - 1))
                            .filter(|(x, y)| *x < old_pixels.dim().1 && *y < old_pixels.dim().0),
                    )
                {
                    let bits_idx = 3 * ((y_old + 1) - y + 1) + ((x_old + 1) - x + 1);
                    bits[bits_idx] = old_pixels[(x_old, y_old)];
                }
                let idx = bits_to_idx(&bits);

                self.pixels[(x, y)] = self.enhancement[idx];
            }
        }
    }

    pub fn enhance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.enhance();
        }
    }

    #[must_use]
    pub fn lit_pixels(&self) -> usize {
        self.pixels.iter().filter(|x| **x).count()
    }
}

fn bits_to_idx(bits: &[bool]) -> usize {
    bits.iter()
        .enumerate()
        .map(|(i, x)| usize::from(*x) << (8 - i))
        .sum()
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..(self.pixels.dim().1) {
            for x in 0..(self.pixels.dim().0) {
                if self.pixels[(x, y)] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
