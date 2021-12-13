use aoc_2016::sector_id::checksum;
use regex::Regex;

fn main() {
    println!("{}", run(include_str!("input/day_04.txt")));
}

fn run(s: &str) -> u32 {
    let re = Regex::new(r"([a-z-]+)-([[:digit:]]+)\[([a-z]+)\]").unwrap();
    let mut sum = 0;

    for line in s.trim().split('\n') {
        for cap in re.captures_iter(line) {
            if checksum(cap.get(1).unwrap().as_str()) == cap.get(3).unwrap().as_str() {
                sum += cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            }
        }
    }

    sum
}

mod test {
    #[test]
    fn day_04_01_test() {
        assert_eq!(super::run(include_str!("input/day_04_test.txt")), 1514);
    }

    #[test]
    fn day_04_01() {
        assert_eq!(super::run(include_str!("input/day_04.txt")), 245102);
    }
}
