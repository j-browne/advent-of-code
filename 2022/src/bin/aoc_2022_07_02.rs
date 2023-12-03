use aoc_2022::filesystem::{Files, Ty};

fn main() {
    println!("{}", run(include_str!("input/day_07.txt")));
}

fn run(input: &str) -> u32 {
    let files = Files::new(input);
    let files = files.files();
    let needed_space = dbg!(files.get(&vec!["/"]).unwrap().1) - 40_000_000;

    *files
        .values()
        .filter_map(|(ty, size)| (*ty == Ty::Dir && *size > needed_space).then_some(size))
        .min()
        .unwrap()
}

mod test {
    #[test]
    fn day_07_02_test() {
        assert_eq!(super::run(include_str!("input/day_07_test.txt")), 24933642);
    }

    #[test]
    fn day_07_02() {
        assert_eq!(super::run(include_str!("input/day_07.txt")), 3866390);
    }
}
