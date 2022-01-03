use aoc_2021::dirac_dice::DiracGame;

fn main() {
    println!("{}", run(include_str!("input/day_21.txt")));
}

fn run(input: &str) -> u64 {
    let mut game = DiracGame::new(input);
    let mut wins = game.play();
    eprintln!("{:?}", wins);
    wins.sort_unstable();
    *wins.last().unwrap()
}

mod test {
    #[test]
    fn day_21_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_21_test.txt")),
            444356092776315
        );
    }

    #[test]
    fn day_21_02() {
        assert_eq!(super::run(include_str!("input/day_21.txt")), 57328067654557);
    }
}
