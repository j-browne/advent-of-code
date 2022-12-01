fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

mod test {
    #[test]
    fn day_01_01_test() {
        assert_eq!(super::run(include_str!("input/day_01_test.txt")), 24000);
    }

    #[test]
    fn day_01_01() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 69626);
    }
}
