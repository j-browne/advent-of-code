use aoc_2021::reactor::Reactor;

fn main() {
    println!("{}", run(include_str!("input/day_22.txt")));
}

fn run(input: &str) -> i64 {
    let reactor = Reactor::new(input);
    reactor.num_on_init()
}

mod test {
    #[test]
    fn day_22_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_22_01_test_01.txt")), 39);
    }

    #[test]
    fn day_22_01_test_02() {
        assert_eq!(
            super::run(include_str!("input/day_22_01_test_02.txt")),
            590784
        );
    }

    #[test]
    fn day_22_01_test_03() {
        assert_eq!(super::run(include_str!("input/day_22_02_test.txt")), 474140);
    }

    #[test]
    fn day_22_01() {
        assert_eq!(super::run(include_str!("input/day_22.txt")), 653798);
    }
}
