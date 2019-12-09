use advent_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    let memory: Vec<u32> = stdin()
        .lock()
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

    println!("{}", machine.memory[0]);
}
