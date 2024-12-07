use aoc_2024::bridge_repair::Calibration;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_07.txt")));
}

fn run(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|s| {
            let cal = Calibration::new(s, vec![u64::checked_add, u64::checked_mul]);
            cal.is_valid().then(|| cal.output())
        })
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_07_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_07_test.txt")), 3749);
    }

    #[test]
    fn aoc_2024_07_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_07.txt")),
            6392012777720
        );
    }
}
