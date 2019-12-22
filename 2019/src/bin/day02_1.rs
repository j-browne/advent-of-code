use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day02_1(stdin().lock()));
}

fn day02_1(input: impl BufRead) -> i64 {
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

    *machine.get_memory_mut(1) = 12;
    *machine.get_memory_mut(2) = 2;

    machine.run().unwrap();
    machine.get_memory(0)
}

#[cfg(test)]
mod test {
    #[test]
    fn day02_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day02_1(BufReader::new(File::open("input/input_day02.txt").unwrap())),
            2842648
        );
    }
}
