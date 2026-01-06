use std::iter::zip;

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
    pub fn from_rows_ttb(s: &str) -> Self {
        let s = s.strip_suffix('\n').unwrap();
        let (operands, operations) = s.rsplit_once('\n').unwrap();
        let operations = operations.split_whitespace().map(Operation::new).collect();

        let nums = operands
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let num_rows = nums.len();
        let num_cols = nums[0].len();

        let operands = (0..num_cols)
            .map(|i_col| (0..num_rows).map(|i_row| nums[i_row][i_col]).collect())
            .collect();

        Self {
            operands,
            operations,
        }
    }

    #[must_use]
    pub fn from_cols_rtl(s: &str) -> Self {
        let s = s.strip_suffix('\n').unwrap();
        let (operands, operations) = s.rsplit_once('\n').unwrap();
        let operations = operations.split_whitespace().map(Operation::new).collect();

        let bytes = operands
            .lines()
            .map(|l| l.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let num_rows = bytes.len();
        let num_cols = bytes[0].len();

        let bytes = (0..num_cols)
            .map(|i_col| {
                String::from_utf8(
                    (0..num_rows)
                        .map(|i_row| bytes[i_row][i_col])
                        .collect::<Vec<_>>(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let operands = bytes
            .split(|col| col.trim().is_empty())
            .map(|group| {
                group
                    .iter()
                    .map(|col| col.trim().parse().unwrap())
                    .collect()
            })
            .collect();
        println!("{operands:?}");
        println!("{operations:?}");

        Self {
            operands,
            operations,
        }
    }

    #[must_use]
    pub fn grand_total(&self) -> u64 {
        let mut grand_total = 0;
        for (operation, operands) in zip(self.operations.iter(), self.operands.iter()) {
            grand_total += match operation {
                Operation::Add => operands.iter().sum::<u64>(),
                Operation::Multiply => operands.iter().product::<u64>(),
            };
        }
        grand_total
    }
}
