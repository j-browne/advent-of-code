use aoc_2024::print_queue::{PrintQueue, Update};

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_05.txt")));
}

fn run(input: &str) -> u32 {
    PrintQueue::new(input)
        .valid_updates()
        .map(Update::middle_page)
        .sum()
}

mod test {
    #[test]
    fn aoc_2024_05_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_05_test.txt")), 143);
    }

    #[test]
    fn aoc_2024_05_01() {
        assert_eq!(super::run(include_str!("input/aoc_2024_05.txt")), 7307);
    }
}
