use std::str::FromStr;

fn main() {
    println!("{}", run(include_str!("input/day_03.txt")));
}

fn run(s: &str) -> u32 {
    let mut sum = 0;
    for line in s.trim().split('\n') {
        let mut sides = line
            .trim()
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        sides.sort_unstable();
        if sides[0] + sides[1] > sides[2] {
            sum += 1;
        }
    }
    sum
}

mod test {
    #[test]
    fn day_03_01() {
        assert_eq!(super::run(include_str!("input/day_03.txt")), 982);
    }
}
