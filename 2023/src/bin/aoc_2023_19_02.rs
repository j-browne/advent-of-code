use aoc_2023::workflow::{accepted_combination_count, Workflow};

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_19.txt")));
}

fn run(input: &str) -> u64 {
    let mut it = input.split("\n\n");
    let workflows = it.next().unwrap().lines().map(Workflow::new).collect();
    accepted_combination_count(&workflows)
}

mod test {
    #[test]
    fn aoc_2023_19_02_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_19_test.txt")),
            167409079868000
        );
    }

    #[test]
    fn aoc_2023_19_02() {
        assert_eq!(super::run(include_str!("input/aoc_2023_19.txt")), 489392);
    }
}
