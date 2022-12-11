use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crt {
    cycle: usize,
    reg_x: i32,
    instructions: VecDeque<Instruction>,
    x_values: Vec<i32>,
}

impl Crt {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let instructions = s.lines().map(Instruction::new).collect();

        Self {
            cycle: 0,
            reg_x: 1,
            instructions,
            x_values: vec![1],
        }
    }

    fn step(&mut self) {
        match self.instructions.pop_front().unwrap() {
            Instruction::Noop => {
                self.cycle += 1;
                self.x_values.push(self.reg_x);
            }
            Instruction::AddX(dx) => {
                self.cycle += 2;
                self.x_values.push(self.reg_x);
                self.x_values.push(self.reg_x);
                self.reg_x += dx;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.instructions.is_empty() {
            self.step();
        }
    }

    #[must_use]
    pub const fn x_values(&self) -> &Vec<i32> {
        &self.x_values
    }

    pub fn render(&mut self) -> String {
        self.run();

        let mut s = String::new();
        for i in 0..6 {
            for j in 0..40 {
                let idx = usize::try_from(i * 40 + j + 1).unwrap();
                let p = if (j - self.x_values[idx]).abs() <= 1 {
                    '#'
                } else {
                    '.'
                };
                s.push(p);
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        match it.next().unwrap() {
            "noop" => Self::Noop,
            "addx" => Self::AddX(it.next().unwrap().parse().unwrap()),
            i => panic!("unknown instruction '{i}'"),
        }
    }
}
