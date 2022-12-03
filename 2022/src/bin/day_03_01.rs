use aoc_2022::rucksacks::Rucksacks;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(input: &str) -> u32 {
    Rucksacks::new(input).mixed_priority_sum()
}

mod test {
    #[test]
    fn day_03_01_test() {
        assert_eq!(super::run(include_str!("input/day_03_test.txt")), 157);
    }

    #[test]
    fn day_03_01() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 7848);
    }
}
