use std::error::Error;
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::digit::Digit;

#[derive(Debug)]
pub struct Line {
    pub input: Vec<Digit>,
    pub output: Vec<Digit>,
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ");

        let input = parts
            .next()
            .ok_or(LineParseError::MissingInput)?
            .split(' ')
            .map(Digit::new)
            .collect::<Vec<_>>();

        let output = parts
            .next()
            .ok_or(LineParseError::MissingOutput)?
            .split(' ')
            .map(Digit::new)
            .collect::<Vec<_>>();

        if parts.next().is_some() {
            return Err(LineParseError::ExtraCharacters);
        }

        Ok(Self { input, output })
    }
}

#[derive(Debug)]
pub enum LineParseError {
    ExtraCharacters,
    MissingInput,
    MissingOutput,
}

impl Display for LineParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtraCharacters => write!(f, "extra characters for line"),
            Self::MissingInput => write!(f, "missing input for line"),
            Self::MissingOutput => write!(f, "missing output for line"),
        }
    }
}

impl Error for LineParseError {}
