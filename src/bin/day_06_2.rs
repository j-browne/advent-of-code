use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = ::std::io::stdin();
    let mut v = Vec::<HashMap<char, u32>>::new();

    for line in stdin.lock().lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            if i >= v.len() {
                v.resize(i + 1, HashMap::<char, u32>::new());
            }

            *v[i].entry(c).or_insert(0) += 1;
        }
    }

    let mut message = String::new();
    for m in v {
        let mut max: Option<(char, u32)> = None;
        for (c, n) in m {
            if let Some((_, n_max)) = max {
                if n < n_max {
                    max = Some((c, n));
                }
            } else {
                max = Some((c, n));
            }
        }
        message.push(max.unwrap().0);
    }
    println!("{}", message);
}
