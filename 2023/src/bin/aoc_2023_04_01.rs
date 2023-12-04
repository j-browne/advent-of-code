use aoc_2023::scratchcard::Scratchcard;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_04.txt")));
}

fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let wins = Scratchcard::new(line).wins();
            if wins == 0 {
                0
            } else {
                2usize.pow(u32::try_from(wins - 1).unwrap())
            }
        })
        .sum()
}

mod test {
    #[test]
    fn aoc_2023_04_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_04_test.txt")), 13);
    }

    #[test]
    fn aoc_2023_04_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_04.txt")), 19135);
    }
}
