use aoc_2023::scratchcard::Scratchcard;
use std::collections::HashMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_04.txt")));
}

fn run(input: &str) -> usize {
    let mut total_cards = 0;
    let mut counts = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let wins = Scratchcard::new(line).wins();

        let curr = *counts.get(&i).unwrap_or(&1);
        for j in 1..=wins {
            *counts.entry(i + j).or_insert(1) += curr;
        }

        total_cards += curr;
    }
    total_cards
}

mod test {
    #[test]
    fn aoc_2023_04_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_04_test.txt")), 30);
    }

    #[test]
    fn aoc_2023_04_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_04.txt")), 5704953);
    }
}
