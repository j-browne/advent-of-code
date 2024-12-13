use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskMap {
    blocks: Vec<(Option<usize>, usize)>,
}

impl DiskMap {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut blocks = Vec::new();
        let mut add = true;
        let mut id = 0;

        let s = s.trim();
        for length in s.bytes() {
            let length = (length - b'0') as usize;
            if add {
                blocks.push((Some(id), length));
                id += 1;
            } else {
                blocks.push((None, length));
            }
            add = !add;
        }
        Self { blocks }
    }

    pub fn defrag(&mut self) {
        let mut i = 0;
        while i < self.blocks.len() {
            while self.blocks.last().unwrap().0.is_none() {
                self.blocks.pop();
            }

            if self.blocks[i].0.is_none() {
                let left_size = self.blocks[i].1;
                let right_size = self.blocks.last().unwrap().1;
                match left_size.cmp(&right_size) {
                    Equal => {
                        self.blocks[i].0 = self.blocks.last().unwrap().0;
                        self.blocks.pop();
                    }
                    Less => {
                        self.blocks[i].0 = self.blocks.last().unwrap().0;
                        self.blocks.last_mut().unwrap().1 = right_size - left_size;
                    }
                    Greater => {
                        self.blocks[i].1 = left_size - right_size;
                        let b = self.blocks.pop().unwrap();
                        self.blocks.insert(i, b);
                    }
                }
            }
            i += 1;
        }
    }

    pub fn defrag_contiguous(&mut self) {
        let mut right_idx = self.blocks.len() - 1;
        while right_idx > 0 {
            if self.blocks[right_idx].0.is_none() {
                right_idx -= 1;
                continue;
            }

            for left_idx in 0..right_idx {
                if self.blocks[left_idx].0.is_some() {
                    continue;
                }
                let left_size = self.blocks[left_idx].1;
                let right_size = self.blocks[right_idx].1;
                match left_size.cmp(&right_size) {
                    Equal => {
                        self.blocks[left_idx].0 = self.blocks[right_idx].0;
                        self.blocks[right_idx].0 = None;
                        right_idx -= 1;
                        break;
                    }
                    Less => {}
                    Greater => {
                        self.blocks[left_idx].1 = left_size - right_size;
                        let id = self.blocks[right_idx].0;
                        self.blocks[right_idx].0 = None;
                        self.blocks.insert(left_idx, (id, right_size));
                        break;
                    }
                }
            }
            right_idx -= 1;
        }
    }

    #[must_use]
    pub fn checksum(&self) -> usize {
        let mut checksum = 0;
        let mut i = 0;
        for (id, length) in &self.blocks {
            if let Some(id) = id {
                checksum += (i..(i + length)).sum::<usize>() * id;
            }
            i += length;
        }
        checksum
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (id, length) in &self.blocks {
            for _ in 0..*length {
                if let Some(x) = id {
                    print!("{x:02} ");
                } else {
                    print!(" . ");
                }
            }
        }
        println!();
    }
}
