use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpCode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Mul,
    Stop,
}

impl TryFrom<u32> for OpCode {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Stop),
            _ => Err(Self::Error::UnknownOpCode),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub memory: Vec<u32>,
    pub cursor: usize,
}

impl Machine {
    pub fn run(&mut self) -> Result<(), Error> {
        while OpCode::try_from(self.memory[self.cursor])? != OpCode::Stop {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Error> {
        match OpCode::try_from(self.memory[self.cursor])? {
            OpCode::Add => {
                let idx_1 = self.memory[self.cursor + 1] as usize;
                let idx_2 = self.memory[self.cursor + 2] as usize;
                let idx_3 = self.memory[self.cursor + 3] as usize;
                self.memory[idx_3] = self.memory[idx_1] + self.memory[idx_2];
                self.cursor += 4;
            }
            OpCode::Mul => {
                let idx_1 = self.memory[self.cursor + 1] as usize;
                let idx_2 = self.memory[self.cursor + 2] as usize;
                let idx_3 = self.memory[self.cursor + 3] as usize;
                self.memory[idx_3] = self.memory[idx_1] * self.memory[idx_2];
                self.cursor += 4;
            }
            OpCode::Stop => {}
        }
        Ok(())
    }
}
