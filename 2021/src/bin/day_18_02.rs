use aoc_2021::snailfish::Number;

fn main() {
    println!("{}", run(include_str!("input/day_18.txt")));
}

fn run(input: &str) -> u32 {
    let nums = input
        .trim()
        .split('\n')
        .map(Number::new)
        .collect::<Vec<_>>();

    nums.iter()
        .flat_map(|a| nums.iter().map(move |b| (a + b).magnitude()))
        .max()
        .unwrap()
}

mod test {
    #[test]
    fn day_18_02_test() {
        assert_eq!(super::run(include_str!("input/day_18_test.txt")), 3993);
    }

    #[test]
    fn day_18_02() {
        assert_eq!(super::run(include_str!("input/day_18.txt")), 4669);
    }
}
