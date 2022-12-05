use aoc_2022::crates::Stacks;

fn main() {
    println!("{}", run(include_str!("input/day_05.txt")));
}

fn run(input: &str) -> String {
    let mut stacks = Stacks::new(input);
    stacks.apply_moves_multiple();
    stacks.tops()
}

mod test {
    #[test]
    fn day_05_02_test() {
        assert_eq!(super::run(include_str!("input/day_05_test.txt")), "MCD");
    }

    #[test]
    fn day_05_02() {
        assert_eq!(super::run(include_str!("input/day_05.txt")), "TZLTLWRNF");
    }
}
