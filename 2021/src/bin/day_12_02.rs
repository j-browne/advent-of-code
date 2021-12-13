use aoc_2021::cave::System;

fn main() {
    println!("{}", run(include_str!("input/day_12.txt")));
}

fn run(input: &str) -> usize {
    let system = System::new(input);
    system.paths_with_doubling().len()
}

mod test {
    #[test]
    fn day_12_02_test_01() {
        assert_eq!(super::run(include_str!("input/day_12_test_01.txt")), 36);
    }

    #[test]
    fn day_12_02_test_02() {
        assert_eq!(super::run(include_str!("input/day_12_test_02.txt")), 103);
    }

    #[test]
    fn day_12_02_test_03() {
        assert_eq!(super::run(include_str!("input/day_12_test_03.txt")), 3509);
    }

    #[test]
    fn day_12_02() {
        assert_eq!(super::run(include_str!("input/day_12.txt")), 107395);
    }
}
