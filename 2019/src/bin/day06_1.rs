use aoc_2019::orbits::Orbits;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day06_1(stdin().lock()));
}

fn day06_1(input: impl BufRead) -> u32 {
    let orbits: Orbits = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let ids: Vec<_> = line.trim().split(')').take(2).collect();
            (ids[0].into(), ids[1].into())
        })
        .collect();

    orbits.num_orbits(1, "COM")
}

#[cfg(test)]
mod test {
    #[test]
    fn day06_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day06_1(BufReader::new(File::open("input/input_day06.txt").unwrap())),
            223251
        );
    }
}
