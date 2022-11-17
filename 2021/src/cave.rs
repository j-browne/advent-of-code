use std::collections::HashMap;

#[derive(Debug)]
pub struct System {
    tunnels: HashMap<Cave, Vec<Cave>>,
}

impl System {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut tunnels = HashMap::new();
        for line in s.trim().split('\n') {
            let mut it = line.split('-');
            let a = Cave::new(it.next().unwrap());
            let b = Cave::new(it.next().unwrap());

            if b.name() != "start" {
                tunnels
                    .entry(a.clone())
                    .or_insert_with(Vec::new)
                    .push(b.clone());
            }
            if a.name() != "start" {
                tunnels.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
        Self { tunnels }
    }

    #[must_use]
    pub fn paths(&self) -> Vec<Vec<Cave>> {
        self.paths_inner(true)
    }

    #[must_use]
    pub fn paths_with_doubling(&self) -> Vec<Vec<Cave>> {
        self.paths_inner(false)
    }

    fn paths_inner(&self, doubled: bool) -> Vec<Vec<Cave>> {
        let mut paths = Vec::new();
        let mut curr_path = vec![Cave::new("start")];

        self.visit(&mut curr_path, &mut paths, doubled);

        paths
    }

    fn visit(&self, curr_path: &mut Vec<Cave>, paths: &mut Vec<Vec<Cave>>, doubled: bool) {
        let curr = curr_path.last().unwrap();

        if curr.name() == "end" {
            paths.push(curr_path.clone());
            return;
        }

        for c in &self.tunnels[curr] {
            if !(c.is_small() && curr_path.contains(c)) {
                curr_path.push(c.clone());
                self.visit(curr_path, paths, doubled);
                curr_path.pop().unwrap();
            } else if !doubled {
                curr_path.push(c.clone());
                self.visit(curr_path, paths, true);
                curr_path.pop().unwrap();
            }
        }
    }
}

pub fn print_path(path: &[Cave]) {
    let mut it = path.iter();
    eprint!("{}", it.next().unwrap().name());

    for c in it {
        eprint!(",{}", c.name());
    }
    eprintln!();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Cave {
    Big(String),
    Small(String),
}

impl Cave {
    #[must_use]
    pub fn new(s: &str) -> Self {
        if s.chars().all(char::is_uppercase) {
            Self::Big(s.to_string())
        } else if s.chars().all(char::is_lowercase) {
            Self::Small(s.to_string())
        } else {
            panic!("invalid cave name `{s}`")
        }
    }

    #[must_use]
    pub const fn is_big(&self) -> bool {
        matches!(self, Self::Big(_))
    }

    #[must_use]
    pub const fn is_small(&self) -> bool {
        matches!(self, Self::Small(_))
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Big(s) | Self::Small(s) => s,
        }
    }
}
