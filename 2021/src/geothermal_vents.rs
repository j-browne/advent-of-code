use crate::Vec2d;

#[derive(Debug)]
pub struct Floor {
    heights: Vec2d<u32>,
}

impl Floor {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let it = s.split('\n').filter(|x| !x.is_empty());
        let rows = it.clone().count();
        let cols = it.clone().next().unwrap().len();
        let dim = (rows, cols);
        let data = it
            .flat_map(|r| {
                r.chars().map(|c| {
                    let c = c as u32;
                    assert!(c >= 48);
                    assert!(c <= 57);
                    c - 48
                })
            })
            .collect();
        let heights = Vec2d::new(dim, data);

        Self { heights }
    }
    #[must_use]
    pub fn height_at(&self, x: usize, y: usize) -> u32 {
        self.heights[(x, y)]
    }

    pub fn low_points(&self) -> impl Iterator<Item = u32> + '_ {
        self.low_point_indices().map(|(x, y)| self.height_at(x, y))
    }

    pub fn low_point_indices(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let dim = self.heights.dim();
        (0..(dim.0))
            .flat_map(move |y| (0..(dim.1)).map(move |x| (x, y)))
            .filter(|(x, y)| self.is_low_point((*x, *y)))
    }

    fn is_low_point(&self, (x, y): (usize, usize)) -> bool {
        let height = self.height_at(x, y);

        self.heights.neighbors(x, y).all(|h| *h > height)
    }

    #[must_use]
    pub fn basins(&self) -> Vec<Basin> {
        let mut basins = Vec::new();

        for lp in self.low_point_indices() {
            let mut points = Vec::new();
            let mut to_visit = vec![lp];

            while !to_visit.is_empty() {
                let curr = to_visit.pop().unwrap();
                for next in self.heights.neighbor_indices(curr.0, curr.1) {
                    if !points.contains(&next)
                        && !to_visit.contains(&next)
                        && self.height_at(next.0, next.1) != 9
                    {
                        to_visit.push(next);
                    }
                }
                points.push(curr);
            }

            basins.push(Basin { points });
        }

        basins
    }
}

pub struct Basin {
    points: Vec<(usize, usize)>,
}

impl Basin {
    #[must_use]
    pub fn size(&self) -> usize {
        self.points.len()
    }
}
