use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Lanternfish {
    pub timer: usize,
}

impl Lanternfish {
    pub fn new(timer: usize) -> Self {
        Self { timer }
    }
}

impl FromStr for Lanternfish {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self::new)
    }
}
