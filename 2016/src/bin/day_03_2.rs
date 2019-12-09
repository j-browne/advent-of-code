use std::io::BufRead;

fn main() {
    let stdin = ::std::io::stdin();

    let mut sum = 0;
    let mut line_no = 0;
    let mut vecs = [Vec::<u32>::new(), Vec::<u32>::new(), Vec::<u32>::new()];

    for line in stdin.lock().lines() {
        if line_no != 0 && line_no % 3 == 0 {
            for v in vecs.iter_mut() {
                v.sort();
                if v[0] + v[1] > v[2] {
                    sum += 1;
                }
                v.clear();
            }
        }

        let sides = line.unwrap();
        let sides: Vec<&str> = sides.split_whitespace().collect();
        let sides: Vec<u32> = sides.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        for i in 0..3 {
            vecs[i].push(sides[i]);
        }

        line_no += 1;
    }

    for v in vecs.iter_mut() {
        v.sort();
        if v[0] + v[1] > v[2] {
            sum += 1;
        }
        v.clear();
    }

    println!("{}", sum);
}
