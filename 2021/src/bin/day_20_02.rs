use aoc_2021::image_enhancement::Image;

fn main() {
    println!("{}", run(include_str!("input/day_20.txt")));
}

fn run(input: &str) -> usize {
    let mut image = Image::new(input);
    image.enhance_n(50);
    image.lit_pixels()
}

mod test {
    #[test]
    fn day_20_02_test() {
        assert_eq!(super::run(include_str!("input/day_20_test.txt")), 3351);
    }

    #[test]
    fn day_20_02() {
        assert_eq!(super::run(include_str!("input/day_20.txt")), 15088);
    }
}
