use aoc_2021::trick_shot::Target;

fn main() {
    println!("{}", run(include_str!("input/day_17.txt")));
}

fn run(input: &str) -> u32 {
    let target = Target::new(input);
    target.num_velocities()
}

mod test {
    #[test]
    fn day_17_02_test() {
        assert_eq!(super::run(include_str!("input/day_17_test.txt")), 112);
    }

    #[test]
    fn day_17_02() {
        assert_eq!(super::run(include_str!("input/day_17.txt")), 2118);
    }
}
