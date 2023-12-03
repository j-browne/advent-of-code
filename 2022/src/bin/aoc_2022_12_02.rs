use aoc_2022::hill::Hill;

fn main() {
    println!("{}", run(include_str!("input/day_12.txt")));
}

fn run(input: &str) -> usize {
    Hill::new(input).best_start_dist()
}

mod test {
    #[test]
    fn day_12_02_test() {
        assert_eq!(super::run(include_str!("input/day_12_test.txt")), 29);
    }

    #[test]
    fn day_12_02() {
        assert_eq!(super::run(include_str!("input/day_12.txt")), 451);
    }
}
