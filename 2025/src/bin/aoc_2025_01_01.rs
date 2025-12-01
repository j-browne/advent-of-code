use aoc_2025::dial_lock::DialLock;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_01.txt")));
}

fn run(input: &str) -> i16 {
    DialLock::new(input).get_end_zeroes()
}

mod test {
    #[test]
    fn aoc_2025_01_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_01_test.txt")), 3);
    }

    #[test]
    fn aoc_2025_01_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_01.txt")), 1031);
    }
}
