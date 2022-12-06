use aoc_2022::datastream::Datastream;

fn main() {
    println!("{}", run(include_str!("input/day_06.txt")));
}

fn run(input: &str) -> usize {
    Datastream::new(input).start_of_message_idx()
}

mod test {
    #[test]
    fn day_06_02_test_01() {
        assert_eq!(super::run(include_str!("input/day_06_test_01.txt")), 19);
    }

    #[test]
    fn day_06_02_test_02() {
        assert_eq!(super::run(include_str!("input/day_06_test_02.txt")), 23);
    }

    #[test]
    fn day_06_02_test_03() {
        assert_eq!(super::run(include_str!("input/day_06_test_03.txt")), 23);
    }

    #[test]
    fn day_06_02_test_04() {
        assert_eq!(super::run(include_str!("input/day_06_test_04.txt")), 29);
    }

    #[test]
    fn day_06_02_test_05() {
        assert_eq!(super::run(include_str!("input/day_06_test_05.txt")), 26);
    }

    #[test]
    fn day_06_02() {
        assert_eq!(super::run(include_str!("input/day_06.txt")), 2823);
    }
}
