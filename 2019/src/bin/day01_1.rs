use std::io::{stdin, BufRead};

fn main() {
    println!("{}", day01_1(stdin().lock()));
}

fn day01_1(input: impl BufRead) -> u32 {
    input
        .lines()
        .map(|x| {
            let mass: u32 = x.unwrap().parse().unwrap();
            mass / 3 - 2
        })
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn day01_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day01_1(BufReader::new(File::open("input/input_day01.txt").unwrap())),
            3372463
        );
    }
}
