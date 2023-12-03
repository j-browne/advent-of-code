fn main() {
    println!("{}", run(include_str!("input/aoc_2023_01_test.txt")));
}

fn run(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum::<u32>()
}

mod test {
    #[test]
    fn day_01_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_01_test.txt")), 142);
    }

    #[test]
    fn day_01_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_01.txt")), 53080);
    }
}
