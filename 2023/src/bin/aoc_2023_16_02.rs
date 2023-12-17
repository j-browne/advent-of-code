use aoc_2023::mirror_map::{Dir, MirrorMap};

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_16.txt")));
}

fn run(input: &str) -> usize {
    let map = MirrorMap::new(input);
    (0..map.dim().0)
        .map(|y| map.num_energized((0, y), Dir::Right))
        .chain((0..map.dim().0).map(|y| map.num_energized((map.dim().1 - 1, y), Dir::Left)))
        .chain((0..map.dim().1).map(|x| map.num_energized((x, 0), Dir::Down)))
        .chain((0..map.dim().1).map(|x| map.num_energized((x, map.dim().0 - 1), Dir::Up)))
        .max()
        .unwrap()
}

mod test {
    #[test]
    fn aoc_2023_16_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16_test.txt")), 51);
    }

    #[test]
    fn aoc_2023_16_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_16.txt")), 8061);
    }
}
