use aoc_2023::mirror_images::Mirror;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_13.txt")));
}

fn run(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|s| Mirror::new(s).score_with_smudge())
        .sum()
}

mod test {
    #[test]
    fn aoc_2023_13_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_13_test.txt")), 400);
    }

    #[test]
    fn aoc_2023_13_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_13.txt")), 22906);
    }
}
