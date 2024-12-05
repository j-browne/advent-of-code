use aoc_2024::word_search::WordSearch;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_04.txt")));
}

fn run(input: &str) -> usize {
    WordSearch::new(input).count_crossed_mas()
}

mod test {
    #[test]
    fn aoc_2024_04_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_04_test.txt")), 9);
    }

    #[test]
    fn aoc_2024_04_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_04.txt")), 2041);
    }
}
