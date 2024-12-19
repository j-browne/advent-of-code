use aoc_2024::garden::Garden;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_12.txt")));
}

fn run(input: &str) -> usize {
    Garden::new(input).discounted_fencing_cost()
}

mod test {
    #[test]
    fn aoc_2024_12_02_test_1() {
        assert_eq!(super::run(include_str!("input/aoc_2024_12_test_1.txt")), 80);
    }

    #[test]
    fn aoc_2024_12_02_test_4() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_4.txt")),
            236
        );
    }

    #[test]
    fn aoc_2024_12_02_test_5() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_5.txt")),
            368
        );
    }

    #[test]
    fn aoc_2024_12_02_test_3() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_12_test_3.txt")),
            1206
        );
    }

    #[test]
    fn aoc_2024_12_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_12.txt")), 887932);
    }
}
