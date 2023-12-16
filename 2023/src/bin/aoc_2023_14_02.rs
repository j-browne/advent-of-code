use aoc_2023::platform::Platform;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_14.txt")));
}

fn run(input: &str) -> u32 {
    Platform::new(input).load_after(1_000_000_000)
}

mod test {
    #[test]
    fn aoc_2023_14_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_14_test.txt")), 64);
    }

    #[test]
    fn aoc_2023_14_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_14.txt")), 103445);
    }
}
