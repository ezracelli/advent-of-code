mod coordinate;
mod floor;
mod line;

use std::convert::Infallible;
use std::str::FromStr;

use crate::floor::Floor;
use crate::line::Line;

#[derive(Debug)]
pub struct Input(Vec<Line>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut floor = Floor::default();

        for line in self
            .0
            .iter()
            .filter(|line| line.from.x == line.to.x || line.from.y == line.to.y)
        {
            for x in if line.from.x <= line.to.x {
                line.from.x..=line.to.x
            } else {
                line.to.x..=line.from.x
            } {
                for y in if line.from.y <= line.to.y {
                    line.from.y..=line.to.y
                } else {
                    line.to.y..=line.from.y
                } {
                    floor[y][x] += 1;
                }
            }
        }

        let mut answer = 0;
        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                if floor[i][j] >= 2 {
                    answer += 1;
                }
            }
        }

        answer
    }

    fn run_part2(&self) -> Self::Answer {
        let mut floor = Floor::default();

        for line in self.0.iter() {
            for coordinate in line.into_iter() {
                floor[coordinate.y][coordinate.x] += 1;
            }
        }

        let mut answer = 0;
        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                if floor[i][j] >= 2 {
                    answer += 1;
                }
            }
        }

        answer
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value.lines().map(|line| line.parse().unwrap()).collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_5() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_5_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 5;
        assert_eq!(parse_day_5().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_5_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 12;
        assert_eq!(parse_day_5().run_part2(), ANSWER);
    }
}
