use aoc_2024::trails::Map;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_10.txt")));
}

fn run(input: &str) -> usize {
    Map::new(input).trailhead_scores().into_iter().sum()
}

mod test {
    #[test]
    fn aoc_2024_10_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_10_test.txt")), 36);
    }

    #[test]
    fn aoc_2024_10_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_10.txt")), 587);
    }
}
