use aoc_2023::desert_map::DesertMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_08.txt")));
}

fn run(input: &str) -> u64 {
    DesertMap::new(input).ghost_steps()
}

mod test {
    #[test]
    fn aoc_2023_08_02_test_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_08_02_test.txt")), 6);
    }

    #[test]
    fn aoc_2023_08_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_08.txt")),
            13663968099527
        );
    }
}
