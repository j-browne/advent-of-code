use aoc_2022::filesystem::{Files, Ty};

fn main() {
    println!("{}", run(include_str!("input/day_07.txt")));
}

fn run(input: &'static str) -> u32 {
    Files::new(input)
        .files()
        .values()
        .filter(|(ty, size)| *ty == Ty::Dir && *size < 100_000)
        .map(|(_, x)| x)
        .sum::<u32>()
}

mod test {
    #[test]
    fn day_07_01_test() {
        assert_eq!(super::run(include_str!("input/day_07_test.txt")), 95437);
    }

    #[test]
    fn day_07_01() {
        assert_eq!(super::run(include_str!("input/day_07.txt")), 1297159);
    }
}
