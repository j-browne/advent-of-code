use aoc_2023::lagoon::Lagoon;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_18.txt")));
}

fn run(input: &str) -> i128 {
    Lagoon::new(input).internal_area()
}

mod test {
    #[test]
    fn aoc_2023_18_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_18_test.txt")), 62);
    }

    #[test]
    fn aoc_2023_18_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_18.txt")), 108909);
    }
}
