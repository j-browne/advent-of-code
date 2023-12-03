use aoc_2021::origami::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_13.txt")));
}

fn run(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.fold_once();
    grid.dots().len()
}

mod test {
    #[test]
    fn day_13_01_test() {
        assert_eq!(super::run(include_str!("input/day_13_test.txt")), 17);
    }

    #[test]
    fn day_13_01() {
        assert_eq!(super::run(include_str!("input/day_13.txt")), 763);
    }
}
