use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Enable,
    Disable,
    Mul(u32, u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorruptedComputer {
    instructions: Vec<Instruction>,
}

impl CorruptedComputer {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let (_, instructions) = parse_line(s).unwrap();
        Self { instructions }
    }

    #[must_use]
    pub fn sum(&self) -> u32 {
        self.instructions
            .iter()
            .fold((true, 0), |(mut enabled, mut acc), i| {
                match i {
                    Instruction::Mul(a, b) => {
                        if enabled {
                            acc += a * b;
                        }
                    }
                    Instruction::Enable => {
                        enabled = true;
                    }
                    Instruction::Disable => {
                        enabled = false;
                    }
                }
                (enabled, acc)
            })
            .1
    }

    #[must_use]
    pub fn sum_without_disable(&self) -> u32 {
        self.instructions
            .iter()
            .filter_map(|i| {
                if let Instruction::Mul(a, b) = i {
                    Some(a * b)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_junk_instr)(input)
}

fn parse_junk_instr(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, output)) = many_till(anychar, parse_instr)(input)?;
    Ok((input, output))
}

fn parse_instr(input: &str) -> IResult<&str, Instruction> {
    alt((parse_enable, parse_disable, parse_mul))(input)
}

fn parse_enable(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Enable))
}

fn parse_disable(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Disable))
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, (n1, n2)) =
        delimited(tag("("), separated_pair(digit1, tag(","), digit1), tag(")"))(input)?;
    let n1 = n1.parse().unwrap();
    let n2 = n2.parse().unwrap();
    Ok((input, Instruction::Mul(n1, n2)))
}
