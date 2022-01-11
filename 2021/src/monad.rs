//TODO: Memoize
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Monad {
    instructions: Vec<Instruction>,
}

impl Monad {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let instructions = s.trim().split('\n').map(Instruction::new).collect();

        Self { instructions }
    }

    fn is_model_number_valid(&self, model_number: &[i64; 14]) -> bool {
        self.run(model_number).z == 0
    }

    #[must_use]
    pub fn largest_valid_model_number(&self) -> u64 {
        let model_nums = ModelNumberRevIter::<14>::new();
        for m in model_nums {
            if self.is_model_number_valid(&m) {
                return m
                    .iter()
                    .rev()
                    .fold(0, |s, x| 10 * s + u64::try_from(*x).unwrap());
            }
        }

        panic!("no valid model number found")
    }

    fn run(&self, mut input: &[i64]) -> MonadState {
        let mut state = MonadState::new();

        for i in &self.instructions {
            self.step(i, &mut input, &mut state);
        }

        state
    }

    fn step(&self, i: &Instruction, input: &mut &[i64], state: &mut MonadState) {
        use Instruction::{AddN, AddR, DivN, DivR, EqlN, EqlR, Inp, ModN, ModR, MulN, MulR};
        state.inst_idx += 1;

        match *i {
            Inp(reg) => {
                state[reg] = input[0];
                *input = &input[1..];
            }
            AddN(reg, num) => state[reg] += num,
            MulN(reg, num) => state[reg] *= num,
            DivN(reg, num) => state[reg] /= num,
            ModN(reg, num) => state[reg] %= num,
            EqlN(reg, num) => state[reg] = if state[reg] == num { 1 } else { 0 },
            AddR(reg1, reg2) => state[reg1] += state[reg2],
            MulR(reg1, reg2) => state[reg1] *= state[reg2],
            DivR(reg1, reg2) => state[reg1] /= state[reg2],
            ModR(reg1, reg2) => state[reg1] %= state[reg2],
            EqlR(reg1, reg2) => state[reg1] = if state[reg1] == state[reg2] { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct MonadState {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    inst_idx: usize,
}

impl MonadState {
    #[must_use]
    fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }
}

impl Index<Register> for MonadState {
    type Output = i64;

    fn index(&self, r: Register) -> &Self::Output {
        match r {
            Register::W => &self.w,
            Register::X => &self.x,
            Register::Y => &self.y,
            Register::Z => &self.z,
        }
    }
}

impl IndexMut<Register> for MonadState {
    fn index_mut(&mut self, r: Register) -> &mut Self::Output {
        match r {
            Register::W => &mut self.w,
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Register),
    AddN(Register, i64),
    MulN(Register, i64),
    DivN(Register, i64),
    ModN(Register, i64),
    EqlN(Register, i64),
    AddR(Register, Register),
    MulR(Register, Register),
    DivR(Register, Register),
    ModR(Register, Register),
    EqlR(Register, Register),
}

impl Instruction {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.trim().split_whitespace();

        let inst = it.next().unwrap();
        let reg1 = Register::new(it.next().unwrap());

        if inst == "inp" {
            return Self::Inp(reg1);
        }

        let arg2 = it.next().unwrap();
        if let Ok(num) = arg2.parse::<i64>() {
            match inst {
                "add" => Self::AddN(reg1, num),
                "mul" => Self::MulN(reg1, num),
                "div" => Self::DivN(reg1, num),
                "mod" => Self::ModN(reg1, num),
                "eql" => Self::EqlN(reg1, num),
                x => panic!("unknown instruction `{}`", x),
            }
        } else {
            let reg2 = Register::new(arg2);
            match inst {
                "add" => Self::AddR(reg1, reg2),
                "mul" => Self::MulR(reg1, reg2),
                "div" => Self::DivR(reg1, reg2),
                "mod" => Self::ModR(reg1, reg2),
                "eql" => Self::EqlR(reg1, reg2),
                x => panic!("unknown instruction `{}`", x),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            x => panic!("unknown register `{}`", x),
        }
    }
}

#[derive(Debug, Clone)]
struct ModelNumberIter<const N: usize> {
    digits: [i64; N],
}

impl<const N: usize> ModelNumberIter<N> {
    fn new() -> Self {
        let mut digits = [1; N];
        digits[0] -= 1;

        Self { digits }
    }
}

impl<const N: usize> Default for ModelNumberIter<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Iterator for ModelNumberIter<N> {
    type Item = [i64; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut carry = true;
        for d in &mut self.digits {
            *d += 1;

            if *d == 10 {
                *d = 1;
                carry = true;
            } else {
                carry = false;
                break;
            }
        }

        if carry {
            None
        } else {
            Some(self.digits)
        }
    }
}

#[derive(Debug, Clone)]
struct ModelNumberRevIter<const N: usize> {
    digits: [i64; N],
}

impl<const N: usize> ModelNumberRevIter<N> {
    fn new() -> Self {
        let mut digits = [9; N];
        digits[0] += 1;

        Self { digits }
    }
}

impl<const N: usize> Default for ModelNumberRevIter<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Iterator for ModelNumberRevIter<N> {
    type Item = [i64; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut carry = true;
        for d in &mut self.digits {
            *d -= 1;

            if *d == 0 {
                *d = 9;
                carry = true;
            } else {
                carry = false;
                break;
            }
        }

        if carry {
            None
        } else {
            Some(self.digits)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn monad_negate() {
        let monad = Monad::new("inp x\nmul x -1");

        for i in -10..10 {
            let input = [i, 500];
            let state = monad.run(&input);
            assert_eq!(state.x, -input[0]);
            assert_ne!(state.x, 500);
            assert_ne!(state.x, -500);
        }
    }

    #[test]
    fn monad_triple() {
        let monad = Monad::new("inp z\ninp x\nmul z 3\neql z x");

        for i in -10..10 {
            let input = [i, 3 * i];
            let state = monad.run(&input);
            assert_eq!(state.z, 1);
        }

        for i in -10..10 {
            let input = [i, i];
            let state = monad.run(&input);
            assert_eq!(state.z, if i == 0 { 1 } else { 0 });
        }
    }

    #[test]
    fn monad_binary() {
        let monad = Monad::new(
            "inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2",
        );

        let input = [0];
        let state = monad.run(&input);
        assert_eq!(state.z, 0);
        assert_eq!(state.y, 0);
        assert_eq!(state.x, 0);
        assert_eq!(state.w, 0);

        let input = [5];
        let state = monad.run(&input);
        assert_eq!(state.z, 1);
        assert_eq!(state.y, 0);
        assert_eq!(state.x, 1);
        assert_eq!(state.w, 0);

        let input = [15];
        let state = monad.run(&input);
        assert_eq!(state.z, 1);
        assert_eq!(state.y, 1);
        assert_eq!(state.x, 1);
        assert_eq!(state.w, 1);
    }
}
