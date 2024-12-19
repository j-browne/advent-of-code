use aoc_2024::claw_game::ClawGame;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_13.txt")));
}

fn run(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(ClawGame::new)
        .filter_map(|c| c.cost_to_win())
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_13_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_13_test.txt")), 480);
    }

    #[test]
    fn aoc_2024_13_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_13.txt")), 32041);
    }
}
