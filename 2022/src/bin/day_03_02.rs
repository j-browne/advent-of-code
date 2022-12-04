use aoc_2022::rucksacks::Rucksacks;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(input: &str) -> u32 {
    Rucksacks::new(input).badge_priority_sum()
}

mod test {
    #[test]
    fn day_03_02_test() {
        assert_eq!(super::run(include_str!("input/day_03_test.txt")), 70);
    }

    #[test]
    fn day_03_02() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 2616);
    }
}
