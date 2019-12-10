use std::{collections::VecDeque, convert::TryFrom};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    UnknownOpCode,
    UnknownParameterMode,
    EmptyInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Stop,
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    EqualTo(ParameterMode, ParameterMode, ParameterMode),
}

impl TryFrom<i32> for Instruction {
    type Error = Error;
    fn try_from(mut value: i32) -> Result<Self, Self::Error> {
        let code = value % 100;
        value /= 100;
        match code {
            1 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_2 = ParameterMode::try_from(value % 10)?;
                Ok(Self::Add(mode_0, mode_1, mode_2))
            }
            2 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_2 = ParameterMode::try_from(value % 10)?;
                Ok(Self::Mul(mode_0, mode_1, mode_2))
            }
            3 => {
                let mode = ParameterMode::try_from(value % 10)?;
                Ok(Self::Input(mode))
            }
            4 => {
                let mode = ParameterMode::try_from(value % 10)?;
                Ok(Self::Output(mode))
            }
            5 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                Ok(Self::JumpIfTrue(mode_0, mode_1))
            }
            6 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                Ok(Self::JumpIfFalse(mode_0, mode_1))
            }
            7 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_2 = ParameterMode::try_from(value % 10)?;
                Ok(Self::LessThan(mode_0, mode_1, mode_2))
            }
            8 => {
                let mode_0 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_1 = ParameterMode::try_from(value % 10)?;
                value /= 10;
                let mode_2 = ParameterMode::try_from(value % 10)?;
                Ok(Self::EqualTo(mode_0, mode_1, mode_2))
            }
            99 => Ok(Self::Stop),
            _ => Err(Self::Error::UnknownOpCode),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<i32> for ParameterMode {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            _ => Err(Self::Error::UnknownParameterMode),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Machine {
    pub memory: Vec<i32>,
    pub cursor: usize,
    pub input: VecDeque<i32>,
    pub output: Vec<i32>,
}

impl Machine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_memory<M: Into<Vec<i32>>>(memory: M) -> Self {
        let memory = memory.into();
        Self {
            memory,
            ..Self::default()
        }
    }

    pub fn with_memory_input<M: Into<Vec<i32>>, I: Into<VecDeque<i32>>>(
        memory: M,
        input: I,
    ) -> Self {
        let memory = memory.into();
        let input = input.into();
        Self {
            memory,
            input,
            ..Self::default()
        }
    }

    fn get(&self, input: i32, mode: ParameterMode) -> i32 {
        use ParameterMode::*;
        match mode {
            Position => self.memory[input as usize],
            Immediate => input,
        }
    }

    fn get_mut(&mut self, input: i32, mode: ParameterMode) -> &mut i32 {
        use ParameterMode::*;
        match mode {
            Position => &mut self.memory[input as usize],
            Immediate => panic!("trying to write in Immediate Mode"),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while Instruction::try_from(self.memory[self.cursor])? != Instruction::Stop {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Error> {
        match Instruction::try_from(self.memory[self.cursor])? {
            Instruction::Add(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                let val_2 = self.get_mut(self.memory[self.cursor + 3], mode_2);
                *val_2 = val_0 + val_1;
                self.cursor += 4;
            }
            Instruction::Mul(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                let val_2 = self.get_mut(self.memory[self.cursor + 3], mode_2);
                *val_2 = val_0 * val_1;
                self.cursor += 4;
            }
            Instruction::Input(mode) => {
                let input = self.input.pop_front().ok_or(Error::EmptyInput)?;
                let val = self.get_mut(self.memory[self.cursor + 1], mode);
                *val = input;
                self.cursor += 2;
            }
            Instruction::Output(mode) => {
                let val = self.get(self.memory[self.cursor + 1], mode);
                self.output.push(val);
                self.cursor += 2;
            }
            Instruction::JumpIfTrue(mode_0, mode_1) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                if val_0 != 0 {
                    self.cursor = val_1 as usize;
                } else {
                    self.cursor += 3;
                }
            }
            Instruction::JumpIfFalse(mode_0, mode_1) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                if val_0 == 0 {
                    self.cursor = val_1 as usize;
                } else {
                    self.cursor += 3;
                }
            }
            Instruction::LessThan(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                let val_2 = self.get_mut(self.memory[self.cursor + 3], mode_2);
                *val_2 = if val_0 < val_1 { 1 } else { 0 };
                self.cursor += 4;
            }
            Instruction::EqualTo(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.memory[self.cursor + 1], mode_0);
                let val_1 = self.get(self.memory[self.cursor + 2], mode_1);
                let val_2 = self.get_mut(self.memory[self.cursor + 3], mode_2);
                *val_2 = if val_0 == val_1 { 1 } else { 0 };
                self.cursor += 4;
            }
            Instruction::Stop => {}
        }
        Ok(())
    }
}
