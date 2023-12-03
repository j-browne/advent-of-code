use aoc_2021::snailfish::Number;

fn main() {
    println!("{}", run(include_str!("input/day_18.txt")));
}

fn run(input: &str) -> u32 {
    input
        .trim()
        .split('\n')
        .map(Number::new)
        .sum::<Number>()
        .magnitude()
}

mod test {
    #[test]
    fn day_18_01_test() {
        assert_eq!(super::run(include_str!("input/day_18_test.txt")), 4140);
    }

    #[test]
    fn day_18_01() {
        assert_eq!(super::run(include_str!("input/day_18.txt")), 3935);
    }
}
