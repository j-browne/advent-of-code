use num::Integer;
use std::collections::HashMap;

pub struct DesertMap {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, Node>,
}

impl DesertMap {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split("\n\n");
        let instructions = it.next().unwrap().chars().map(Instruction::new).collect();
        let nodes = it.next().unwrap().lines().map(Node::new).collect();
        Self {
            instructions,
            nodes,
        }
    }

    #[must_use]
    pub fn steps(&self) -> u64 {
        let src = "AAA";
        let dest = "ZZZ";

        let mut curr = (src, &self.nodes[src]);
        let mut num_steps = 0;

        for inst in self.instructions.iter().cycle() {
            if curr.0 == dest {
                break;
            }
            match inst {
                Instruction::Left => {
                    curr.0 = &curr.1.left;
                }
                Instruction::Right => {
                    curr.0 = &curr.1.right;
                }
            }

            curr.1 = &self.nodes[curr.0];
            num_steps += 1;
        }

        num_steps
    }

    #[must_use]
    pub fn ghost_steps(&self) -> u64 {
        let mut curr = self
            .nodes
            .iter()
            .filter(|(k, _v)| k.ends_with('A'))
            .collect::<Vec<_>>();
        let mut zs = curr.iter().map(|_| (None, None)).collect::<Vec<_>>();

        for (i, inst) in self.instructions.iter().cycle().enumerate() {
            if zs.iter().all(|(x1, x2)| x1.is_some() && x2.is_some()) {
                break;
            }

            for (j, (k, v)) in curr.iter_mut().enumerate() {
                if k.ends_with('Z') {
                    if zs[j].0.is_none() {
                        zs[j].0 = Some(i);
                    } else if zs[j].1.is_none() {
                        zs[j].1 = Some(i);
                    }
                }

                match inst {
                    Instruction::Left => {
                        *k = &v.left;
                    }
                    Instruction::Right => {
                        *k = &v.right;
                    }
                }
                *v = &self.nodes[*k];
            }
        }

        // This is all under the assumption that the length to get to an exit is the same as the
        // length to get to get to the next exit and that that is a loop. These assumptions aren't
        // really justified by the problem text, but they are true for the inputs given.
        let mut lcm = 1;
        for (x1, x2) in zs {
            assert_eq!(x1.unwrap() * 2, x2.unwrap());
            lcm = u64::lcm(&lcm, &(x1.unwrap() as u64));
        }

        lcm
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    #[must_use]
    pub fn new(s: &str) -> (String, Self) {
        let mut it = s.split(" = ");
        let label = it.next().unwrap().to_string();
        let mut it = it
            .next()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ");
        let node = Node {
            left: it.next().unwrap().to_string(),
            right: it.next().unwrap().to_string(),
        };
        (label, node)
    }
}

enum Instruction {
    Left,
    Right,
}

impl Instruction {
    #[must_use]
    fn new(c: char) -> Self {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("unknown instruction"),
        }
    }
}
