use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day13_1(stdin().lock()));
}

fn day13_1(input: impl BufRead) -> usize {
    let memory: Vec<i64> = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut machine = Machine::with_memory(memory);

    machine.run().unwrap();
    machine
        .output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|x| **x == 2)
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn day13_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day13_1(BufReader::new(File::open("input/input_day13.txt").unwrap())),
            398
        );
    }
}
