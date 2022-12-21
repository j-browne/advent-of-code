use aoc_2022::hill::Hill;

fn main() {
    println!("{}", run(include_str!("input/day_12.txt")));
}

fn run(input: &str) -> usize {
    Hill::new(input).shortest_path_dist()
}

mod test {
    #[test]
    fn day_12_01_test() {
        assert_eq!(super::run(include_str!("input/day_12_test.txt")), 31);
    }

    #[test]
    fn day_12_01() {
        assert_eq!(super::run(include_str!("input/day_12.txt")), 462);
    }
}
