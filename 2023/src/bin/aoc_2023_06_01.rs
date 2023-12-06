use aoc_2023::races::Races;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_06.txt")));
}

fn run(input: &str) -> u64 {
    Races::with_spaces(input)
        .num_ways_to_win()
        .into_iter()
        .product()
}

mod test {
    #[test]
    fn aoc_2023_06_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_06_test.txt")), 288);
    }

    #[test]
    fn aoc_2023_06_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_06.txt")), 800280);
    }
}
