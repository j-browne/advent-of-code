use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let input = stdin()
        .lock()
        .lines()
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .split('-')
        .take(2)
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let valids = (input[0]..=input[1]).filter(|x| valid(*x)).count();
    println!("{:?}", valids);
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
