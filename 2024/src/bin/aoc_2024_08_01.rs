use aoc_2024::antenna::Map;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_08.txt")));
}

fn run(input: &str) -> usize {
    Map::new(input).num_antinodes_one()
}

mod test {
    #[test]
    fn aoc_2024_08_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_08_test.txt")), 14);
    }

    #[test]
    fn aoc_2024_08_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_08.txt")), 400);
    }
}
