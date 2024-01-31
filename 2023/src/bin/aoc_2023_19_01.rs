use aoc_2023::workflow::{Part, Workflow};

fn main() {
    println!("{}", run(include_str!("input/aoc_2023_19.txt")));
}

fn run(input: &str) -> u32 {
    let mut it = input.split("\n\n");
    let workflows = it.next().unwrap().lines().map(Workflow::new).collect();
    it.next()
        .unwrap()
        .lines()
        .filter_map(|l| {
            let part = Part::new(l);
            part.accepted(&workflows).then(|| part.sum())
        })
        .sum()
}

mod test {
    #[test]
    fn aoc_2023_19_01_test() {
        assert_eq!(
            super::run(include_str!("input/aoc_2023_19_test.txt")),
            19114
        );
    }

    #[test]
    fn aoc_2023_19_01() {
        assert_eq!(super::run(include_str!("input/aoc_2023_19.txt")), 489392);
    }
}
