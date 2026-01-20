use aoc_2025::machines::Machine;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_10.txt")));
}

fn run(input: &str) -> u32 {
    input
        .lines()
        .map(|s| Machine::new(s).fewest_presses())
        .sum()
}

mod test {
    #[test]
    fn aoc_2025_10_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_10_test.txt")), 7);
    }

    #[test]
    fn aoc_2025_10_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_10.txt")), 512);
    }
}
