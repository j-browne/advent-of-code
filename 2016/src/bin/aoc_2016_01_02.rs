use aoc_2016::city_block::{parse_command, State};
use std::collections::HashSet;

fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(s: &str) -> i32 {
    let mut state = State::new();
    let mut visited = HashSet::<(i32, i32)>::new();

    for c in s.trim().split(", ") {
        let (t, l) = parse_command(c);
        state.turn(t);
        for _ in 0..l {
            if !visited.insert(state.loc) {
                break;
            }
            state.forward(1);
        }
    }

    state.distance()
}

mod test {
    #[test]
    fn day_01_02_test() {
        assert_eq!(super::run(include_str!("input/day_01_02_test.txt")), 4);
    }

    #[test]
    fn day_01_02() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 161);
    }
}
