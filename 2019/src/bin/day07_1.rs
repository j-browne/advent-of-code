use aoc_2019::intcode::Machine;
use permutohedron::Heap;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day07_1(stdin().lock()));
}

fn day07_1(input: impl BufRead) -> i64 {
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

    Heap::new(&mut [0, 1, 2, 3, 4])
        .map(|p| run_amplifiers(&memory, &p))
        .max()
        .unwrap()
}

fn run_amplifiers(program: &[i64], phase_settings: &[i64]) -> i64 {
    let mut signal = 0;
    for p in phase_settings {
        let mut machine = Machine::with_memory_input(program, vec![*p, signal]);
        machine.run().unwrap();
        signal = machine.output[0];
    }
    signal
}

#[cfg(test)]
mod test {
    #[test]
    fn day07_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day07_1(BufReader::new(File::open("input/input_day07.txt").unwrap())),
            117312
        );
    }
}
