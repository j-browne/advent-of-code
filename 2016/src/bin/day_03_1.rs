use std::io::BufRead;

fn main() {
    let stdin = ::std::io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let sides = line.unwrap();
        let sides: Vec<&str> = sides.split_whitespace().collect();
        let mut sides: Vec<u32> = sides.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
