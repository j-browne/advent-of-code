use std::io::{stdin, BufRead};

fn main() {
    let fuel_sum: u32 = stdin()
        .lock()
        .lines()
        .map(|x| {
            let mass: u32 = x.unwrap().parse().unwrap();
            mass / 3 - 2
        })
        .sum();

    println!("{}", fuel_sum);
}
