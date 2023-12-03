use aoc_2021::diagnostic_report::DiagnosticReport;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(input: &str) -> u32 {
    let report = DiagnosticReport::new(input);

    report.oxygen_generator_rating() * report.co2_scrubber_rating()
}

mod test {
    #[test]
    fn day_03_02_test() {
        assert_eq!(super::run(include_str!("input/day_03_test.txt")), 230);
    }

    #[test]
    fn day_03_02() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 3379326);
    }
}
