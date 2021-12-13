fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(input: &str) -> usize {
    let depths = input.split('\n').filter_map(|x| x.parse::<i32>().ok());
    let diffs = Iterator::zip(depths.clone(), depths.skip(1)).map(|(a, b)| b - a);

    diffs.filter(|x| *x > 0).count()
}

mod test {
    #[test]
    fn day_01_01_test() {
        assert_eq!(super::run(include_str!("input/day_01_test.txt")), 7);
    }

    #[test]
    fn day_01_01() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 1709);
    }
}
