use aoc_2023::oasis::Oasis;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_09.txt")));
}

fn run(input: &str) -> i32 {
    input.lines().map(|l| Oasis::new(l).prev()).sum()
}

mod test {
    #[test]
    fn aoc_2023_09_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_09_test.txt")), 2);
    }

    #[test]
    fn aoc_2023_09_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_09.txt")), 803);
    }
}
