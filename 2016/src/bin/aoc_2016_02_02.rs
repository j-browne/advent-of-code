use aoc_2016::keypad::State;

fn main() {
    println!("{}", run(include_str!("input/day_02.txt")));
}

fn run(s: &str) -> String {
    let mut state = State::new();
    let mut nums = String::new();

    for line in s.trim().split('\n') {
        for c in line.chars() {
            state.do_move(c.into());
        }
        nums.push_str(&format!("{:X}", state.loc));
    }
    nums
}

mod test {
    #[test]
    fn day_02_02_test() {
        assert_eq!(super::run(include_str!("input/day_02_test.txt")), "5DB3");
    }

    #[test]
    fn day_02_02() {
        assert_eq!(super::run(include_str!("input/day_02.txt")), "46C91");
    }
}
