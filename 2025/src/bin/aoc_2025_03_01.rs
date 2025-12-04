use aoc_2025::batteries::Batteries;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_03.txt")));
}

fn run(input: &str) -> u64 {
    let batteries = Batteries::new(input);
    batteries.sum_max_joltage(2)
}

mod test {
    #[test]
    fn aoc_2025_03_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_03_test.txt")), 357);
    }

    #[test]
    fn aoc_2025_03_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_03.txt")), 17113);
    }
}
