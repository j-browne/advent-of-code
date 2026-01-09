use aoc_2025::tiles::Tiles;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_09.txt")));
}

fn run(input: &str) -> u64 {
    Tiles::new(input).largest_area_rect_red_green()
}

mod test {
    #[test]
    fn aoc_2025_09_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_09_test.txt")), 24);
    }

    #[test]
    fn aoc_2025_09_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_09.txt")),
            1439894345
        );
    }
}
