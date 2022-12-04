use aoc_2022::cleaning::Assignments;

fn main() {
    println!("{}", run(include_str!("input/day_04.txt")));
}

fn run(input: &str) -> usize {
    Assignments::new(input).overlap_count()
}

mod test {
    #[test]
    fn day_04_02_test() {
        assert_eq!(super::run(include_str!("input/day_04_test.txt")), 4);
    }

    #[test]
    fn day_04_02() {
        assert_eq!(super::run(include_str!("input/day_04.txt")), 897);
    }
}
