use aoc_2024::disk_map::DiskMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_09.txt")));
}

fn run(input: &str) -> usize {
    let mut map = DiskMap::new(input);
    map.defrag();
    map.checksum()
}

mod test {
    #[test]
    fn aoc_2024_09_01_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_09_test.txt")), 1928);
    }

    #[test]
    fn aoc_2024_09_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2024_09.txt")),
            6386640365805
        );
    }
}
