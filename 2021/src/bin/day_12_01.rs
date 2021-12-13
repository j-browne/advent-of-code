use aoc_2021::cave::System;

fn main() {
    println!("{}", run(include_str!("input/day_12.txt")));
}

fn run(input: &str) -> usize {
    let system = System::new(input);
    system.paths().len()
}

mod test {
    #[test]
    fn day_12_01_test_01() {
        assert_eq!(super::run(include_str!("input/day_12_test_01.txt")), 10);
    }

    #[test]
    fn day_12_01_test_02() {
        assert_eq!(super::run(include_str!("input/day_12_test_02.txt")), 19);
    }

    #[test]
    fn day_12_01_test_03() {
        assert_eq!(super::run(include_str!("input/day_12_test_03.txt")), 226);
    }

    #[test]
    fn day_12_01() {
        assert_eq!(super::run(include_str!("input/day_12.txt")), 3679);
    }
}
