use aoc_2025::tiles::Tiles;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_09.txt")));
}

fn run(input: &str) -> u64 {
    Tiles::new(input).largest_area_rect()
}

mod test {
    #[test]
    fn aoc_2025_09_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_09_test.txt")), 50);
    }

    #[test]
    fn aoc_2025_09_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_09.txt")),
            4777967538
        );
    }
}
