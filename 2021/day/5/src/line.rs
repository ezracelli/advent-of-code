use std::convert::Infallible;
use std::str::FromStr;

use crate::coordinate::Coordinate;

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
            Some(Coordinate {
                x: x.unwrap_or(self.x0),
                y: y.unwrap_or(self.y0),
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let from = parts.next().unwrap().parse().unwrap();
        let to = parts.next().unwrap().parse().unwrap();

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
