use std::{fmt::Display, ops::Not};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mirror {
    image: Vec<Vec<Tile>>,
}

impl Mirror {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let image = s
            .lines()
            .map(|l| l.as_bytes().iter().copied().map(Tile::new).collect())
            .collect();
        Self { image }
    }

    #[must_use]
    pub fn score(&self) -> u32 {
        match find_mirror(&self.image, None) {
            Some(MirrorPos::Row(row)) => 100 * (row + 1),
            Some(MirrorPos::Col(col)) => col + 1,
            None => panic!("no mirror detected:\n{self}"),
        }
    }

    #[must_use]
    pub fn score_with_smudge(&self) -> u32 {
        let orig_pos = find_mirror(&self.image, None);

        let mut fixed_image = self.image.clone();
        for y in 0..self.image.len() {
            for x in 0..self.image[y].len() {
                fixed_image[y][x] = !fixed_image[y][x];

                match find_mirror(&fixed_image, orig_pos) {
                    Some(MirrorPos::Row(row)) => return 100 * (row + 1),
                    Some(MirrorPos::Col(col)) => return col + 1,
                    _ => {}
                }

                // undo fix, so we can reuse the image
                fixed_image[y][x] = !fixed_image[y][x];
            }
        }
        panic!("no mirror detected:\n{self}")
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.image {
            for x in row {
                match x {
                    Tile::Ash => write!(f, "\u{25a1}")?,
                    Tile::Rock => write!(f, "\u{25a3}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn transpose(src: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    if src.is_empty() {
        return Vec::new();
    }

    let y_max = src.len();
    let x_max = src[0].len();
    assert!(src.iter().all(|row| row.len() == x_max));

    let mut dest = Vec::with_capacity(x_max);
    for x in 0..x_max {
        dest.push(Vec::with_capacity(y_max));
        for src_row in src {
            dest[x].push(src_row[x]);
        }
    }
    dest
}

fn find_mirror(image: &[Vec<Tile>], to_skip: Option<MirrorPos>) -> Option<MirrorPos> {
    if let Some(row) = find_mirror_row(image, to_skip.and_then(MirrorPos::as_row)) {
        Some(MirrorPos::Row(row))
    } else {
        find_mirror_row(&transpose(image), to_skip.and_then(MirrorPos::as_col)).map(MirrorPos::Col)
    }
}

fn find_mirror_row(image: &[Vec<Tile>], to_skip: Option<u32>) -> Option<u32> {
    let len = image.len();
    'outer: for start in 0..(len - 1) {
        if let Some(to_skip) = to_skip {
            if usize::try_from(to_skip).unwrap() == start {
                continue;
            }
        }

        'inner: for diff in 0.. {
            let row1 = start - diff;
            let row2 = start + diff + 1;

            if image[row1] != image[row2] {
                continue 'outer;
            }
            if row1 == 0 || row2 == len - 1 {
                break 'inner;
            }
        }
        // we only reach this if all row comparisons were equal
        return Some(u32::try_from(start).unwrap());
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn new(x: u8) -> Self {
        match x {
            b'.' => Self::Ash,
            b'#' => Self::Rock,
            _ => panic!("unknown tile: {x}"),
        }
    }
}

impl Not for Tile {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorPos {
    Row(u32),
    Col(u32),
}

impl MirrorPos {
    fn as_col(self) -> Option<u32> {
        if let Self::Col(col) = self {
            Some(col)
        } else {
            None
        }
    }

    fn as_row(self) -> Option<u32> {
        if let Self::Row(row) = self {
            Some(row)
        } else {
            None
        }
    }
}
