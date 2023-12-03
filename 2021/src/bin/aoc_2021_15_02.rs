use aoc_2021::chiton_cave::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_15.txt")));
}

fn run(input: &str) -> u32 {
    let grid = Grid::new_expanded(input);
    grid.min_path()
}

mod test {
    #[test]
    fn day_15_02_test() {
        assert_eq!(super::run(include_str!("input/day_15_test.txt")), 315);
    }

    #[test]
    fn day_15_02() {
        assert_eq!(super::run(include_str!("input/day_15.txt")), 2825);
    }
}
