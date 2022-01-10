use aoc_2021::beacons::Beacons;

fn main() {
    println!("{}", run(include_str!("input/day_19.txt")));
}

fn run(input: &str) -> u32 {
    let _beacons = Beacons::new(input);
    todo!()
}

mod test {
    #[test]
    fn day_19_01_test() {
        assert_eq!(super::run(include_str!("input/day_19_test.txt")), 79);
    }

    #[test]
    fn day_19_01() {
        assert_eq!(super::run(include_str!("input/day_19.txt")), 0);
    }
}
