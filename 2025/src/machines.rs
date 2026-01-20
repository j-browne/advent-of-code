use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lights(Vec<bool>);

impl Lights {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(
            s.bytes()
                .map(|b| match b {
                    b'.' => false,
                    b'#' => true,
                    _ => panic!("unknown light state: {:?}", b as char),
                })
                .collect(),
        )
    }

    fn apply(&mut self, button: &Button) {
        for i in &button.0 {
            self.0[*i] = !self.0[*i];
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button(Vec<usize>);

impl Button {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(s.split(',').map(|s| s.parse().unwrap()).collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joltage(Vec<u32>);

impl Joltage {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(s.split(',').map(|s| s.parse().unwrap()).collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    light_goal: Lights,
    buttons: Vec<Button>,
    joltage: Joltage,
}

impl Machine {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut light_goal = None;
        let mut buttons = Vec::new();
        let mut joltage = None;

        for chunk in s.split_whitespace() {
            if let Some(chunk) = chunk.strip_prefix('[')
                && let Some(chunk) = chunk.strip_suffix(']')
            {
                light_goal = Some(Lights::new(chunk));
            } else if let Some(chunk) = chunk.strip_prefix('(')
                && let Some(chunk) = chunk.strip_suffix(')')
            {
                buttons.push(Button::new(chunk));
            } else if let Some(chunk) = chunk.strip_prefix('{')
                && let Some(chunk) = chunk.strip_suffix('}')
            {
                joltage = Some(Joltage::new(chunk));
            } else {
                panic!("unknown chunk format. chunk: {chunk:?}");
            }
        }

        Self {
            light_goal: light_goal.unwrap(),
            buttons,
            joltage: joltage.unwrap(),
        }
    }

    #[must_use]
    pub fn fewest_presses(&self) -> u32 {
        ButtonStateIter::new(self.buttons.len())
            .filter_map(|button_state| {
                let mut num_pressed = 0;
                let mut light_state = Lights(vec![false; self.light_goal.0.len()]);
                for (button_i, is_pushed) in button_state.into_iter().enumerate() {
                    if is_pushed {
                        light_state.apply(&self.buttons[button_i]);
                        num_pressed += 1;
                    }
                }
                (light_state == self.light_goal).then_some(num_pressed)
            })
            .min()
            .unwrap()
    }
}

struct ButtonStateIter {
    inner: Range<usize>,
}

impl ButtonStateIter {
    fn new(size: usize) -> Self {
        Self {
            inner: 0..(2usize.pow(u32::try_from(size).unwrap())),
        }
    }
}

impl Iterator for ButtonStateIter {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|mut x| {
            let mut binary = Vec::new();
            while x > 0 {
                binary.push(x % 2 != 0);
                x /= 2;
            }
            binary
        })
    }
}
