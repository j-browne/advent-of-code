use aoc_2023::races::Races;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_06.txt")));
}

fn run(input: &str) -> u64 {
    Races::without_spaces(input)
        .num_ways_to_win()
        .into_iter()
        .product()
}

mod test {
    #[test]
    fn aoc_2023_06_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_06_test.txt")),
            71503
        );
    }

    #[test]
    fn aoc_2023_06_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_06.txt")), 45128024);
    }
}
