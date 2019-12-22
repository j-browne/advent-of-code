extern crate pcre;

use pcre::Pcre;
use std::fmt;
use std::io::BufRead;

struct Disp {
    data: [[bool; 50]; 6],
}

impl Disp {
    fn new() -> Disp {
        Disp {
            data: [[false; 50]; 6]
        }
    }

    fn rect(&mut self, cols: usize, rows: usize) {
        for i in 0..rows {
            for j in 0..cols {
                self.data[i][j] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, num: usize) {
        let max_col = self.data[row].len() - 1;

        for _ in 0..num {
            let mut last = self.data[row][max_col];
            for col in 0..(max_col + 1) {
                ::std::mem::swap(&mut last, &mut self.data[row][col]);
            }
        }
    }

    fn rotate_col(&mut self, col: usize, num: usize) {
        let max_row = self.data.len() - 1;

        for _ in 0..num {
            let mut last = self.data[max_row][col];
            for row in 0..(max_row + 1) {
                ::std::mem::swap(&mut last, &mut self.data[row][col]);
            }
        }
    }

    fn parse_command(&mut self, c: &str) {
        let mut re_rect: Pcre = Pcre::compile(r"rect ([0-9]+)x([0-9]+)").unwrap();
        let mut re_rot_row: Pcre = Pcre::compile(r"rotate row y=([0-9]+) by ([0-9]+)").unwrap();
        let mut re_rot_col: Pcre = Pcre::compile(r"rotate column x=([0-9]+) by ([0-9]+)").unwrap();

        let mut comm_sum = 0;

        for m_comm in re_rect.matches(c) {
            comm_sum += 1;
            let cols = m_comm.group(1).parse::<usize>().unwrap();
            let rows = m_comm.group(2).parse::<usize>().unwrap();
            self.rect(cols, rows);
        }
        for m_comm in re_rot_row.matches(c) {
            comm_sum += 1;
            let row = m_comm.group(1).parse::<usize>().unwrap();
            let num = m_comm.group(2).parse::<usize>().unwrap();
            self.rotate_row(row, num);
        }
        for m_comm in re_rot_col.matches(c) {
            comm_sum += 1;
            let col = m_comm.group(1).parse::<usize>().unwrap();
            let num = m_comm.group(2).parse::<usize>().unwrap();
            self.rotate_col(col, num);
        }
        if comm_sum != 1 {
            println!("ERROR PARSING {:?}: {} commands matched", c, comm_sum);
        }
    }

    fn num_lit(&self) -> u32 {
        let mut sum = 0u32;
        for row in self.data.iter() {
            for i in row.iter() {
                if *i {
                    sum += 1;
                }
            }
        }
        sum
    }
}

impl fmt::Display for Disp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            for i in row.iter() {
                write!(f, "{}", if *i {'#'} else {'.'})?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut d = Disp::new();
    let stdin = ::std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        d.parse_command(&line);
    }

    println!("{}", d.num_lit());
}
