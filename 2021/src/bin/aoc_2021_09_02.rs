use aoc_2021::geothermal_vents::{Basin, Floor};
use std::cmp::Reverse;

fn main() {
    println!("{}", run(include_str!("input/day_09.txt")));
}

fn run(input: &str) -> usize {
    let floor = Floor::new(input);
    let mut basins = floor.basins();
    basins.sort_unstable_by_key(|b| Reverse(b.size()));

    basins.iter().take(3).map(Basin::size).product()
}

mod test {
    #[test]
    fn day_09_02_test() {
        assert_eq!(super::run(include_str!("input/day_09_test.txt")), 1134);
    }

    #[test]
    fn day_09_02() {
        assert_eq!(super::run(include_str!("input/day_09.txt")), 1023660);
    }
}
