use aoc_2022::keep_away::KeepAway;

fn main() {
    println!("{}", run(include_str!("input/day_11.txt")));
}

fn run(input: &str) -> usize {
    let mut keep_away = KeepAway::new(input);
    keep_away.run(20, true);
    keep_away.monkey_business()
}

mod test {
    #[test]
    fn day_11_01_test() {
        assert_eq!(super::run(include_str!("input/day_11_test.txt")), 10605);
    }

    #[test]
    fn day_11_01() {
        assert_eq!(super::run(include_str!("input/day_11.txt")), 95472);
    }
}
