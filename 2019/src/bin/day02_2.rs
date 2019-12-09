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

    let machine = Machine { memory, cursor: 0 };

    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut machine = machine.clone();
            machine.memory[1] = i;
            machine.memory[2] = j;

            machine.run().unwrap();

            if machine.memory[0] == 19_690_720 {
                println!("{}", 100 * i + j);
                break 'outer;
            }
        }
    }
}
