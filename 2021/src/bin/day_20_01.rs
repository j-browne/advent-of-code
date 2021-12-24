use aoc_2021::image_enhancement::Image;

fn main() {
    println!("{}", run(include_str!("input/day_20.txt")));
}

fn run(input: &str) -> usize {
    let mut image = Image::new(input);
    image.enhance_n(2);
    image.lit_pixels()
}

mod test {
    #[test]
    fn day_20_01_test() {
        assert_eq!(super::run(include_str!("input/day_20_test.txt")), 35);
    }

    #[test]
    fn day_20_01() {
        assert_eq!(super::run(include_str!("input/day_20.txt")), 5081);
    }
}
