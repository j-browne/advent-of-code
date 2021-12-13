use aoc_2021::seven_segment_display::Display;

fn main() {
    println!("{}", run(include_str!("input/day_08.txt")));
}

fn run(input: &str) -> u32 {
    let it = input.trim().split('\n').map(Display::new);

    it.map(|x| x.output_value()).sum()
}

mod test {
    #[test]
    fn day_08_02_test() {
        assert_eq!(super::run(include_str!("input/day_08_test.txt")), 61229);
    }

    #[test]
    fn day_08_02() {
        assert_eq!(super::run(include_str!("input/day_08.txt")), 1091609);
    }
}
