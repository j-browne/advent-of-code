use aoc_2023::camel_cards::Hand;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_07.txt")));
}

fn run(input: &str) -> u32 {
    let mut hand_bets = input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            (
                Hand::with_jokers(it.next().unwrap()),
                it.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(Hand, u32)>>();
    hand_bets.sort();
    hand_bets
        .into_iter()
        .enumerate()
        .map(|(rank, (_hand, bet))| bet * (u32::try_from(rank).unwrap() + 1))
        .sum()
}

mod test {
    #[test]
    fn aoc_2023_07_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_07_test.txt")), 5905);
    }

    #[test]
    fn aoc_2023_07_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_07.txt")), 247885995);
    }
}
