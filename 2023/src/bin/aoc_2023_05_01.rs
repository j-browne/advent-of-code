use aoc_2023::almanac::Almanac;
use std::ops::Range;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_05.txt")));
}

fn run(input: &str) -> u64 {
    Almanac::from_singles(input)
        .locations()
        .into_iter()
        .map(|Range { start, .. }| start)
        .min()
        .unwrap()
}

mod test {
    #[test]
    fn aoc_2023_05_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_05_test.txt")), 35);
    }

    #[test]
    fn aoc_2023_05_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_05.txt")), 836040384);
    }
}
