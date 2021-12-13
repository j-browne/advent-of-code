use aoc_2021::chunk::Chunk;

fn main() {
    println!("{}", run(include_str!("input/day_10.txt")));
}

fn run(input: &str) -> u128 {
    let mut scores = input
        .trim()
        .split('\n')
        .map(Chunk::new)
        .filter_map(|x| x.status().ok())
        .collect::<Vec<_>>();
    scores.sort_unstable();

    let len = scores.len();
    scores[len / 2]
}

mod test {
    #[test]
    fn day_10_02_test() {
        assert_eq!(super::run(include_str!("input/day_10_test.txt")), 288957);
    }

    #[test]
    fn day_10_02() {
        assert_eq!(super::run(include_str!("input/day_10.txt")), 3662008566);
    }
}
