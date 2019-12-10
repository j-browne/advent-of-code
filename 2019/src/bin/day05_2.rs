use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day05_2(stdin().lock()));
}

fn day05_2(input: impl BufRead) -> i32 {
    let memory: Vec<i32> = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut machine = Machine::with_memory_input(memory, vec![5]);

    machine.run().unwrap();
    *machine.output.last().unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day05_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day05_2(BufReader::new(File::open("input/input_day05.txt").unwrap())),
            9168267
        );
    }
}
