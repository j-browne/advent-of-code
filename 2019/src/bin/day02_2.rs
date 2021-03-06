use aoc_2019::intcode::Machine;
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day02_2(stdin().lock()));
}

fn day02_2(input: impl BufRead) -> i64 {
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

    let machine = Machine::with_memory(memory);
    let mut output = None;

    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut machine = machine.clone();
            *machine.get_memory_mut(1) = i;
            *machine.get_memory_mut(2) = j;

            machine.run().unwrap();

            if machine.get_memory(0) == 19_690_720 {
                output = Some(100 * i + j);
                break 'outer;
            }
        }
    }

    output.unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn day02_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day02_2(BufReader::new(File::open("input/input_day02.txt").unwrap())),
            9074
        );
    }
}
