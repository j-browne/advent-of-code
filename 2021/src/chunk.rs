#[derive(Debug)]
pub struct Chunk<'a> {
    data: &'a str,
}

impl<'a> Chunk<'a> {
    #[must_use]
    pub const fn new(s: &'a str) -> Self {
        Self { data: s }
    }

    pub fn status(&self) -> Result<u128, u128> {
        let mut stack = Vec::new();
        for c in self.data.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return Err(3);
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return Err(57);
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return Err(1197);
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        return Err(25137);
                    }
                }
                _ => panic!("bad character `{c}`"),
            }
        }

        let mut score = 0;
        for c in stack.iter().rev() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("bad character `{c}`"),
            }
        }

        Ok(score)
    }
}
