use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use super::coordinate::{Coordinate, ParseCoordinateError};

pub struct Iter {
    x: Box<dyn Iterator<Item = usize>>,
    x0: usize,
    y: Box<dyn Iterator<Item = usize>>,
    y0: usize,
}

impl Iterator for Iter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x.next();
        let y = self.y.next();

        if x.is_none() && y.is_none() {
            None
        } else {
            Some(Coordinate::new(x.unwrap_or(self.x0), y.unwrap_or(self.y0)))
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Line {
    pub fn new(from: Coordinate, to: Coordinate) -> Self {
        Self { from, to }
    }
}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let from = parts
            .next()
            .ok_or(ParseLineError::Other)?
            .parse()
            .map_err(ParseLineError::Coordinate)?;
        let to = parts
            .next()
            .ok_or(ParseLineError::Other)?
            .parse()
            .map_err(ParseLineError::Coordinate)?;

        if parts.next().is_some() {
            return Err(ParseLineError::Other);
        }

        Ok(Self::new(from, to))
    }
}

impl IntoIterator for Line {
    type Item = Coordinate;
    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let x = if self.from.x > self.to.x {
            Box::new((self.to.x..=self.from.x).rev()) as Box<dyn Iterator<Item = usize>>
        } else {
            Box::new(self.from.x..=self.to.x)
        };

        let y = if self.from.y > self.to.y {
            Box::new((self.to.y..=self.from.y).rev()) as Box<dyn Iterator<Item = usize>>
        } else {
            Box::new(self.from.y..=self.to.y)
        };

        Iter {
            x,
            x0: self.from.x,
            y,
            y0: self.from.y,
        }
    }
}

#[derive(Debug)]
pub enum ParseLineError {
    Coordinate(ParseCoordinateError),
    Other,
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(e) => write!(f, "invalid line: {}", e),
            Self::Other => write!(f, "invalid line"),
        }
    }
}

impl Error for ParseLineError {}
