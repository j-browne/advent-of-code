use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    println!("{}", day04_1(stdin().lock()));
}

fn day04_1(input: impl BufRead) -> usize {
    let bounds = input
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .split('-')
        .take(2)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (bounds[0]..=bounds[1]).filter(|x| valid(*x)).count()
}

fn valid(pass: u32) -> bool {
    const PASS_LEN: usize = 6;
    let digits = (0..PASS_LEN)
        .scan(pass, |state, _| {
            let digit = *state % 10;
            *state /= 10;
            Some(digit)
        })
        .collect::<Vec<u32>>();

    let monotonic = digits
        .iter()
        .zip(digits.iter().skip(1))
        .all(|(a, b)| a >= b);

    let double = digits
        .iter()
        .fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_insert(0) += 1;
            map
        })
        .iter()
        .any(|(_a, b)| *b >= 2);

    monotonic && double
}

#[cfg(test)]
mod test {
    #[test]
    fn day04_1() {
        use std::{fs::File, io::BufReader};

        assert_eq!(
            super::day04_1(BufReader::new(File::open("input/input_day04.txt").unwrap())),
            579
        );
    }
}
