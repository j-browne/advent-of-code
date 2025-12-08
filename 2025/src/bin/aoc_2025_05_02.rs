use aoc_2025::cafeteria::Inventory;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_05.txt")));
}

fn run(input: &str) -> u64 {
    Inventory::new(input).range_count()
}

mod test {
    #[test]
    fn aoc_2025_05_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_05_test.txt")), 14);
    }

    #[test]
    fn aoc_2025_05_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_05.txt")),
            344771884978261
        );
    }
}
