use aoc_2024::red_nosed_reports::RedNosedReport;

fn main() {
    println!("{}", run(include_str!("input/aoc_2024_02.txt")));
}

fn run(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let r = RedNosedReport::new(l);
            r.is_safe_with_dampener().then(|| r)
        })
        .count()
}

mod test {
    #[test]
    fn aoc_2024_02_02_test() {
        assert_eq!(super::run(include_str!("input/aoc_2024_02_test.txt")), 4);
    }

    #[test]
    fn aoc_2024_02_02() {
        assert_eq!(super::run(include_str!("input/aoc_2024_02.txt")), 710);
    }
}
