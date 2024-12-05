use aoc_2024::word_search::WordSearch;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_04.txt")));
}

fn run(input: &str) -> usize {
    WordSearch::new(input).count_xmas()
}

mod test {
    #[test]
    fn aoc_2024_04_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_04_test.txt")), 18);
    }

    #[test]
    fn aoc_2024_04_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_04.txt")), 2575);
    }
}
