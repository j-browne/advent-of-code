use aoc_2021::sea_cucumber::Grid;

fn main() {
    println!("{}", run(include_str!("input/day_25.txt")));
}

fn run(input: &str) -> u32 {
    let _grid = Grid::new(input);
    todo!()
}

mod test {
    #[test]
    fn day_25_01_test() {
        assert_eq!(super::run(include_str!("input/day_25_test.txt")), 58);
    }

    #[test]
    fn day_25_01() {
        assert_eq!(super::run(include_str!("input/day_25.txt")), 0);
    }
}
