use aoc_2021::lanternfish::Population;

fn main() {
    println!("{}", run(include_str!("input/day_06.txt")));
}

fn run(input: &str) -> u128 {
    let mut pop = Population::new(input);
    pop.step_n(256);
    pop.total()
}

mod test {
    #[test]
    fn day_06_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_06_test.txt")),
            26984457539
        );
    }

    #[test]
    fn day_06_02() {
        assert_eq!(super::run(include_str!("input/day_06.txt")), 1695929023803);
    }
}
