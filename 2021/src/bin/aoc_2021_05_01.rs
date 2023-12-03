use aoc_2021::vents::Floor;

fn main() {
    println!("{}", run(include_str!("input/day_05.txt")));
}

fn run(input: &str) -> usize {
    let floor = Floor::new(input);
    floor.overlap_hv()
}

mod test {
    #[test]
    fn day_05_01_test() {
        assert_eq!(super::run(include_str!("input/day_05_test.txt")), 5);
    }

    #[test]
    fn day_05_01() {
        assert_eq!(super::run(include_str!("input/day_05.txt")), 5197);
    }
}
