use aoc_2019::{arcade::Arcade, intcode::{Machine, Return}};
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    thread,
    time,
};

fn main() {
    println!("{}", day13_2(BufReader::new(File::open("input/input_day13.txt").unwrap())));
}

fn day13_2(input: impl BufRead) -> i64 {
    let mut memory: Vec<i64> = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    memory[0] = 2;
    let machine = Machine::with_memory(memory);
    let mut arcade = Arcade::with_machine(machine);

    let mut score = None;
    let mut line = String::new();
    while arcade.machine.run().unwrap() != Return::Stopped {
        let render = arcade.render();
        if let Some(x) = render.1 {
            score = Some(x);
        }

        //println!("Score: {}", score.unwrap_or(0));
        //println!("{}", render.0);

        'inner: loop {
            line.clear();
            stdin().lock().read_line(&mut line).unwrap();
            let line = line.trim().parse::<i64>();

            if let Ok(x) = line {
                arcade.machine.input.push_back(x);
                break 'inner;
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }

    let render = arcade.render();
    if let Some(x) = render.1 {
        score = Some(x);
    }
    score.unwrap()
}

#[cfg(test)]
mod test {
    //#[test]
    fn day13_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day13_2(BufReader::new(File::open("input/input_day13.txt").unwrap())),
            unimplemented!()
        );
    }
}
