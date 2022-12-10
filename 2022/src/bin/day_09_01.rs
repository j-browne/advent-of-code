use aoc_2022::rope::Movements;

fn main() {
    println!("{}", run(include_str!("input/day_09.txt")));
}

fn run(input: &str) -> usize {
    Movements::with_len(input, 2).tail_locations().len()
}

mod test {
    #[test]
    fn day_09_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_09_test_01.txt")), 13);
    }

    #[test]
    fn day_09_01() {
        assert_eq!(super::run(include_str!("input/day_09.txt")), 6209);
    }
}
