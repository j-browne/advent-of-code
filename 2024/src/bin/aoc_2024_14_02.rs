use aoc_2024::bathroom_robots::BathroomRobots;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_14.txt")));
}

fn run(input: &str) -> usize {
    let mut robots = BathroomRobots::new(input);
    robots.easter_egg()
}

mod test {
    #[test]
    fn aoc_2024_14_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_14.txt")), 6620);
    }
}
