use aoc_2022::trees::Trees;

fn main() {
    println!("{}", run(include_str!("input/day_08.txt")));
}

fn run(input: &str) -> usize {
    Trees::new(input).max_scenic_score()
}

mod test {
    #[test]
    fn day_08_02_test() {
        assert_eq!(super::run(include_str!("input/day_08_test.txt")), 8);
    }

    #[test]
    fn day_08_02() {
        assert_eq!(super::run(include_str!("input/day_08.txt")), 517440);
    }
}
