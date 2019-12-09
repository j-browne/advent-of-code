extern crate regex;

use regex::Regex;
use std::io::BufRead;
use std::collections::HashMap;

#[allow(dead_code)]
fn checksum(s: &str) -> String {
    let mut m = HashMap::<char, u32>::new();
    let mut v = Vec::<(char, u32)>::new();
    for c in s.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    for i in m {
        v.push(i);
    }
    v.sort_by(|x, y| {
        let mut o = x.1.cmp(&y.1);
        if o == ::std::cmp::Ordering::Equal {
            o = x.0.cmp(&y.0).reverse();
        }
        o.reverse()
    });
    let mut s = String::new();
    for i in v {
        if i.0 != '-' {
            s.push(i.0);
        }
    }
    s.truncate(5);
    s
}

fn shift(s: &str, i: u8) -> String {
    let mut out = String::new();
    for c in s.chars() {
        let mut new_c = c;
        for _ in 0..i {
            match new_c {
                '-' | ' ' => { new_c = ' '; },
                'z' => { new_c = 'a' },
                _ => { new_c = ((new_c as u8) + 1) as char; },
            };
        }
        out.push(new_c);
    }
    out
}

fn main() {
    let re = Regex::new(r"([a-z-]+)-([[:digit:]]+)\[([a-z]+)\]").unwrap();
    let stdin = ::std::io::stdin();

    for line in stdin.lock().lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            for i in 1..27 {
                let sector = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                println!("{}: {}", shift(cap.get(1).unwrap().as_str(), i), sector);
            }
        }
    }
}
