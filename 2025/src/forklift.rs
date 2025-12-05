use crate::array_2d::Array2d;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollMap {
    roll_map: Array2d<usize, bool>,
}

impl RollMap {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let roll_map = Array2d::from_grid(s, |c| match c {
            b'.' => false,
            b'@' => true,
            c => panic!("unknown input: {c}"),
        });
        Self { roll_map }
    }

    pub fn remove(&mut self) -> usize {
        let mut removed = 0;

        let old_roll_map = self.roll_map.clone();
        for (idx, v) in self.roll_map.iter_mut().enumerate() {
            if *v
                && old_roll_map
                    .neighbors_8(old_roll_map.indices(idx).unwrap())
                    .filter(|v| **v)
                    .count()
                    < 4
            {
                *v = false;
                removed += 1;
            }
        }

        removed
    }
}

#[allow(dead_code, clippy::trivially_copy_pass_by_ref)]
fn print_tile(t: &bool) -> String {
    match t {
        false => String::from("."),
        true => String::from("@"),
    }
}
