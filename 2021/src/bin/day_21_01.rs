use aoc_2021::dirac_dice::DeterministicGame;

fn main() {
    println!("{}", run(include_str!("input/day_21.txt")));
}

fn run(input: &str) -> u32 {
    let mut game = DeterministicGame::new(input);
    game.play();

    let mut scores = game.scores().clone();
    scores.sort_unstable();

    scores[scores.len() - 2] * game.num_rolls()
}

mod test {
    #[test]
    fn day_21_01_test() {
        assert_eq!(super::run(include_str!("input/day_21_test.txt")), 739785);
    }

    #[test]
    fn day_21_01() {
        assert_eq!(super::run(include_str!("input/day_21.txt")), 428736);
    }
}
