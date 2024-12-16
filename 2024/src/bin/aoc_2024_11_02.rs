use aoc_2024::stones::Stones;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_11.txt")));
}

fn run(input: &str) -> usize {
    Stones::new(input).num_stones_after(75)
}

mod test {
    #[test]
    fn aoc_2024_11_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_11.txt")),
            207961583799296
        );
    }
}
