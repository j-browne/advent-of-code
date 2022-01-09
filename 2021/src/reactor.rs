use std::cmp::{max, min};

pub struct Reactor {
    cubes: Vec<Cube>,
}

impl Reactor {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut cubes = Vec::new();
        let mut old_cubes = Vec::new();

        for line in s.trim().split('\n') {
            let mut it = line.split(' ');
            let state = match it.next().unwrap() {
                "on" => true,
                "off" => false,
                x => panic!("unknown state `{}`", x),
            };

            let mut it = it.next().unwrap().split(',');
            let x = parse_range(it.next().unwrap(), "x");
            let y = parse_range(it.next().unwrap(), "y");
            let z = parse_range(it.next().unwrap(), "z");

            let cube = Cube { lim: [x, y, z] };
            std::mem::swap(&mut cubes, &mut old_cubes);

            // whether turning cube on or off, add the difference of each
            // existing cube and the new cube (order matters!) to cubes.
            // If turning on, add cube when done.
            for c in old_cubes.drain(..) {
                cubes.add_diff(&c, &cube);
            }
            if state {
                cubes.push(cube);
            }
        }

        Self { cubes }
    }

    #[must_use]
    pub fn num_on_init(&self) -> i64 {
        let mut cubes = Vec::new();
        let lim = Cube {
            lim: [(-50, 50); 3],
        };
        for c in &self.cubes {
            cubes.add_union(c, &lim);
        }
        cubes.iter().map(Cube::size).sum()
    }

    #[must_use]
    pub fn num_on(&self) -> i64 {
        self.cubes.iter().map(Cube::size).sum()
    }
}

fn parse_range(s: &str, dim: &'static str) -> (i64, i64) {
    let mut it = s.split('=');
    assert_eq!(it.next().unwrap(), dim);
    let mut it = it.next().unwrap().split("..");
    (
        it.next().unwrap().parse::<i64>().unwrap(),
        it.next().unwrap().parse::<i64>().unwrap(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    lim: [(i64, i64); 3],
}

impl Cube {
    fn size(&self) -> i64 {
        (self.lim[0].1 - self.lim[0].0 + 1)
            * (self.lim[1].1 - self.lim[1].0 + 1)
            * (self.lim[2].1 - self.lim[2].0 + 1)
    }

    fn overlaps(&self, other: &Self) -> [Overlap; 3] {
        // TODO: use array instead of Vec?
        Iterator::zip(self.lim.iter(), other.lim.iter())
            .map(|(a, b)| {
                if (b.0 < a.0 && b.1 < a.0) || (b.0 > a.1 && b.1 > a.1) {
                    Overlap::None
                } else if b.0 <= a.0 && b.1 < a.1 {
                    Overlap::Low((a.0, b.1), (b.1 + 1, a.1))
                } else if b.0 > a.0 && b.1 < a.1 {
                    Overlap::Mid((a.0, b.0 - 1), (b.0, b.1), (b.1 + 1, a.1))
                } else if b.0 > a.0 && b.1 >= a.1 {
                    Overlap::High((a.0, b.0 - 1), (b.0, a.1))
                } else if b.0 <= a.0 && b.1 >= a.1 {
                    Overlap::All((a.0, a.1))
                } else {
                    unreachable!()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
}

trait CubeOps {
    /// Find the set difference of a and b, and add it to self.
    /// This may involve splitting a into multiple cubes.
    fn add_diff(&mut self, a: &Cube, b: &Cube);

    fn add_diff_inner(&mut self, lim: [(i64, i64); 3], overlaps: [Overlap; 3], depth: usize);

    /// Find set union of a and b, and add it to self.
    fn add_union(&mut self, a: &Cube, b: &Cube);
}

impl CubeOps for Vec<Cube> {
    fn add_diff(&mut self, a: &Cube, b: &Cube) {
        let overlaps = a.overlaps(b);

        // if any of the dimensions don't overlap, the difference is a
        if a.overlaps(b).iter().any(|x| *x == Overlap::None) {
            self.push(a.clone());
            return;
        }

        self.add_diff_inner(a.lim, overlaps, 0);
    }

    fn add_diff_inner(&mut self, mut lim: [(i64, i64); 3], overlaps: [Overlap; 3], depth: usize) {
        use Overlap::{All, High, Low, Mid, None};

        match overlaps[depth] {
            Low(a, b) => {
                lim[depth] = a;
                if depth != 2 {
                    self.add_diff_inner(lim, overlaps, depth + 1);
                }

                lim[depth] = b;
                self.push(Cube { lim });
            }
            Mid(a, b, c) => {
                lim[depth] = a;
                self.push(Cube { lim });

                lim[depth] = b;
                if depth != 2 {
                    self.add_diff_inner(lim, overlaps, depth + 1);
                }

                lim[depth] = c;
                self.push(Cube { lim });
            }
            High(a, b) => {
                lim[depth] = a;
                self.push(Cube { lim });

                lim[depth] = b;
                if depth != 2 {
                    self.add_diff_inner(lim, overlaps, depth + 1);
                }
            }
            All(a) => {
                lim[depth] = a;
                if depth != 2 {
                    self.add_diff_inner(lim, overlaps, depth + 1);
                }
            }
            None => unreachable!(),
        }
    }

    fn add_union(&mut self, a: &Cube, b: &Cube) {
        // if any of the dimensions don't overlap, the union is empty
        // otherwise, there is one overlapping Cube
        if !a.overlaps(b).iter().any(|x| *x == Overlap::None) {
            self.push(Cube {
                lim: [
                    (max(a.lim[0].0, b.lim[0].0), min(a.lim[0].1, b.lim[0].1)),
                    (max(a.lim[1].0, b.lim[1].0), min(a.lim[1].1, b.lim[1].1)),
                    (max(a.lim[2].0, b.lim[2].0), min(a.lim[2].1, b.lim[2].1)),
                ],
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Overlap {
    None,
    Low((i64, i64), (i64, i64)),
    Mid((i64, i64), (i64, i64), (i64, i64)),
    High((i64, i64), (i64, i64)),
    All((i64, i64)),
}
