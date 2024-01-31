use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
enum Key {
    X = 0,
    M,
    A,
    S,
}

impl Key {
    #[must_use]
    fn new(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("unknown key: {s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Condition {
    Lt(Key, u32),
    Gt(Key, u32),
    Always,
}

impl Condition {
    #[must_use]
    fn new(s: &str) -> Self {
        if let Some((key, value)) = s.split_once('<') {
            Condition::Lt(Key::new(key), value.parse().unwrap())
        } else if let Some((key, value)) = s.split_once('>') {
            Condition::Gt(Key::new(key), value.parse().unwrap())
        } else {
            panic!("unknown condition: {s}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Workflow<'a> {
    rules: Vec<(Condition, &'a str)>,
}

impl<'a> Workflow<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> (&'a str, Self) {
        let mut it = s.split('{');
        let key = it.next().unwrap();
        let rules = it
            .next()
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|x| {
                if let Some((cond, next)) = x.split_once(':') {
                    (Condition::new(cond), next)
                } else {
                    (Condition::Always, x)
                }
            })
            .collect();
        (key, Self { rules })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Part {
    ratings: [u32; 4],
}

impl Part {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut ratings = [None; 4];

        for rating in s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let (key, value) = rating.split_once('=').unwrap();
            ratings[Key::new(key) as usize] = Some(value.parse().unwrap());
        }

        let ratings = ratings
            .into_iter()
            .map(Option::unwrap)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { ratings }
    }

    #[must_use]
    pub fn accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut curr = "in";
        'outer: loop {
            if curr == "A" {
                return true;
            }
            if curr == "R" {
                return false;
            }

            let w = &workflows[curr];
            for rule in &w.rules {
                match rule {
                    (Condition::Lt(k, v), next) => {
                        if self.ratings[*k as usize] < *v {
                            curr = next;
                            continue 'outer;
                        }
                    }
                    (Condition::Gt(k, v), next) => {
                        if self.ratings[*k as usize] > *v {
                            curr = next;
                            continue 'outer;
                        }
                    }
                    (Condition::Always, next) => {
                        curr = next;
                        continue 'outer;
                    }
                }
            }
            unreachable!();
        }
    }

    #[must_use]
    pub fn sum(&self) -> u32 {
        self.ratings.iter().sum()
    }
}

#[must_use]
pub fn accepted_combination_count(workflows: &Vec<(&str, Workflow)>) -> u64 {
    todo!()
}
