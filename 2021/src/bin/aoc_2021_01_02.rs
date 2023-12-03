fn main() {
    println!("{}", run(include_str!("input/day_01.txt")));
}

fn run(input: &str) -> usize {
    let depths = input.split('\n').filter_map(|x| x.parse::<i32>().ok());
    let sums = Iterator::zip(
        depths.clone(),
        Iterator::zip(depths.clone().skip(1), depths.skip(2)),
    )
    .map(|(a, (b, c))| a + b + c);
    let diffs = Iterator::zip(sums.clone(), sums.skip(1)).map(|(a, b)| b - a);

    diffs.filter(|x| *x > 0).count()
}

mod test {
    #[test]
    fn day_01_02_test() {
        assert_eq!(super::run(include_str!("input/day_01_test.txt")), 5);
    }

    #[test]
    fn day_01_02() {
        assert_eq!(super::run(include_str!("input/day_01.txt")), 1761);
    }
}
