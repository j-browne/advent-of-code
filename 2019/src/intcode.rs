use std::{cell::RefCell, collections::VecDeque, convert::TryFrom, iter::repeat};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Return {
    Stopped,
    EmptyInput,
}

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
    AdjustRelativeBase(ParameterMode),
}

impl TryFrom<i64> for Instruction {
    type Error = Error;
    fn try_from(mut value: i64) -> Result<Self, Self::Error> {
        let op_code = value % 100;
        value /= 100;
        match op_code {
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
            9 => {
                let mode = ParameterMode::try_from(value % 10)?;
                Ok(Self::AdjustRelativeBase(mode))
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
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err(Self::Error::UnknownParameterMode),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Machine {
    memory: RefCell<Vec<i64>>,
    pub cursor: i64,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    relative_base: i64,
}

impl Machine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_memory<M: Into<Vec<i64>>>(memory: M) -> Self {
        let memory = RefCell::new(memory.into());
        Self {
            memory,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn with_memory_input<M, I>(memory: M, input: I) -> Self
    where
        M: Into<Vec<i64>>,
        I: Into<VecDeque<i64>>,
    {
        let memory = RefCell::new(memory.into());
        let input = input.into();
        Self {
            memory,
            input,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn get_memory(&self, address: i64) -> i64 {
        let len = self.memory.borrow().len();
        if len <= usize::try_from(address).unwrap() {
            self.memory
                .borrow_mut()
                .extend(repeat(0).take(1 + usize::try_from(address).unwrap() - len));
        }
        self.memory.borrow()[usize::try_from(address).unwrap()]
    }

    #[must_use]
    pub fn get_memory_mut(&mut self, address: i64) -> &mut i64 {
        let len = self.memory.borrow().len();
        if len <= usize::try_from(address).unwrap() {
            self.memory
                .borrow_mut()
                .extend(repeat(0).take(1 + usize::try_from(address).unwrap() - len));
        }
        &mut self.memory.get_mut()[usize::try_from(address).unwrap()]
    }

    #[must_use]
    fn get(&self, input: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => self.get_memory(input),
            ParameterMode::Immediate => input,
            ParameterMode::Relative => self.get_memory(self.relative_base + input),
        }
    }

    #[must_use]
    fn get_mut(&mut self, input: i64, mode: ParameterMode) -> &mut i64 {
        match mode {
            ParameterMode::Position => self.get_memory_mut(input),
            ParameterMode::Immediate => panic!("trying to write in Immediate Mode"),
            ParameterMode::Relative => self.get_memory_mut(self.relative_base + input),
        }
    }

    pub fn run(&mut self) -> Result<Return, Error> {
        while Instruction::try_from(self.get_memory(self.cursor))? != Instruction::Stop {
            let ret = self.step();
            match ret {
                Err(Error::EmptyInput) => {
                    return Ok(Return::EmptyInput);
                }
                Err(e) => {
                    return Err(e);
                }
                Ok(()) => {}
            }
        }
        Ok(Return::Stopped)
    }

    pub fn step(&mut self) -> Result<(), Error> {
        match Instruction::try_from(self.get_memory(self.cursor))? {
            Instruction::Add(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                let val_2 = self.get_mut(self.get_memory(self.cursor + 3), mode_2);
                *val_2 = val_0 + val_1;
                self.cursor += 4;
            }
            Instruction::Mul(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                let val_2 = self.get_mut(self.get_memory(self.cursor + 3), mode_2);
                *val_2 = val_0 * val_1;
                self.cursor += 4;
            }
            Instruction::Input(mode) => {
                let input = self.input.pop_front().ok_or(Error::EmptyInput)?;
                let val = self.get_mut(self.get_memory(self.cursor + 1), mode);
                *val = input;
                self.cursor += 2;
            }
            Instruction::Output(mode) => {
                let val = self.get(self.get_memory(self.cursor + 1), mode);
                self.output.push_back(val);
                self.cursor += 2;
            }
            Instruction::JumpIfTrue(mode_0, mode_1) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                if val_0 == 0 {
                    self.cursor += 3;
                } else {
                    self.cursor = val_1;
                }
            }
            Instruction::JumpIfFalse(mode_0, mode_1) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                if val_0 == 0 {
                    self.cursor = val_1;
                } else {
                    self.cursor += 3;
                }
            }
            Instruction::LessThan(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                let val_2 = self.get_mut(self.get_memory(self.cursor + 3), mode_2);
                *val_2 = i64::from(val_0 < val_1);
                self.cursor += 4;
            }
            Instruction::EqualTo(mode_0, mode_1, mode_2) => {
                let val_0 = self.get(self.get_memory(self.cursor + 1), mode_0);
                let val_1 = self.get(self.get_memory(self.cursor + 2), mode_1);
                let val_2 = self.get_mut(self.get_memory(self.cursor + 3), mode_2);
                *val_2 = i64::from(val_0 == val_1);
                self.cursor += 4;
            }
            Instruction::AdjustRelativeBase(mode) => {
                let val = self.get(self.get_memory(self.cursor + 1), mode);
                self.relative_base += val;
                self.cursor += 2;
            }
            Instruction::Stop => {}
        }
        Ok(())
    }
}
