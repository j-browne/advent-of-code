use aoc_2025::junction_boxes::Circuits;

fn main() {
    println!("{}", run(include_str!("input/aoc_2025_08.txt"), 1000));
}

fn run(input: &str, num_to_connect: usize) -> usize {
    let mut circuits = Circuits::new(input);
    circuits.connect(num_to_connect);
    let mut sizes = circuits.circuit_sizes();
    sizes.sort_by_key(|x| std::cmp::Reverse(*x));
    sizes.into_iter().take(3).product()
}

mod test {
    #[test]
    fn aoc_2025_08_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_08_test.txt"), 10),
            40
        );
    }

    #[test]
    fn aoc_2025_08_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2025_08.txt"), 1000),
            84968
        );
    }
}
