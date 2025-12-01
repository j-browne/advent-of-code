use aoc_2025::dial_lock::DialLock;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_01.txt")));
}

fn run(input: &str) -> i16 {
    DialLock::new(input).get_all_zeroes()
}

mod test {
    #[test]
    fn aoc_2025_01_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_01_test.txt")), 6);
    }

    #[test]
    fn aoc_2025_01_02() {
        assert_eq!(super::run(include_str!("input/aoc_2025_01.txt")), 5831);
    }
}
