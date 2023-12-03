use aoc_2023::cube_game::CubeGame;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_02.txt")));
}

fn run(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(": ").skip(1);
            let game = CubeGame::new(it.next().unwrap());
            game.power()
        })
        .sum::<u32>()
}

mod test {
    #[test]
    fn aoc_2023_02_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_02_test.txt")), 2286);
    }

    #[test]
    fn aoc_2023_02_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_02.txt")), 71535);
    }
}
