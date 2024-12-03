use aoc_2024::corrupted_computer::CorruptedComputer;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_03.txt")));
}

fn run(input: &str) -> u32 {
    CorruptedComputer::new(input).sum()
}

mod test {
    #[test]
    fn aoc_2024_03_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_03_02_test.txt")),
            48
        );
    }

    #[test]
    fn aoc_2024_03_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_03.txt")), 108830766);
    }
}
