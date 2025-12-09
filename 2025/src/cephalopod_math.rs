#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    #[must_use]
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => panic!("unknown operation"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathProblems {
    operands: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl MathProblems {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let lines = s.lines();
        let num_lines = lines.clone().count();
        let operands = lines
            .clone()
            .take(num_lines - 1)
            .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
            .collect();
        let operations = lines
            .skip(num_lines - 1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(Operation::new)
            .collect();
        Self {
            operands,
            operations,
        }
    }

    #[must_use]
    pub fn grand_total(&self) -> u64 {
        let mut grand_total = 0;
        for (operation_idx, operation) in self.operations.iter().enumerate() {
            let mut total = match operation {
                Operation::Add => 0,
                Operation::Multiply => 1,
            };
            for operand_list in &self.operands {
                match operation {
                    Operation::Add => total += operand_list[operation_idx],
                    Operation::Multiply => total *= operand_list[operation_idx],
                };
            }
            grand_total += total
        }
        grand_total
    }
}
