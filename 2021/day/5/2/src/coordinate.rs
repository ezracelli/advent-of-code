use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts
            .next()
            .ok_or(ParseCoordinateError::Other)?
            .parse()
            .map_err(ParseCoordinateError::Int)?;
        let y = parts
            .next()
            .ok_or(ParseCoordinateError::Other)?
            .parse()
            .map_err(ParseCoordinateError::Int)?;

        if parts.next().is_some() {
            return Err(ParseCoordinateError::Other);
        }

        Ok(Self::new(x, y))
    }
}

#[derive(Debug)]
pub enum ParseCoordinateError {
    Int(std::num::ParseIntError),
    Other,
}

impl Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(e) => write!(f, "invalid coordinate: {}", e),
            Self::Other => write!(f, "invalid coordinate"),
        }
    }
}

impl Error for ParseCoordinateError {}
