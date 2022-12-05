#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stacks {
    stacks: Vec<Vec<String>>,
    moves: Vec<Move>,
}

impl Stacks {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");

        let mut lines = it.next().unwrap().lines().rev();
        let num_stacks = (lines.next().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); num_stacks];
        for line in lines {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let idx = i * 4 + 1;
                let label = line.get(idx..=idx).unwrap();
                if label != " " {
                    stack.push(label.to_string());
                }
            }
        }

        let moves = it.next().unwrap().lines().map(Move::new).collect();

        Self { stacks, moves }
    }

    pub fn apply_moves_single(&mut self) {
        for m in self.moves.drain(..) {
            for _ in 0..(m.amount) {
                let label = self.stacks[m.src].pop().unwrap();
                self.stacks[m.dest].push(label);
            }
        }
    }

    pub fn apply_moves_multiple(&mut self) {
        for m in self.moves.drain(..) {
            let idx = self.stacks[m.src].len() - m.amount;
            let mut moving = self.stacks[m.src].split_off(idx);
            self.stacks[m.dest].append(&mut moving);
        }
    }

    #[must_use]
    pub fn tops(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap().as_str())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    amount: usize,
    src: usize,
    dest: usize,
}

impl Move {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next().unwrap(), "move");
        let amount = it.next().unwrap().parse().unwrap();
        assert_eq!(it.next().unwrap(), "from");
        let src = it.next().unwrap().parse::<usize>().unwrap() - 1;
        assert_eq!(it.next().unwrap(), "to");
        let dest = it.next().unwrap().parse::<usize>().unwrap() - 1;

        Self { amount, src, dest }
    }
}
