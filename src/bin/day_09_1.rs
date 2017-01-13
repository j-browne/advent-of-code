use std::io::Read;
use State::*;

#[derive(Debug)]
enum State {
    Closed,
    Marker0,
    Marker1(usize),
    Data(usize, usize),
}

fn process(s: &str) -> String {
    let mut res = String::new();
    let mut state = Closed;
    let mut buf = String::new();

    for c in s.chars() {
        match (&state, c) {
            (&Closed, '(') => {
                state = Marker0;
            }
            (&Closed, _) => {
                if !c.is_whitespace() {
                    res.push(c);
                }
            }
            (&Marker0, 'x') => {
                state = Marker1(buf.parse::<usize>().unwrap());
                buf.clear();
            }
            (&Marker0, _) => {
                if !c.is_whitespace() {
                    buf.push(c);
                }
            }
            (&Marker1(n), ')') => {
                state = Data(n, buf.parse::<usize>().unwrap());
                buf.clear();
            }
            (&Marker1(_), _) => {
                if !c.is_whitespace() {
                    buf.push(c);
                }
            }
            (&Data(n, m), _) => {
                if !c.is_whitespace() {
                    buf.push(c);
                }
                if buf.len() == n {
                    for _ in 0..m {
                        res.push_str(&buf);
                    }
                    state = Closed;
                    buf.clear();
                }
            }
        }
    }

    res
}

fn main() {
    let mut stdin = ::std::io::stdin();
    let mut text = String::new();
    let _ = stdin.read_to_string(&mut text).unwrap();
    println!("{}", process(&text).len());
}
