use aoc_2021::diagnostic_report::DiagnosticReport;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(input: &str) -> u32 {
    let report = DiagnosticReport::new(input);

    report.gamma_rate() * report.epsilon_rate()
}

mod test {
    #[test]
    fn day_03_01_test() {
        assert_eq!(super::run(include_str!("input/day_03_test.txt")), 198);
    }

    #[test]
    fn day_03_01() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 693486);
    }
}
