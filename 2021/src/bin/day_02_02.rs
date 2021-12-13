use aoc_2021::submarine::Direction;

fn main() {
    println!("{}", run(include_str!("input/day_02.txt")));
}

fn run(input: &str) -> i32 {
    let dirs = input
        .split('\n')
        .filter_map(|x| x.parse::<Direction>().ok());

    let (forward, down, _) = dirs.fold((0, 0, 0), |(f, d, a), x| match x {
        Direction::Forward(x) => (f + x, d + x * a, a),
        Direction::Up(x) => (f, d, a - x),
        Direction::Down(x) => (f, d, a + x),
    });

    forward * down
}

mod test {
    #[test]
    fn day_02_02_test() {
        assert_eq!(super::run(include_str!("input/day_02_test.txt")), 900);
    }

    #[test]
    fn day_02_02() {
        assert_eq!(super::run(include_str!("input/day_02.txt")), 1544000595);
    }
}
