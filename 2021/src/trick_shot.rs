use std::cmp::{max, max_by_key, min};

#[derive(Debug)]
pub struct Target {
    x: (i32, i32),
    y: (i32, i32),
}

impl Target {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.trim().split(": ").nth(1).unwrap().split(", ").map(|s| {
            let mut it = s.split('=');
            let dir = it.next().unwrap();
            let mut it = it.next().unwrap().split("..");
            let extents = (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            );
            (dir, extents)
        });

        let (dir, x) = it.next().unwrap();
        assert_eq!(dir, "x");
        let (dir, y) = it.next().unwrap();
        assert_eq!(dir, "y");

        Self { x, y }
    }

    #[must_use]
    pub fn max_height(&self) -> i32 {
        // Because of the symmetry of movement in the y-axis, the ball will pass through y=0 at an
        // integer time interval.
        // With that constraint, the furthest the ball can travel in one step without overshooting
        // is to/from the y of the box furthest from 0.
        let dy = max_by_key(self.y.0, self.y.1, |x| x.abs());
        if dy > 0 {
            (0..=dy).sum()
        } else {
            (0..dy.abs()).sum()
        }
    }

    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn num_velocities(&self) -> u32 {
        // The same reasoning as in `max_height` applies here.
        let vy_min = min(self.y.0, self.y.1);
        let vy_max = max(self.y.0.abs(), self.y.1.abs());
        let y_min = min(0, vy_min);
        let vx_min = min(min(self.x.0, self.x.1), 0);
        let vx_max = max(self.x.0, self.x.1);

        let mut num_valid = 0;
        for vy in vy_min..=vy_max {
            for vx in vx_min..=vx_max {
                let mut x = 0;
                let mut y = 0;
                let mut vx = vx;
                let mut vy = vy;
                loop {
                    y += vy;
                    x += vx;
                    vy -= 1;
                    // This moves vx closer to 0 by 1 (or keeps it at 0)
                    vx -= vx.signum();

                    if x >= self.x.0 && x <= self.x.1 && y >= self.y.0 && y <= self.y.1 {
                        num_valid += 1;
                        break;
                    }
                    if y < y_min {
                        break;
                    }
                }
            }
        }

        num_valid
    }
}
