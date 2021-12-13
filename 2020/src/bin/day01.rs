fn main() {
    let input = include_str!("input/day01.txt");
    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    'outer: for x in &nums {
        for y in &nums {
            if x + y == 2020 {
                println!("{}", x * y);
                break 'outer;
            }
        }
    }
}
