use aoc_2021::trick_shot::Target;

fn main() {
    println!("{}", run(include_str!("input/day_17.txt")));
}

fn run(input: &str) -> i32 {
    let target = Target::new(input);
    target.max_height()
}

mod test {
    #[test]
    fn day_17_01_test() {
        assert_eq!(super::run(include_str!("input/day_17_test.txt")), 45);
    }

    #[test]
    fn day_17_01() {
        assert_eq!(super::run(include_str!("input/day_17.txt")), 5565);
    }
}
