use aoc_2022::rosham::StrategyGuide;

fn main() {
    println!("{}", run(include_str!("input/day_02.txt")));
}

fn run(input: &str) -> u32 {
    StrategyGuide::with_res(input).score()
}

mod test {
    #[test]
    fn day_02_01_test() {
        assert_eq!(super::run(include_str!("input/day_02_test.txt")), 12);
    }

    #[test]
    fn day_02_01() {
        assert_eq!(super::run(include_str!("input/day_02.txt")), 11618);
    }
}
