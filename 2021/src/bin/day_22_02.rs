use aoc_2021::reactor::Reactor;

fn main() {
    println!("{}", run(include_str!("input/day_22.txt")));
}

fn run(input: &str) -> i64 {
    let reactor = Reactor::new(input);
    reactor.num_on()
}

mod test {
    #[test]
    fn day_22_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_22_02_test.txt")),
            2758514936282235
        );
    }

    #[test]
    fn day_22_02() {
        assert_eq!(
            super::run(include_str!("input/day_22.txt")),
            1257350313518866
        );
    }
}
