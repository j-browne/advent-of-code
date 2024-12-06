use aoc_2024::guard_patrol::Map;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_06.txt")));
}

fn run(input: &str) -> usize {
    Map::new(input).num_loop_candidates()
}

mod test {
    #[test]
    fn aoc_2024_06_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_06_test.txt")), 6);
    }

    #[test]
    fn aoc_2024_06_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_06.txt")), 2162);
    }
}
