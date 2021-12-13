use aoc_2016::keypad::State;

fn main() {
    println!("{}", run(include_str!("input/day_02.txt")));
}

fn run(s: &str) -> String {
    let mut state = State::new();
    let mut nums = String::new();

    for line in s.trim().split('\n') {
        for c in line.chars() {
            state.do_move_std(c.into());
        }
        nums.push_str(&state.loc.to_string());
    }
    nums
}

mod test {
    #[test]
    fn day_02_01_test() {
        assert_eq!(super::run(include_str!("input/day_02_test.txt")), "1985");
    }

    #[test]
    fn day_02_01() {
        assert_eq!(super::run(include_str!("input/day_02.txt")), "24862");
    }
}
