use aoc_2022::datastream::Datastream;

fn main() {
    println!("{}", run(include_str!("input/day_06.txt")));
}

fn run(input: &str) -> usize {
    Datastream::new(input).start_of_packet_idx()
}

mod test {
    #[test]
    fn day_06_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_06_test_01.txt")), 7);
    }

    #[test]
    fn day_06_01_test_02() {
        assert_eq!(super::run(include_str!("input/day_06_test_02.txt")), 5);
    }

    #[test]
    fn day_06_01_test_03() {
        assert_eq!(super::run(include_str!("input/day_06_test_03.txt")), 6);
    }

    #[test]
    fn day_06_01_test_04() {
        assert_eq!(super::run(include_str!("input/day_06_test_04.txt")), 10);
    }

    #[test]
    fn day_06_01_test_05() {
        assert_eq!(super::run(include_str!("input/day_06_test_05.txt")), 11);
    }

    #[test]
    fn day_06_01() {
        assert_eq!(super::run(include_str!("input/day_06.txt")), 1850);
    }
}
