use aoc_2023::lagoon::Lagoon;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_18.txt")));
}

fn run(input: &str) -> i128 {
    Lagoon::from_swapped(input).internal_area()
}

mod test {
    #[test]
    fn aoc_2023_18_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_18_test.txt")),
            952408144115
        );
    }

    #[test]
    fn aoc_2023_18_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_18.txt")),
            133125706867777
        );
    }
}
