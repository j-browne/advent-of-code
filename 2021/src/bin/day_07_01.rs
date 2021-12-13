use aoc_2021::least_fuel::least_fuel;
use std::str::FromStr;

fn main() {
    println!("{}", run(include_str!("input/day_07.txt")));
}

fn run(input: &str) -> i32 {
    let nums = input
        .trim()
        .split(',')
        .map(i32::from_str)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    least_fuel(nums, |x, i| (x - i).abs())
}

mod test {
    #[test]
    fn day_07_01_test() {
        assert_eq!(super::run(include_str!("input/day_07_test.txt")), 37);
    }

    #[test]
    fn day_07_01() {
        assert_eq!(super::run(include_str!("input/day_07.txt")), 336721);
    }
}
