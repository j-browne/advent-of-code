use aoc_2021::bingo::Game;

fn main() {
    println!("{}", run(include_str!("input/day_04.txt")));
}

fn run(input: &str) -> u32 {
    let mut game = Game::new(input);
    let (board, number) = game.first_winner();
    board.score(number)
}

mod test {
    #[test]
    fn day_04_01_test() {
        assert_eq!(super::run(include_str!("input/day_04_test.txt")), 4512);
    }

    #[test]
    fn day_04_01() {
        assert_eq!(super::run(include_str!("input/day_04.txt")), 54275);
    }
}
