use std::fmt::{self, Display};
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Down,
    Forward,
    Up,
}

#[derive(Debug)]
struct ParseDirectionError;

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "unparseable direction".fmt(f)
    }
}

impl std::error::Error for ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    units: usize,
}

#[derive(Debug)]
enum ParseMovementError {
    Direction(ParseDirectionError),
    Unit(ParseIntError),
    Other,
}

impl Display for ParseMovementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMovementError::Direction(e) => e.fmt(f),
            ParseMovementError::Unit(e) => e.fmt(f),
            ParseMovementError::Other => "unparseable movement".fmt(f),
        }
    }
}

impl std::error::Error for ParseMovementError {}

impl From<ParseDirectionError> for ParseMovementError {
    fn from(e: ParseDirectionError) -> Self {
        Self::Direction(e)
    }
}

impl From<ParseIntError> for ParseMovementError {
    fn from(e: ParseIntError) -> Self {
        Self::Unit(e)
    }
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");

        if let Some(direction) = parts.next() {
            if let Some(units) = parts.next() {
                let direction = direction.parse()?;
                let units = units.parse()?;

                Ok(Self { direction, units })
            } else {
                Err(ParseMovementError::Other)
            }
        } else {
            Err(ParseMovementError::Other)
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    horizontal: usize,
    vertical: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let mut position = Position::default();

    for line in stdin.lock().lines() {
        let movement = line?.parse::<Movement>()?;

        match movement.direction {
            Direction::Down => position.vertical += movement.units,
            Direction::Forward => position.horizontal += movement.units,
            Direction::Up => position.vertical -= movement.units,
        }
    }

    let answer = position.horizontal * position.vertical;
    println!("{}", answer);
    Ok(())
}
