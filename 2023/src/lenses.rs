pub struct Boxes {
    boxes: Vec<Vec<(String, usize)>>,
}

impl Boxes {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut boxes = vec![Vec::new(); 256];
        for x in s.trim().split(',') {
            let i = x.find(['-', '=']).unwrap();
            let (label, rest) = x.split_at(i);
            let idx = usize::try_from(hash(label)).unwrap();
            let b = &mut boxes[idx];
            match rest.chars().next().unwrap() {
                '-' => {
                    if let Some(idx) = b.iter().position(|(l, _p)| l == label) {
                        b.remove(idx);
                    }
                }
                '=' => {
                    let power = rest[1..].parse().unwrap();
                    if let Some(idx) = b.iter().position(|(l, _p)| l == label) {
                        b[idx].1 = power;
                    } else {
                        b.push((label.to_string(), power));
                    }
                }
                _ => unreachable!(),
            }
        }

        Self { boxes }
    }

    #[must_use]
    pub fn total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_num, b)| {
                b.iter()
                    .enumerate()
                    .map(move |(slot_num, (_label, power))| (box_num + 1) * (slot_num + 1) * power)
            })
            .sum()
    }
}

#[must_use]
pub fn hash(s: &str) -> u32 {
    let mut curr = 0;
    for c in s.bytes() {
        let c = u32::from(c);
        curr += c;
        curr *= 17;
        curr %= 256;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
