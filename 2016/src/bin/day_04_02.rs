use aoc_2016::sector_id::shift;
use regex::Regex;

fn main() {
    println!("{}", run(include_str!("input/day_04.txt")));
}

fn run(s: &str) -> u32 {
    let re = Regex::new(r"([a-z-]+)-([[:digit:]]+)\[([a-z]+)\]").unwrap();

    for line in s.trim().split('\n') {
        for cap in re.captures_iter(line) {
            let sector = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let sh = (sector % 26) as u8;
            if shift(cap.get(1).unwrap().as_str(), sh) == "northpole object storage" {
                return sector;
            }
        }
    }
    panic!("'northpole object storage' not found")
}

mod test {
    #[test]
    fn day_04_02() {
        assert_eq!(super::run(include_str!("input/day_04.txt")), 324);
    }
}
