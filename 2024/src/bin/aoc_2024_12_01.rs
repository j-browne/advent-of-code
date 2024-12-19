use aoc_2024::garden::Garden;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_12.txt")));
}

fn run(input: &str) -> usize {
    Garden::new(input).fencing_cost()
}

mod test {
    #[test]
    fn aoc_2024_12_01_test_1() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_1.txt")),
            140
        );
    }

    #[test]
    fn aoc_2024_12_01_test_2() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_2.txt")),
            772
        );
    }

    #[test]
    fn aoc_2024_12_01_test_3() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_3.txt")),
            1930
        );
    }

    #[test]
    fn aoc_2024_12_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_12.txt")), 1461806);
    }
}
