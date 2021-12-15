use aoc_2021::polymer::Polymer;

fn main() {
    println!("{}", run(include_str!("input/day_14.txt")));
}

fn run(input: &str) -> u128 {
    let mut poly = Polymer::new(input);
    poly.insert_n(10);
    let counts = poly.counts();
    let max = counts.iter().map(|(_, x)| x).max().unwrap();
    let min = counts.iter().map(|(_, x)| x).min().unwrap();
    max - min
}

mod test {
    #[test]
    fn day_14_01_test() {
        assert_eq!(super::run(include_str!("input/day_14_test.txt")), 1588);
    }

    #[test]
    fn day_14_01() {
        assert_eq!(super::run(include_str!("input/day_14.txt")), 2194);
    }
}
