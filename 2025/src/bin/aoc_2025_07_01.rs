use aoc_2025::tachyon_manifold::Manifold;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_07.txt")));
}

fn run(input: &str) -> u64 {
    Manifold::new(input).total_splits()
}

mod test {
    #[test]
    fn aoc_2025_07_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_07_test.txt")), 21);
    }

    #[test]
    fn aoc_2025_07_01() {
        assert_eq!(super::run(include_str!("input/aoc_2025_07.txt")), 1581);
    }
}
