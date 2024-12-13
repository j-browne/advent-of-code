use aoc_2024::disk_map::DiskMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_09.txt")));
}

fn run(input: &str) -> usize {
    let mut map = DiskMap::new(input);
    map.defrag_contiguous();
    map.checksum()
}

mod test {
    #[test]
    fn aoc_2024_09_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_09_test.txt")), 2858);
    }

    #[test]
    fn aoc_2024_09_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_09.txt")),
            6423258376982
        );
    }
}
