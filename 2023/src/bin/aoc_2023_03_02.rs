use aoc_2023::engine::Engine;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_03.txt")));
}

fn run(input: &str) -> u32 {
    Engine::new(input).gear_ratios().into_iter().sum()
}

mod test {
    #[test]
    fn aoc_2023_03_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_03_test.txt")),
            467835
        );
    }

    #[test]
    fn aoc_2023_03_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_03.txt")), 80694070);
    }
}
