use aoc_2023::lenses::hash;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_15.txt")));
}

fn run(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

mod test {
    #[test]
    fn aoc_2023_15_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_15_test.txt")), 1320);
    }

    #[test]
    fn aoc_2023_15_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_15.txt")), 501680);
    }
}
