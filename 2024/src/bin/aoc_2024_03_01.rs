use aoc_2024::corrupted_computer::CorruptedComputer;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_03.txt")));
}

fn run(input: &str) -> u32 {
    CorruptedComputer::new(input).sum_without_disable()
}

mod test {
    #[test]
    fn aoc_2024_03_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_03_01_test.txt")),
            161
        );
    }

    #[test]
    fn aoc_2024_03_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_03.txt")), 165225049);
    }
}
