use aoc_2023::lenses::Boxes;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_15.txt")));
}

fn run(input: &str) -> usize {
    Boxes::new(input).total_focusing_power()
}

mod test {
    #[test]
    fn aoc_2023_15_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_15_test.txt")), 145);
    }

    #[test]
    fn aoc_2023_15_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_15.txt")), 241094);
    }
}
