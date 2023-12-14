use aoc_2023::mirrors::Mirror;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_13.txt")));
}

fn run(input: &str) -> u32 {
    input.split("\n\n").map(|s| Mirror::new(s).score()).sum()
}

mod test {
    #[test]
    fn aoc_2023_13_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_13_test.txt")), 405);
    }

    #[test]
    fn aoc_2023_13_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_13.txt")), 27505);
    }
}
