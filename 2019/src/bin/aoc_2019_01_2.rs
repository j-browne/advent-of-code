use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day01_2(stdin().lock()));
}

fn day01_2(input: impl BufRead) -> i32 {
    input
        .lines()
        .map(|x| {
            let mass: i32 = x.unwrap().parse().unwrap();
            let mut fuel: i32 = 0;
            let mut add_fuel = mass;

            #[allow(clippy::while_immutable_condition)]
            while {
                add_fuel = add_fuel / 3 - 2;
                add_fuel > 0
            } {
                fuel += add_fuel;
            }

            fuel
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn day01_2() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day01_2(BufReader::new(File::open("input/input_day01.txt").unwrap())),
            5055835
        );
    }
}
