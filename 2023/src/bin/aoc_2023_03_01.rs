use aoc_2023::engine::Engine;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_03.txt")));
}

fn run(input: &str) -> u32 {
    Engine::new(input).part_numbers().into_iter().sum()
}

mod test {
    #[test]
    fn aoc_2023_03_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_03_test.txt")), 4361);
    }

    #[test]
    fn aoc_2023_03_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_03.txt")), 521601);
    }
}
