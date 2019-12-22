use aoc_2019::intcode::{Machine, Return};
use permutohedron::Heap;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day07_2(stdin().lock()));
}

fn day07_2(input: impl BufRead) -> i64 {
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

    Heap::new(&mut [5, 6, 7, 8, 9])
        .map(|p| run_amplifiers(&memory, &p))
        .max()
        .unwrap()
}

fn run_amplifiers(program: &[i64], phase_settings: &[i64]) -> i64 {
    let mut machines = phase_settings
        .iter()
        .map(|p| Machine::with_memory_input(program, vec![*p]))
        .collect::<Vec<_>>();

    let mut signal = 0;
    let mut res = Return::EmptyInput;
    while res != Return::Stopped {
        for m in machines.iter_mut() {
            m.input.push_back(signal);
            res = m.run().unwrap();
            signal = m.output.pop_front().unwrap();
        }
    }

    signal
}

#[cfg(test)]
mod test {
    #[test]
    fn day07_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day07_2(BufReader::new(File::open("input/input_day07.txt").unwrap())),
            1336480
        );
    }
}
