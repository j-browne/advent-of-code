use aoc_2023::desert_map::DesertMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_08.txt")));
}

fn run(input: &str) -> u64 {
    DesertMap::new(input).steps()
}

mod test {
    #[test]
    fn aoc_2023_08_01_test_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_08_01_test_01.txt")),
            2
        );
    }

    #[test]
    fn aoc_2023_08_01_test_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_08_01_test_02.txt")),
            6
        );
    }

    #[test]
    fn aoc_2023_08_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_08.txt")), 19199);
    }
}
