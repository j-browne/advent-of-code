use aoc_2023::{array_2d::Indices, dir::Dir4, mirror_map::MirrorMap};

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_16.txt")));
}

fn run(input: &str) -> usize {
    MirrorMap::new(input).num_energized(Indices { row: 0, col: 0 }, Dir4::Right)
}

mod test {
    #[test]
    fn aoc_2023_16_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16_test.txt")), 46);
    }

    #[test]
    fn aoc_2023_16_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16.txt")), 7728);
    }
}
