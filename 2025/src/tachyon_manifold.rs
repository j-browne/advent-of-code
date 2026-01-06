use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifold {
    start: usize,
    splitters: Vec<Vec<usize>>,
}

impl Manifold {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut start = 0;
        let splitters = s
            .lines()
            .map(|l| {
                l.bytes()
                    .enumerate()
                    .filter_map(|(idx, byte)| match byte {
                        b'^' => Some(idx),
                        b'S' => {
                            start = idx;
                            None
                        }
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Self { start, splitters }
    }

    #[must_use]
    pub fn total_splits(&self) -> u64 {
        let mut splits = 0;

        let mut beams = HashSet::new();
        beams.insert(self.start);

        for row in &self.splitters {
            for splitter in row {
                if beams.remove(splitter) {
                    beams.insert(splitter - 1);
                    beams.insert(splitter + 1);
                    splits += 1;
                }
            }
        }

        splits
    }

    #[must_use]
    pub fn total_timelines(&self) -> u64 {
        let mut beams = HashMap::new();
        beams.insert(self.start, 1);

        for row in &self.splitters {
            for splitter in row {
                if let Some(count) = beams.remove(splitter) {
                    *beams.entry(splitter - 1).or_default() += count;
                    *beams.entry(splitter + 1).or_default() += count;
                }
            }
        }

        beams.into_iter().fold(0, |acc, (_pos, count)| acc + count)
    }
}
