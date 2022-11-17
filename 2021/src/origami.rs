use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

#[derive(Debug)]
pub struct Grid {
    extents: (i32, i32),
    dots: HashSet<(i32, i32)>,
    folds: VecDeque<Fold>,
}

impl Grid {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.trim().split("\n\n");

        let dots = it
            .next()
            .unwrap()
            .split('\n')
            .map(|s| {
                let mut it = s.split(',');
                let a = it.next().unwrap().parse().unwrap();
                let b = it.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect::<HashSet<_, _>>();

        let folds = it.next().unwrap().split('\n').map(Fold::new).collect();

        let extents = dots.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (max(max_x, *x), max(max_y, *y))
        });

        Self {
            extents,
            dots,
            folds,
        }
    }

    pub fn fold(&mut self) {
        while !self.folds.is_empty() {
            self.fold_once();
        }
    }

    pub fn fold_once(&mut self) {
        let fold = self.folds.pop_front().unwrap();

        let mut old_dots = HashSet::with_capacity(self.dots.len());
        std::mem::swap(&mut self.dots, &mut old_dots);
        for (x, y) in old_dots {
            self.dots.insert(match fold {
                Fold::X(f) => {
                    self.extents.0 = f;
                    (if x > f { f - (x - f) } else { x }, y)
                }
                Fold::Y(f) => {
                    self.extents.1 = f;
                    (x, if y > f { f - (y - f) } else { y })
                }
            });
        }
    }

    #[must_use]
    pub const fn dots(&self) -> &HashSet<(i32, i32)> {
        &self.dots
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..(self.extents.1) {
            for x in 0..(self.extents.0) {
                if self.dots.contains(&(x, y)) {
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

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next().unwrap(), "fold");
        assert_eq!(it.next().unwrap(), "along");

        let mut it = it.next().unwrap().split('=');
        let dir = it.next().unwrap();
        let amt = it.next().unwrap().parse().unwrap();
        match dir {
            "x" => Self::X(amt),
            "y" => Self::Y(amt),
            _ => panic!("unknown fold direction `{dir}`"),
        }
    }
}
