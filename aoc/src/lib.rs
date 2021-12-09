use std::convert::Infallible;
use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

pub trait Input: FromStr<Err = Infallible> {
    type Answer;

    fn run_part1(&self) -> Self::Answer;
    fn run_part2(&self) -> Self::Answer;
}

#[derive(Debug)]
pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = PartParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(PartParseError),
        }
    }
}

#[derive(Debug)]
pub struct PartParseError;

impl Display for PartParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown part")
    }
}

impl Error for PartParseError {}
