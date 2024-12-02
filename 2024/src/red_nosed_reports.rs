#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNosedReport {
    levels: Vec<i32>,
}

impl RedNosedReport {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let levels = s
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Self { levels }
    }

    #[must_use]
    pub fn is_safe(&self) -> bool {
        enum Dir {
            Pos,
            Neg,
        }

        let mut dir = None;
        for (n1, n2) in self.levels.iter().zip(self.levels.iter().skip(1)) {
            if dir.is_none() {
                if n2 > n1 {
                    dir = Some(Dir::Pos);
                } else {
                    dir = Some(Dir::Neg);
                }
            }

            match dir {
                None => unreachable!("checked above"),
                Some(Dir::Pos) => {
                    if n2 - n1 < 1 || n2 - n1 > 3 {
                        return false;
                    }
                }
                Some(Dir::Neg) => {
                    if n1 - n2 < 1 || n1 - n2 > 3 {
                        return false;
                    }
                }
            }
        }

        true
    }

    #[must_use]
    pub fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        (0..self.levels.len())
            .map(|i| {
                let mut levels = self.levels.clone();
                levels.remove(i);
                Self { levels }
            })
            .any(|r| r.is_safe())
    }
}
