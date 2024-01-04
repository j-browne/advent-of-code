use aoc_2023::{dir::Dir4, platform::Platform};

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_14.txt")));
}

fn run(input: &str) -> usize {
    let mut p = Platform::new(input);
    p.tilt(Dir4::Up);
    p.load()
}

mod test {
    #[test]
    fn aoc_2023_14_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_14_test.txt")), 136);
    }

    #[test]
    fn aoc_2023_14_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_14.txt")), 108840);
    }
}
