use aoc_2024::stones::Stones;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_11.txt")));
}

fn run(input: &str) -> usize {
    Stones::new(input).num_stones_after(25)
}

mod test {
    #[test]
    fn aoc_2024_11_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_11_test.txt")),
            55312
        );
    }

    #[test]
    fn aoc_2024_11_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_11.txt")), 175006);
    }
}
