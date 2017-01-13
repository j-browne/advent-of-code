use std::io::Read;

use Part::*;
use State::*;

#[derive(Debug)]
enum State {
    Closed,
    Marker0,
    Marker1(usize),
}

#[derive(Debug)]
enum Part {
    Ch(char),
    Co(u64, Compressed),
}

impl Part {
    fn size(&self) -> u64 {
        match self {
            &Ch(_) => 1,
            &Co(ref n, ref c) => n * c.size(),
        }
    }
}

#[derive(Debug)]
struct Compressed {
    v: Vec<Part>,
}

impl Compressed {
    fn process(s: &[u8]) -> Compressed {
//        println!("{:?}", ::std::str::from_utf8(s));
        let mut v = Vec::<Part>::new();
        let mut state = Closed;
        let mut buf = String::new();

        let mut i = 0;
        while i < s.len() {
            let c = s[i];
            match (&state, c) {
                (&Closed, b'(') => {
                    state = Marker0;
                }
                (&Closed, _) => {
                    if !(c as char).is_whitespace() {
                        v.push(Ch(c as char));
                    }
                }
                (&Marker0, b'x') => {
                    state = Marker1(buf.parse::<usize>().unwrap());
                    buf.clear();
                }
                (&Marker0, _) => {
                    if !(c as char).is_whitespace() {
                        buf.push(c as char);
                    }
                }
                (&Marker1(n), b')') => {
                    let m = buf.parse::<u64>().unwrap();
                    state = Closed;
                    buf.clear();
                    v.push(Co(m, Compressed::process(&s[(i + 1)..(i + n + 1)])));
                    i += n;
                }
                (&Marker1(_), _) => {
                    if !(c as char).is_whitespace() {
                        buf.push(c as char);
                    }
                }
            }
            i += 1;
        }

        Compressed {
            v: v
        }
    }

    fn size(&self) -> u64 {
        let mut sum = 0;
        for i in &self.v {
            sum += i.size();
        }
        sum
    }
}

fn main() {
    let mut stdin = ::std::io::stdin();
    let mut text = String::new();
    let _ = stdin.read_to_string(&mut text).unwrap();
    let c = Compressed::process(text.as_bytes());
    println!("{}", c.size());
}
