use aoc_2019::space_image::SpaceImage;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day08_2(stdin().lock()));
}

fn day08_2(input: impl BufRead) -> String {
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

    format!("{}", image.render())
}

#[cfg(test)]
mod test {
    #[test]
    fn day08_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day08_2(BufReader::new(File::open("input/input_day08.txt").unwrap())),
            "\
╔═════════════════════════╗
║█  ████  █    █ ██ █   ██║
║ ██ ████ ████ █ ██ █ ██ █║
║ ███████ ███ ██    █ ██ █║
║ ███████ ██ ███ ██ █   ██║
║ ██ █ ██ █ ████ ██ █ █ ██║
║█  ███  ██    █ ██ █ ██ █║
╚═════════════════════════╝\
            "
        );
    }
}
