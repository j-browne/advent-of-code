use aoc_2021::seven_segment_display::Display;

fn main() {
    println!("{}", run(include_str!("input/day_08.txt")));
}

fn run(input: &str) -> usize {
    let it = input.trim().split('\n').map(Display::new);

    it.map(|x| x.output_uniques()).sum()
}

mod test {
    #[test]
    fn day_08_01_test() {
        assert_eq!(super::run(include_str!("input/day_08_test.txt")), 26);
    }

    #[test]
    fn day_08_01() {
        assert_eq!(super::run(include_str!("input/day_08.txt")), 456);
    }
}
