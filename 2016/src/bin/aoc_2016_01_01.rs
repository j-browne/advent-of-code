use aoc_2016::city_block::{parse_command, State};

fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(s: &str) -> i32 {
    let mut state = State::new();

    for c in s.trim().split(", ") {
        let (t, l) = parse_command(c);
        state.do_command(t, l);
    }

    state.distance()
}

mod test {
    #[test]
    fn day_01_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_01_01_test_01.txt")), 5);
    }

    #[test]
    fn day_01_01_test_02() {
        assert_eq!(super::run(include_str!("input/day_01_01_test_02.txt")), 2);
    }

    #[test]
    fn day_01_01_test_03() {
        assert_eq!(super::run(include_str!("input/day_01_01_test_03.txt")), 12);
    }

    #[test]
    fn day_01_01() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 278);
    }
}
