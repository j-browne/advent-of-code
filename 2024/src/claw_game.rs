#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClawGame {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl ClawGame {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.lines();
        let button_a = Button::new(it.next().unwrap());
        let button_b = Button::new(it.next().unwrap());
        let prize = Prize::new(it.next().unwrap());
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    #[must_use]
    pub fn new_corrected(s: &str) -> Self {
        let mut it = s.lines();
        let button_a = Button::new(it.next().unwrap());
        let button_b = Button::new(it.next().unwrap());
        let prize = Prize::new_corrected(it.next().unwrap());
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    #[must_use]
    pub fn cost_to_win(&self) -> Option<i64> {
        let a = &self.button_a;
        let b = &self.button_b;
        let z = &self.prize;

        let numer_a = b.y * z.x - b.x * z.y;
        let numer_b = a.x * z.y - a.y * z.x;
        let denom = a.x * b.y - a.y * b.x;

        let n_a = (numer_a % denom == 0).then_some(numer_a / denom)?;
        let n_b = (numer_b % denom == 0).then_some(numer_b / denom)?;

        Some(3 * n_a + n_b)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    x: i64,
    y: i64,
}

impl Button {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split(": ").nth(1).unwrap().split(", ");
        let x = it
            .next()
            .unwrap()
            .strip_prefix("X+")
            .unwrap()
            .parse()
            .unwrap();
        let y = it
            .next()
            .unwrap()
            .strip_prefix("Y+")
            .unwrap()
            .parse()
            .unwrap();
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prize {
    x: i64,
    y: i64,
}

impl Prize {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let mut it = s.split(": ").nth(1).unwrap().split(", ");
        let x = it
            .next()
            .unwrap()
            .strip_prefix("X=")
            .unwrap()
            .parse()
            .unwrap();
        let y = it
            .next()
            .unwrap()
            .strip_prefix("Y=")
            .unwrap()
            .parse()
            .unwrap();
        Self { x, y }
    }

    #[must_use]
    pub fn new_corrected(s: &str) -> Self {
        let mut it = s.split(": ").nth(1).unwrap().split(", ");
        let x = it
            .next()
            .unwrap()
            .strip_prefix("X=")
            .unwrap()
            .parse::<i64>()
            .unwrap()
            + 10_000_000_000_000;
        let y = it
            .next()
            .unwrap()
            .strip_prefix("Y=")
            .unwrap()
            .parse::<i64>()
            .unwrap()
            + 10_000_000_000_000;
        Self { x, y }
    }
}
