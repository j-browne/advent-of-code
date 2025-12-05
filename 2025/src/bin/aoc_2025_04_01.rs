use aoc_2025::forklift::RollMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_04.txt")));
}

fn run(input: &str) -> usize {
    RollMap::new(input).num_accessible()
}

mod test {
    #[test]
    fn aoc_2025_04_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_04_test.txt")), 13);
    }

    #[test]
    fn aoc_2025_04_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_04.txt")), 1474);
    }
}
