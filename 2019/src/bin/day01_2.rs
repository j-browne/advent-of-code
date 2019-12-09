use std::io::{stdin, BufRead};

fn main() {
    let fuel_sum: i32 = stdin()
        .lock()
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
        .sum();

    println!("{}", fuel_sum);
}
