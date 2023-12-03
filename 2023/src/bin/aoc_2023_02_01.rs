use aoc_2023::cube_game::CubeGame;
use std::collections::HashMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_02.txt")));
}

fn run(input: &str) -> u32 {
    let maxima = {
        let mut maxima = HashMap::new();
        maxima.insert("red".into(), 12);
        maxima.insert("green".into(), 13);
        maxima.insert("blue".into(), 14);
        maxima
    };

    input
        .lines()
        .filter_map(|line| {
            let mut it = line.split(": ");
            let game_num = it
                .next()
                .unwrap()
                .strip_prefix("Game ")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let game = CubeGame::new(it.next().unwrap());
            game.is_valid_for(&maxima).then_some(game_num)
        })
        .sum::<u32>()
}

mod test {
    #[test]
    fn aoc_2023_02_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_02_test.txt")), 8);
    }

    #[test]
    fn aoc_2023_02_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_02.txt")), 2720);
    }
}
