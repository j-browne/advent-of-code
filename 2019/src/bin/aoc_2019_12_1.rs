use aoc_2019::n_body::{NBody, Vector};
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    println!("{}", day12_1(stdin().lock()));
}

fn day12_1(input: impl BufRead) -> u32 {
    let positions: Vec<Vector> = input
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let map = line
                .trim_start_matches('<')
                .trim_end_matches('>')
                .split(',')
                .map(|part| {
                    let mut iter = part.trim().split('=');
                    (iter.next().unwrap(), iter.next().unwrap())
                })
                .collect::<HashMap<_, _>>();
            Vector::new(
                map[&"x"].parse::<i32>().unwrap(),
                map[&"y"].parse::<i32>().unwrap(),
                map[&"z"].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut n_body = NBody::with_positions(positions);
    for _ in 0..1000 {
        n_body.step();
    }
    n_body.total_energy()
}

#[cfg(test)]
mod test {
    #[test]
    fn day12_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day12_1(BufReader::new(File::open("input/input_day12.txt").unwrap())),
            10944
        );
    }
}
