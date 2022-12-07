use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Files<'a> {
    files: HashMap<Vec<&'a str>, (Ty, u32)>,
}

impl<'a> Files<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let mut files = HashMap::new();
        files.insert(vec!["/"], (Ty::Dir, 0));
        let mut stack = vec![];

        for c in s.strip_prefix("$ ").unwrap().split("\n$ ") {
            let mut it = c.lines();
            match Command::new(it.next().unwrap()) {
                Command::Cd("..") => {
                    stack.pop();
                }
                Command::Cd(dir) => {
                    stack.push(dir);
                }
                Command::Ls => {
                    for l in it {
                        let mut it = l.split_whitespace();
                        let field1 = it.next().unwrap();
                        let name = it.next().unwrap();
                        stack.push(name);
                        if field1 == "dir" {
                            files.insert(stack.clone(), (Ty::Dir, 0));
                        } else {
                            files.insert(stack.clone(), (Ty::File, 0));
                        }
                        if let Ok(size) = field1.parse::<u32>() {
                            for i in 0..stack.len() {
                                let idx = stack.len() - i;
                                let path = &stack[..idx];
                                files.get_mut(path).unwrap().1 += size;
                            }
                        }
                        stack.pop();
                    }
                }
            }
        }

        Self { files }
    }

    #[must_use]
    pub const fn files(&self) -> &HashMap<Vec<&'a str>, (Ty, u32)> {
        &self.files
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

impl<'a> Command<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let mut it = s.split_whitespace();
        match it.next().unwrap() {
            "cd" => Command::Cd(it.next().unwrap()),
            "ls" => Command::Ls,
            c => panic!("unknown command '{c}'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    Dir,
    File,
}
