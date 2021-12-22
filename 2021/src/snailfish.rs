use self::{
    ExplodeState::{Done, FoundL, FoundLR, FoundR, NotFound},
    Number::{NoNumber, Pair, Single},
};
use nom::{
    character::complete::{char, digit1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{iter::Sum, ops::Add};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Number {
    NoNumber,
    Single(u32),
    Pair(Box<(Number, Number)>),
}

impl Number {
    #[must_use]
    pub fn new(s: &str) -> Self {
        parse_number(s).unwrap().1
    }

    #[must_use]
    pub fn magnitude(&self) -> u32 {
        match self {
            NoNumber => 0,
            Single(n) => *n,
            Pair(p) => 3 * p.0.magnitude() + 2 * p.1.magnitude(),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Single(a) => *a,
            _ => panic!("value must called on non-Single"),
        }
    }

    fn explode(&mut self) -> bool {
        !matches!(self.explode_inner(0), NotFound)
    }

    fn explode_inner(&mut self, depth: usize) -> ExplodeState {
        match self {
            NoNumber | Single(_) => NotFound,
            Pair(p) => {
                if depth == 4 {
                    let left = p.0.value();
                    let right = p.1.value();
                    *self = Single(0);

                    FoundLR { left, right }
                } else {
                    match p.0.explode_inner(depth + 1) {
                        NotFound => {}
                        s @ (FoundL { .. } | Done) => return s,
                        FoundLR { left, right } => {
                            p.1.add_l(right);
                            return FoundL { left };
                        }
                        FoundR { right } => {
                            p.1.add_l(right);
                            return Done;
                        }
                    }

                    match p.1.explode_inner(depth + 1) {
                        s @ (NotFound | FoundR { .. } | Done) => s,
                        FoundLR { left, right } => {
                            p.0.add_r(left);
                            FoundR { right }
                        }
                        FoundL { left } => {
                            p.0.add_r(left);
                            Done
                        }
                    }
                }
            }
        }
    }

    fn add_r(&mut self, add: u32) {
        match self {
            NoNumber => {}
            Single(n) => *n += add,
            Pair(p) => p.1.add_r(add),
        }
    }

    fn add_l(&mut self, add: u32) {
        match self {
            NoNumber => {}
            Single(n) => *n += add,
            Pair(p) => p.0.add_l(add),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            NoNumber => false,
            Single(ref n) => {
                if *n >= 10 {
                    *self = Pair(Box::new((Single(n / 2), Single(n / 2 + n % 2))));
                    true
                } else {
                    false
                }
            }
            Pair(p) => {
                // because of short-circuiting rules, p.1.split won't run if p.0.split was true
                p.0.split() || p.1.split()
            }
        }
    }

    fn reduce(&mut self) {
        // because of short-circuiting rules, split is only called if explode is false
        while self.explode() || self.split() {}
    }
}

enum ExplodeState {
    NotFound,
    FoundLR { left: u32, right: u32 },
    FoundL { left: u32 },
    FoundR { right: u32 },
    Done,
}

impl Add for &Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = match (self, rhs) {
            (NoNumber, NoNumber) => NoNumber,
            (n, NoNumber) | (NoNumber, n) => n.clone(),
            (n1, n2) => Number::Pair(Box::new((n1.clone(), n2.clone()))),
        };
        sum.reduce();
        sum
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = match (self, rhs) {
            (NoNumber, NoNumber) => NoNumber,
            (n, NoNumber) | (NoNumber, n) => n,
            (n1, n2) => Self::Pair(Box::new((n1, n2))),
        };
        sum.reduce();
        sum
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(NoNumber, |a, b| a + b)
    }
}

pub fn parse_number(s: &str) -> IResult<&str, Number> {
    if let Ok((rest, n)) = digit1::<_, ()>(s) {
        Ok((rest, Single(n.parse().unwrap())))
    } else if let Ok((rest, (n1, n2))) = delimited(
        char('['),
        separated_pair(parse_number, char(','), parse_number),
        char(']'),
    )(s)
    {
        Ok((rest, Pair(Box::new((n1, n2)))))
    } else {
        panic!("failed to parse Number")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn magnitude() {
        let n = Number::new("[9,1]");
        assert_eq!(n.magnitude(), 29);

        let n = Number::new("[1,9]");
        assert_eq!(n.magnitude(), 21);

        let n = Number::new("[[9,1],[1,9]]");
        assert_eq!(n.magnitude(), 129);

        let n = Number::new("[[1,2],[[3,4],5]]");
        assert_eq!(n.magnitude(), 143);

        let n = Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(n.magnitude(), 1384);

        let n = Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(n.magnitude(), 445);

        let n = Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(n.magnitude(), 791);

        let n = Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(n.magnitude(), 1137);

        let n = Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(n.magnitude(), 3488);
    }

    #[test]
    fn explode() {
        let mut n1 = Number::new("[[[[[9,8],1],2],3],4]");
        n1.explode();
        let n2 = Number::new("[[[[0,9],2],3],4]");
        assert_eq!(n1, n2);

        let mut n1 = Number::new("[7,[6,[5,[4,[3,2]]]]]");
        n1.explode();
        let n2 = Number::new("[7,[6,[5,[7,0]]]]");
        assert_eq!(n1, n2);

        let mut n1 = Number::new("[[6,[5,[4,[3,2]]]],1]");
        n1.explode();
        let n2 = Number::new("[[6,[5,[7,0]]],3]");
        assert_eq!(n1, n2);

        let mut n1 = Number::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n1.explode();
        let n2 = Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(n1, n2);

        let mut n1 = Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        n1.explode();
        let n2 = Number::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        assert_eq!(n1, n2);
    }

    #[test]
    fn add() {
        let n1 = Number::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Number::new("[1,1]");
        let sum = Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(n1 + n2, sum);
    }

    #[test]
    fn sum_list() {
        let left = ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
            .into_iter()
            .map(Number::new)
            .sum::<Number>();
        let right = Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(left, right);

        let left = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]
            .into_iter()
            .map(Number::new)
            .sum::<Number>();
        let right = Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(left, right);

        let left = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]
            .into_iter()
            .map(Number::new)
            .sum::<Number>();
        let right = Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(left, right);

        let left = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ]
        .into_iter()
        .map(Number::new)
        .sum::<Number>();
        let right = Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(left, right);
    }
}
