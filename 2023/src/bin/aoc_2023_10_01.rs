use aoc_2023::pipe_maze::Maze;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_10.txt")));
}

fn run(input: &str) -> u32 {
    Maze::new(input).furthest_in_loop()
}

mod test {
    #[test]
    fn aoc_2023_10_01_test_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_10_01_test_01.txt")),
            4
        );
    }

    #[test]
    fn aoc_2023_10_01_test_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_10_01_test_02.txt")),
            4
        );
    }

    #[test]
    fn aoc_2023_10_01_test_03() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_10_01_test_03.txt")),
            8
        );
    }

    #[test]
    fn aoc_2023_10_01_test_04() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_10_01_test_04.txt")),
            8
        );
    }

    #[test]
    fn aoc_2023_10_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_10.txt")), 6856);
    }
}
