use aoc_2021::vents::Floor;

fn main() {
    println!("{}", run(include_str!("input/day_05.txt")));
}

fn run(input: &str) -> usize {
    let floor = Floor::new(input);
    floor.overlap()
}

mod test {
    #[test]
    fn day_05_02_test() {
        assert_eq!(super::run(include_str!("input/day_05_test.txt")), 12);
    }

    #[test]
    fn day_05_02() {
        assert_eq!(super::run(include_str!("input/day_05.txt")), 18605);
    }
}
