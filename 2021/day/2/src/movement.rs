use std::convert::Infallible;
use std::str::FromStr;

use crate::direction::Direction;

#[derive(Debug)]
pub struct Movement {
    pub direction: Direction,
    pub units: usize,
}

impl FromStr for Movement {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        let direction = parts.next().unwrap().parse().unwrap();
        let units = parts.next().unwrap().parse().unwrap();

        Ok(Self { direction, units })
    }
}
