use aoc_2019::space_image::SpaceImage;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day08_1(stdin().lock()));
}

fn day08_1(input: impl BufRead) -> u32 {
    let digits: Vec<u32> = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let image = SpaceImage::new(25, 6, digits);

    image
        .occurrences_in_layers()
        .iter()
        .min_by_key(|x| x.get(&0).unwrap_or(&0))
        .map(|x| x.get(&1).unwrap_or(&0) * x.get(&2).unwrap_or(&0))
        .unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day08_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day08_1(BufReader::new(File::open("input/input_day08.txt").unwrap())),
            2159
        );
    }
}
