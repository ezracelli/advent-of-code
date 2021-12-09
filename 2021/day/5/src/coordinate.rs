use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();

        Ok(Self::new(x, y))
    }
}
