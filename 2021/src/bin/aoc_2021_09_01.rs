use aoc_2021::geothermal_vents::Floor;

fn main() {
    println!("{}", run(include_str!("input/day_09.txt")));
}

fn run(input: &str) -> u32 {
    let floor = Floor::new(input);
    floor.low_points().map(|x| x + 1).sum()
}

mod test {
    #[test]
    fn day_09_01_test() {
        assert_eq!(super::run(include_str!("input/day_09_test.txt")), 15);
    }

    #[test]
    fn day_09_01() {
        assert_eq!(super::run(include_str!("input/day_09.txt")), 516);
    }
}
