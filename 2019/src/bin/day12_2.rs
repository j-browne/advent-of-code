use aoc_2019::n_body::{NBody, Vector};
use num::Integer;
use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    println!("{}", day12_1(stdin().lock()));
}

fn day12_1(input: impl BufRead) -> u64 {
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
    let mut visited = [HashMap::new(), HashMap::new(), HashMap::new()];
    let mut periods = [None; 3];
    let mut i = 0;
    loop {
        n_body.step();
        i += 1;
        for j in 0..3 {
            if periods[j].is_none() {
                if let Some(k) = visited[j].insert(
                    (
                        n_body.positions()[j].clone(),
                        n_body.velocities()[j].clone(),
                    ),
                    i,
                ) {
                    periods[j] = Some(i - k);
                }
            }
        }
        if let [Some(a), Some(b), Some(c)] = periods {
            return a.lcm(&b).lcm(&c);
        }
    }
}

#[cfg(test)]
mod test {
    //#[test]
    fn day12_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day12_1(BufReader::new(File::open("input/input_day12.txt").unwrap())),
            unimplemented!()
        );
    }
}
