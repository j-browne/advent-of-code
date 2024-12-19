use aoc_2024::bathroom_robots::BathroomRobots;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_14.txt")));
}

fn run(input: &str) -> usize {
    BathroomRobots::new(input).safety_factor(100)
}

mod test {
    #[test]
    fn aoc_2024_14_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_14_test.txt")), 12);
    }

    #[test]
    fn aoc_2024_14_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_14.txt")), 233709840);
    }
}
