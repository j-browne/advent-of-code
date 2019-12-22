use aoc_2019::{intcode::Machine, painting_robot::Robot};
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day11_1(stdin().lock()));
}

fn day11_1(input: impl BufRead) -> u32 {
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
    let mut robot = Robot::with_machine(machine);

    robot.run().unwrap();
    robot.num_painted()
}

#[cfg(test)]
mod test {
    #[test]
    fn day11_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day11_1(BufReader::new(File::open("input/input_day11.txt").unwrap())),
            2252
        );
    }
}
