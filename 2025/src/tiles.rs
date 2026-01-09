#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        (0..self.red_tiles.len())
            .flat_map(|i| {
                ((i + 1)..self.red_tiles.len()).map(move |j| {
                    let Point(x0, y0) = self.red_tiles[i];
                    let Point(x1, y1) = self.red_tiles[j];

                    (u64::abs_diff(x0, x1) + 1) * (u64::abs_diff(y0, y1) + 1)
                })
            })
            .max()
            .unwrap()
    }
}
