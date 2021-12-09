use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lanternfish {
    pub timer: usize,
}

impl Lanternfish {
    pub fn new(timer: usize) -> Self {
        Self { timer }
    }
}

impl FromStr for Lanternfish {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse().unwrap()))
    }
}
