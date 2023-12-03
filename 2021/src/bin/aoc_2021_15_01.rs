use aoc_2021::chiton_cave::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_15.txt")));
}

fn run(input: &str) -> u32 {
    let grid = Grid::new(input);
    grid.min_path()
}

mod test {
    #[test]
    fn day_15_01_test() {
        assert_eq!(super::run(include_str!("input/day_15_test.txt")), 40);
    }

    #[test]
    fn day_15_01() {
        assert_eq!(super::run(include_str!("input/day_15.txt")), 447);
    }
}
