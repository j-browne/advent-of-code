use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Datastream<'a> {
    chars: &'a str,
}

impl<'a> Datastream<'a> {
    #[must_use]
    pub fn new(s: &'a str) -> Self {
        let chars = s.lines().next().unwrap();
        Self { chars }
    }

    #[must_use]
    pub fn start_of_packet_idx(&self) -> usize {
        self.no_repeats(4)
    }

    #[must_use]
    pub fn start_of_message_idx(&self) -> usize {
        self.no_repeats(14)
    }

    #[must_use]
    fn no_repeats(&self, n: usize) -> usize {
        let mut buffer: VecDeque<char> = VecDeque::with_capacity(n - 1);
        let mut idx = None;

        for (i, c) in self.chars.chars().enumerate() {
            if let Some(idx_found) = buffer.iter().position(|x| *x == c) {
                buffer.drain(..=idx_found);
            } else if buffer.len() == n - 1 {
                idx = Some(i);
                break;
            }
            if buffer.len() == n - 1 {
                let _ = buffer.pop_front();
            }

            buffer.push_back(c);
        }

        idx.unwrap() + 1
    }
}
