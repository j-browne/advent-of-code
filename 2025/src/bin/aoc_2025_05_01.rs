use aoc_2025::cafeteria::Inventory;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_05.txt")));
}

fn run(input: &str) -> usize {
    Inventory::new(input).num_fresh()
}

mod test {
    #[test]
    fn aoc_2025_05_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_05_test.txt")), 3);
    }

    #[test]
    fn aoc_2025_05_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_05.txt")), 821);
    }
}
