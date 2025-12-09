use aoc_2025::cephalopod_math::MathProblems;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_06.txt")));
}

fn run(input: &str) -> u64 {
    MathProblems::new(input).grand_total()
}

mod test {
    #[test]
    fn aoc_2025_06_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_06_test.txt")),
            4277556
        );
    }

    #[test]
    fn aoc_2025_06_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_06.txt")),
            6209956042374
        );
    }
}
