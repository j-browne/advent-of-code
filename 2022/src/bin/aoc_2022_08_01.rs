use aoc_2022::trees::Trees;

fn main() {
    println!("{}", run(include_str!("input/day_08.txt")));
}

fn run(input: &str) -> usize {
    Trees::new(input).visible_outside()
}

mod test {
    #[test]
    fn day_08_01_test() {
        assert_eq!(super::run(include_str!("input/day_08_test.txt")), 21);
    }

    #[test]
    fn day_08_01() {
        assert_eq!(super::run(include_str!("input/day_08.txt")), 1870);
    }
}
