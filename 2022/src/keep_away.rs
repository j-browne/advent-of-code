use std::{collections::BinaryHeap, mem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepAway {
    monkeys: Vec<Monkey>,
}

impl KeepAway {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let monkeys = s.split("\n\n").map(Monkey::new).collect();

        Self { monkeys }
    }

    pub fn run(&mut self, rounds: usize, with_reducing: bool) {
        let mut tmp_items = vec![];

        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                let (operation, test, if_true, if_false) = {
                    let curr = self.monkeys.get_mut(i).unwrap();
                    mem::swap(&mut curr.items, &mut tmp_items);
                    curr.inspections += tmp_items.len();

                    (curr.operation, curr.test, curr.if_true, curr.if_false)
                };

                #[allow(clippy::iter_with_drain)]
                for mut item in tmp_items.drain(..) {
                    match if apply_and_test(&mut item, operation, test, with_reducing) {
                        &if_true
                    } else {
                        &if_false
                    } {
                        Action::ThrowTo(j) => self.monkeys.get_mut(*j).unwrap().items.push(item),
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn monkey_business(&self) -> usize {
        let mut h = self
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<BinaryHeap<_>>();
        h.pop().unwrap() * h.pop().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monkey {
    inspections: usize,
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    if_true: Action,
    if_false: Action,
}

impl Monkey {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it_1 = s.lines();

        let _name = it_1.next().unwrap();

        let mut it_2 = it_1.next().unwrap().split(": ");
        assert_eq!(it_2.next().unwrap(), "  Starting items");
        let items = it_2
            .next()
            .unwrap()
            .split(", ")
            .map(|x| (x.parse().unwrap()))
            .collect();

        let mut it_2 = it_1.next().unwrap().split(": ");
        assert_eq!(it_2.next().unwrap(), "  Operation");
        let operation = Operation::new(it_2.next().unwrap());

        let mut it_2 = it_1.next().unwrap().split(": ");
        assert_eq!(it_2.next().unwrap(), "  Test");
        let test = Test::new(it_2.next().unwrap());

        let mut it_2 = it_1.next().unwrap().split(": ");
        assert_eq!(it_2.next().unwrap(), "    If true");
        let if_true = Action::new(it_2.next().unwrap());

        let mut it_2 = it_1.next().unwrap().split(": ");
        assert_eq!(it_2.next().unwrap(), "    If false");
        let if_false = Action::new(it_2.next().unwrap());

        Self {
            inspections: 0,
            items,
            operation,
            test,
            if_true,
            if_false,
        }
    }
}

#[must_use]
fn apply_and_test(item: &mut u128, operation: Operation, test: Test, with_reducing: bool) -> bool {
    *item = match operation {
        Operation::Add(other) => *item + other,
        Operation::Mul(other) => *item * other,
        Operation::MulSelf => *item * *item,
    };
    if with_reducing {
        *item /= 3;
    }

    let Test::Divisible(d) = test;
    *item % d == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(u128),
    Mul(u128),
    MulSelf,
}

impl Operation {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next().unwrap(), "new");
        assert_eq!(it.next().unwrap(), "=");
        assert_eq!(it.next().unwrap(), "old");
        match (it.next().unwrap(), it.next().unwrap()) {
            ("*", "old") => Self::MulSelf,
            ("+", s) => Self::Add(s.parse().unwrap()),
            ("*", s) => Self::Mul(s.parse().unwrap()),
            (s, _) => panic!("unknown operation '{s}'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Test {
    Divisible(u128),
}

impl Test {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next().unwrap(), "divisible");
        assert_eq!(it.next().unwrap(), "by");
        Self::Divisible(it.next().unwrap().parse().unwrap())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    ThrowTo(usize),
}

impl Action {
    #[must_use]
    fn new(s: &str) -> Self {
        let mut it = s.split_whitespace();
        assert_eq!(it.next().unwrap(), "throw");
        assert_eq!(it.next().unwrap(), "to");
        assert_eq!(it.next().unwrap(), "monkey");
        Self::ThrowTo(it.next().unwrap().parse().unwrap())
    }
}
