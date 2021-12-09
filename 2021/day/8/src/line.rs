use std::convert::Infallible;
use std::str::FromStr;

use crate::digit::Digit;

#[derive(Debug, Clone)]
pub struct Line {
    pub input: Vec<Digit>,
    pub output: Vec<Digit>,
}

impl Line {
    pub fn new(input: Vec<Digit>, output: Vec<Digit>) -> Self {
        Self { input, output }
    }
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ");

        let input = parts.next().unwrap().split(' ').map(Digit::new).collect();
        let output = parts.next().unwrap().split(' ').map(Digit::new).collect();

        Ok(Self::new(input, output))
    }
}
