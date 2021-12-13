use aoc_2021::dumbo_octopus::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_11.txt")));
}

fn run(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    grid.step_n(100);
    grid.flashes()
}

mod test {
    #[test]
    fn day_11_01_test() {
        assert_eq!(super::run(include_str!("input/day_11_test.txt")), 1656);
    }

    #[test]
    fn day_11_01() {
        assert_eq!(super::run(include_str!("input/day_11.txt")), 1688);
    }
}
