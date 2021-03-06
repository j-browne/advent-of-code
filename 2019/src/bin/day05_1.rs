use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day05_1(stdin().lock()));
}

fn day05_1(input: impl BufRead) -> i64 {
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

    let mut machine = Machine::with_memory_input(memory, vec![1]);

    machine.run().unwrap();
    *machine.output.back().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day05_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day05_1(BufReader::new(File::open("input/input_day05.txt").unwrap())),
            6745903
        );
    }
}
