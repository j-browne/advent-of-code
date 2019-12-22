use aoc_2019::{
    intcode::Machine,
    painting_robot::{Color, Position, Robot},
};
use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day11_2(stdin().lock()));
}

fn day11_2(input: impl BufRead) -> String {
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
    let _ = robot
        .painted_tiles
        .insert(Position { x: 0, y: 0 }, Color::White);

    robot.run().unwrap();
    robot.painted_image()
}

#[cfg(test)]
mod test {
    #[test]
    fn day11_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day11_2(BufReader::new(File::open("input/input_day11.txt").unwrap())),
            "\
╔═══════════════════════════════════════════╗
║  ██   ██   ██  █    ███   ██    ██ ████   ║
║ █  █ █  █ █  █ █    █  █ █  █    █ █      ║
║ █  █ █    █  █ █    █  █ █       █ ███    ║
║ ████ █ ██ ████ █    ███  █ ██    █ █      ║
║ █  █ █  █ █  █ █    █ █  █  █ █  █ █      ║
║ █  █  ███ █  █ ████ █  █  ███  ██  ████   ║
╚═══════════════════════════════════════════╝\
            "
        );
    }
}
