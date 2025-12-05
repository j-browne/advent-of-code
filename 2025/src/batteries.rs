#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batteries {
    banks: Vec<Vec<u8>>,
}

impl Batteries {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let banks = s
            .lines()
            .map(|l| l.bytes().map(|c| c - b'0').collect())
            .collect();
        Self { banks }
    }

    #[must_use]
    pub fn bank_maxes(&self) -> Vec<u32> {
        self.banks.iter().map(bank_max).collect()
    }
}

#[must_use]
fn bank_max(bank: &Vec<u8>) -> u32 {
    let (idx, first) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        // rev() is necessary, so that we get the first max, not the last
        .rev()
        .max_by_key(|k| k.1)
        .unwrap();
    let second = bank[idx + 1..].iter().max().unwrap();

    u32::from(*first) * 10 + u32::from(*second)
}
