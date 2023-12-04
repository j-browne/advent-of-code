use std::collections::HashMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_01.txt")));
}

fn run(input: &str) -> u32 {
    let mut digit_map = HashMap::new();
    digit_map.insert("one", "1");
    digit_map.insert("two", "2");
    digit_map.insert("three", "3");
    digit_map.insert("four", "4");
    digit_map.insert("five", "5");
    digit_map.insert("six", "6");
    digit_map.insert("seven", "7");
    digit_map.insert("eight", "8");
    digit_map.insert("nine", "9");

    input
        .lines()
        .map(|old_s| {
            let mut s = String::from(old_s);
            for (name, digit) in &digit_map {
                if let Some(idx) = old_s.find(name) {
                    s.replace_range(idx..=idx, digit);
                }
                if let Some(idx) = old_s.rfind(name) {
                    s.replace_range(idx..=idx, digit);
                }
            }
            let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum::<u32>()
}

mod test {
    #[test]
    fn aoc_2023_01_02_test_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_01_test.txt")), 142);
    }

    #[test]
    fn aoc_2023_01_02_test_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_01_02_test.txt")),
            281
        );
    }

    #[test]
    fn aoc_2023_01_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_01.txt")), 53268);
    }
}
