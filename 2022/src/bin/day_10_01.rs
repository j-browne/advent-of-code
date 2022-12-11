use aoc_2022::crt::Crt;

fn main() {
    println!("{}", run(include_str!("input/day_10.txt")));
}

fn run(input: &str) -> i32 {
    let mut crt = Crt::new(input);
    crt.run();
    let x_values = crt.x_values();

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| i32::try_from(i).unwrap() * x_values[i])
        .sum()
}

mod test {
    #[test]
    fn day_10_01_test() {
        assert_eq!(super::run(include_str!("input/day_10_test.txt")), 13140);
    }

    #[test]
    fn day_10_01() {
        assert_eq!(super::run(include_str!("input/day_10.txt")), 15880);
    }
}
