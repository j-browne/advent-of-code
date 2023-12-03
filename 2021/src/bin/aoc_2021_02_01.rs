use aoc_2021::submarine::Direction;

fn main() {
    println!("{}", run(include_str!("input/day_02.txt")));
}

fn run(input: &str) -> i32 {
    let dirs = input
        .split('\n')
        .filter_map(|x| x.parse::<Direction>().ok());

    let (forward, down) = dirs.fold((0, 0), |(f, d), x| match x {
        Direction::Forward(x) => (f + x, d),
        Direction::Up(x) => (f, d - x),
        Direction::Down(x) => (f, d + x),
    });

    forward * down
}

mod test {
    #[test]
    fn day_02_01_test() {
        assert_eq!(super::run(include_str!("input/day_02_test.txt")), 150);
    }

    #[test]
    fn day_02_01() {
        assert_eq!(super::run(include_str!("input/day_02.txt")), 1727835);
    }
}
