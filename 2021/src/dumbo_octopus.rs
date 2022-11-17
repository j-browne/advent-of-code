use crate::Vec2d;

#[derive(Debug)]
pub struct Grid {
    energies: Vec2d<u8>,
    flashes: u32,
}

impl Grid {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let data = s
            .split_whitespace()
            .flat_map(|l| {
                l.bytes().map(|c| {
                    assert!(c >= b'0');
                    assert!(c <= b'9');
                    c - b'0'
                })
            })
            .collect::<Vec<_>>();
        let energies = Vec2d::new((10, 10), data);

        Self {
            energies,
            flashes: 0,
        }
    }

    pub fn step(&mut self) {
        let dim = self.energies.dim();
        let mut to_visit = (0..(dim.0))
            .flat_map(move |y| (0..(dim.1)).map(move |x| (x, y)))
            .collect::<Vec<_>>();
        while !to_visit.is_empty() {
            let (x, y) = to_visit.pop().unwrap();
            self.energies[(x, y)] += 1;
            if self.energies[(x, y)] == 10 {
                self.flashes += 1;
                for (new_x, new_y) in self.energies.neighbor_diag_indices(x, y) {
                    to_visit.push((new_x, new_y));
                }
            }
        }
        for e in &mut self.energies {
            if *e > 9 {
                *e = 0;
            }
        }
    }

    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    #[must_use]
    pub const fn flashes(&self) -> u32 {
        self.flashes
    }
}
