use aoc_2022::keep_away::KeepAway;

fn main() {
    println!("{}", run(include_str!("input/day_11.txt")));
}

fn run(input: &str) -> usize {
    let mut keep_away = KeepAway::new(input, false);
    keep_away.run(10000);
    keep_away.monkey_business()
}

mod test {
    #[test]
    fn day_11_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_11_test.txt")),
            2713310158
        );
    }

    #[test]
    fn day_11_02() {
        assert_eq!(super::run(include_str!("input/day_11.txt")), 17926061332);
    }
}
