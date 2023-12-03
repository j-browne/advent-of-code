use aoc_2021::bingo::Game;

fn main() {
    println!("{}", run(include_str!("input/day_04.txt")));
}

fn run(input: &str) -> u32 {
    let mut game = Game::new(input);
    let (board, number) = game.last_winner();
    board.score(number)
}

mod test {
    #[test]
    fn day_04_02_test() {
        assert_eq!(super::run(include_str!("input/day_04_test.txt")), 1924);
    }

    #[test]
    fn day_04_02() {
        assert_eq!(super::run(include_str!("input/day_04.txt")), 13158);
    }
}
