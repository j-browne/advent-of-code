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

    #[must_use]
    pub fn num_accessible(&self) -> usize {
        self.roll_map
            .iter()
            .enumerate()
            .filter(|(idx, v)| {
                **v && self
                    .roll_map
                    .neighbors_8(self.roll_map.indices(*idx).unwrap())
                    .filter(|v| **v)
                    .count()
                    < 4
            })
            .count()
    }
}

#[allow(dead_code, clippy::trivially_copy_pass_by_ref)]
fn print_tile(t: &bool) -> String {
    match t {
        false => String::from("."),
        true => String::from("@"),
    }
}
