use aoc_2021::amphipod::Layout;

fn main() {
    println!("{}", run(include_str!("input/day_23.txt")));
}

fn run(input: &str) -> u32 {
    let _layout = Layout::new(input);
    todo!()
}

mod test {
    #[test]
    fn day_23_01_test() {
        assert_eq!(super::run(include_str!("input/day_23_test.txt")), 12521);
    }

    #[test]
    fn day_23_01() {
        assert_eq!(super::run(include_str!("input/day_23.txt")), 0);
    }
}
