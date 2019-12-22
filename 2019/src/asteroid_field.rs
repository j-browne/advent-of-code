use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    UnknownSpaceObject,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpaceObject {
    Empty,
    Asteroid,
}

impl TryFrom<char> for SpaceObject {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Asteroid),
            _ => Err(Self::Error::UnknownSpaceObject),
        }
    }
}
