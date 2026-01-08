use aoc_2025::junction_boxes::Circuits;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_08.txt")));
}

fn run(input: &str) -> u64 {
    let circuits = Circuits::new(input);
    let (box_1, box_2) = circuits.get_last_connection();
    u64::from(box_1.0) * u64::from(box_2.0)
}

mod test {
    #[test]
    fn aoc_2025_08_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_08_test.txt")),
            25272
        );
    }

    #[test]
    fn aoc_2025_08_02() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_08.txt")),
            8663467782
        );
    }
}
