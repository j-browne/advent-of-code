use aoc_2024::print_queue::PrintQueue;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_05.txt")));
}

fn run(input: &str) -> u32 {
    PrintQueue::new(input)
        .invalid_updates_corrected()
        .map(|u| u.middle_page())
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_05_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_05_test.txt")), 123);
    }

    #[test]
    fn aoc_2024_05_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_05.txt")), 4713);
    }
}
