use aoc_2021::chunk::Chunk;

fn main() {
    println!("{}", run(include_str!("input/day_10.txt")));
}

fn run(input: &str) -> u128 {
    let it = input.trim().split('\n').map(Chunk::new);

    it.filter_map(|x| x.status().err()).sum()
}

mod test {
    #[test]
    fn day_10_01_test() {
        assert_eq!(super::run(include_str!("input/day_10_test.txt")), 26397);
    }

    #[test]
    fn day_10_01() {
        assert_eq!(super::run(include_str!("input/day_10.txt")), 464991);
    }
}
