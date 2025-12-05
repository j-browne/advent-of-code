use aoc_2025::forklift::RollMap;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_04.txt")));
}

fn run(input: &str) -> usize {
    let mut sum = 0;
    let mut roll_map = RollMap::new(input);
    loop {
        let removed = roll_map.remove();
        if removed == 0 {
            break;
        }
        sum += removed;
    }
    sum
}

mod test {
    #[test]
    fn aoc_2025_04_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2025_04_test.txt")), 43);
    }

    #[test]
    fn aoc_2025_04_02() {
        assert_eq!(super::run(include_str!("input/aoc_2025_04.txt")), 8910);
    }
}
