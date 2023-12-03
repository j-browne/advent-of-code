use std::{cmp::Reverse, collections::BTreeSet};

fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| Reverse(s.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .take(3)
        .map(|x| x.0)
        .sum()
}

mod test {
    #[test]
    fn day_01_02_test() {
        assert_eq!(super::run(include_str!("input/day_01_test.txt")), 45000);
    }

    #[test]
    fn day_01_02() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 206780);
    }
}
