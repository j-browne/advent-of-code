use aoc_2023::almanac::Almanac;
use std::ops::Range;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_05.txt")));
}

fn run(input: &str) -> u64 {
    Almanac::from_ranges(input)
        .locations()
        .into_iter()
        .map(|Range { start, .. }| start)
        .min()
        .unwrap()
}

mod test {
    #[test]
    fn aoc_2023_05_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_05_test.txt")), 46);
    }

    #[test]
    fn aoc_2023_05_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_05.txt")), 10834440);
    }
}
