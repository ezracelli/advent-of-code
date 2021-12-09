mod lanternfish;

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::str::FromStr;

use crate::lanternfish::Lanternfish;

#[derive(Debug)]
pub struct Input(BTreeMap<Lanternfish, usize>);

impl Input {
    fn run(&self, generations: usize) -> <Self as aoc::Input>::Answer {
        let mut lanternfish = self.0.clone();

        for _ in 0..generations {
            let zeroes = *lanternfish.entry(Lanternfish::new(0)).or_default();

            for timer in 1..=8 {
                *lanternfish.entry(Lanternfish::new(timer - 1)).or_default() =
                    *lanternfish.entry(Lanternfish::new(timer)).or_default();
            }

            *lanternfish.entry(Lanternfish::new(6)).or_default() += zeroes;
            *lanternfish.entry(Lanternfish::new(8)).or_default() = zeroes;
        }

        lanternfish.values().sum()
    }
}

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        self.run(80)
    }

    fn run_part2(&self) -> Self::Answer {
        self.run(256)
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut lanternfish = BTreeMap::new();

        for line in value.lines() {
            for timer in line.split(',') {
                let timer = timer.parse().unwrap();
                let count = lanternfish.entry(Lanternfish { timer }).or_default();
                *count += 1;
            }
        }

        Ok(Self(lanternfish))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_6() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_6_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 5934;
        assert_eq!(parse_day_6().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_6_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 26984457539;
        assert_eq!(parse_day_6().run_part2(), ANSWER);
    }
}
