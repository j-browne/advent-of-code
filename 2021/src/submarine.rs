use std::str::FromStr;

pub enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

pub struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match (split.next(), split.next().map(i32::from_str)) {
            (Some("forward"), Some(Ok(x))) => Ok(Self::Forward(x)),
            (Some("up"), Some(Ok(x))) => Ok(Self::Up(x)),
            (Some("down"), Some(Ok(x))) => Ok(Self::Down(x)),
            _ => Err(ParseDirectionError),
        }
    }
}
