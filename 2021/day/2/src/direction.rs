use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub enum Direction {
    Down,
    Forward,
    Up,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            _ => unreachable!(),
        }
    }
}
