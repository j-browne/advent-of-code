use aoc_2022::crt::Crt;

fn main() {
    println!("{}", run(include_str!("input/day_10.txt")));
}

fn run(input: &str) -> String {
    let mut crt = Crt::new(input);
    crt.render()
}

mod test {
    #[test]
    fn day_10_02_test() {
        assert_eq!(
            super::run(include_str!("input/day_10_test.txt")),
            include_str!("output/day_10_02_test.txt")
        );
    }

    #[test]
    fn day_10_02() {
        assert_eq!(
            super::run(include_str!("input/day_10.txt")),
            include_str!("output/day_10_02.txt")
        );
    }
}
