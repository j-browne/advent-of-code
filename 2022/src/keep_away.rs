use std::{
    collections::{BinaryHeap, HashMap},
    mem,
    ops::{AddAssign, DivAssign, MulAssign, Rem},
};

#[derive(Debug, Clone)]
pub struct KeepAway {
    monkeys: Vec<Monkey>,
    with_reducing: bool,
}

impl KeepAway {
    #[must_use]
    pub fn new(s: &str, with_reducing: bool) -> Self {
        let mut monkeys = s.split("\n\n").map(Monkey::new).collect::<Vec<_>>();

        if !with_reducing {
            let divisors = monkeys
                .iter()
                .map(|m| {
                    let Test::Divisible(d) = m.test;
                    d
                })
                .collect::<Vec<_>>();

            for m in &mut monkeys {
                for item in &mut m.items {
                    item.to_mods(&divisors);
                }
            }
        };

        Self {
            monkeys,
            with_reducing,
        }
    }

    pub fn run(&mut self, rounds: usize) {
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
                    match if apply_and_test(&mut item, operation, test, self.with_reducing) {
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

#[derive(Debug, Clone)]
pub struct Monkey {
    inspections: usize,
    items: Vec<Item>,
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
        let items = it_2.next().unwrap().split(", ").map(Item::new).collect();

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
fn apply_and_test(item: &mut Item, operation: Operation, test: Test, with_reducing: bool) -> bool {
    match operation {
        Operation::Add(other) => *item += other,
        Operation::Mul(other) => *item *= other,
        Operation::MulSelf => item.mul_self(),
    };

    if with_reducing {
        *item /= 3;
    }

    let Test::Divisible(d) = test;
    &*item % d == 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(u32),
    Mul(u32),
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
    Divisible(u32),
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

#[derive(Debug, Clone)]
enum Item {
    Single(u32),
    Mods(HashMap<u32, u32>),
}

impl Item {
    #[must_use]
    fn new(s: &str) -> Self {
        let worry = s.parse().unwrap();

        Self::Single(worry)
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_mods(&mut self, divisors: &[u32]) {
        if let Self::Single(initial) = self {
            *self = Self::Mods(divisors.iter().map(|d| (*d, *initial)).collect());
        } else {
            panic!("cannot convert to Item::Mods if alread Item::Mods");
        }
    }

    fn mul_self(&mut self) {
        match self {
            Self::Single(w) => {
                *w *= *w;
            }
            Self::Mods(ws) => {
                for (d, w) in ws.iter_mut() {
                    *w *= *w;
                    *w %= d;
                }
            }
        }
    }
}

impl AddAssign<u32> for Item {
    fn add_assign(&mut self, rhs: u32) {
        match self {
            Self::Single(w) => {
                *w += rhs;
            }
            Self::Mods(ws) => {
                for (d, w) in ws.iter_mut() {
                    *w += rhs;
                    *w %= d;
                }
            }
        }
    }
}

impl MulAssign<u32> for Item {
    fn mul_assign(&mut self, rhs: u32) {
        match self {
            Self::Single(w) => {
                *w *= rhs;
            }
            Self::Mods(ws) => {
                for (d, w) in ws.iter_mut() {
                    *w *= rhs;
                    *w %= d;
                }
            }
        }
    }
}

impl DivAssign<u32> for Item {
    fn div_assign(&mut self, rhs: u32) {
        match self {
            Self::Single(w) => {
                *w /= rhs;
            }
            Self::Mods(_ws) => {
                panic!("cannot divide Item::Mods");
            }
        }
    }
}

impl Rem<u32> for &Item {
    type Output = u32;

    fn rem(self, rhs: u32) -> Self::Output {
        match self {
            Item::Single(w) => w % rhs,
            Item::Mods(ws) => ws[&rhs],
        }
    }
}
