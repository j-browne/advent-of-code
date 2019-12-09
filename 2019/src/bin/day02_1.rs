use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day02_1(stdin().lock()));
}

fn day02_1(input: impl BufRead) -> u32 {
    let memory: Vec<u32> = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut machine = Machine { memory, cursor: 0 };

    machine.memory[1] = 12;
    machine.memory[2] = 2;

    machine.run().unwrap();
    machine.memory[0]
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
