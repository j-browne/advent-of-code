use aoc_2023::galaxy::Galaxy;

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_11_02.txt")));
}

fn run(input: &str) -> usize {
    let mut it = input.split("\n\n");
    let expansion = it.next().unwrap().parse().unwrap();
    let mut galaxy = Galaxy::new(it.next().unwrap());
    galaxy.set_expansion(expansion);
    galaxy.sum_of_distances()
}

mod test {
    #[test]
    fn aoc_2023_11_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_11_01_test.txt")),
            374
        );
    }

    #[test]
    fn aoc_2023_11_01() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_11_01.txt")),
            9639160
        );
    }
}
