mod direction;
mod movement;
mod position;

use std::convert::Infallible;
use std::str::FromStr;

use crate::direction::Direction;
use crate::movement::Movement;
use crate::position::Position;

#[derive(Debug)]
pub struct Input(Vec<Movement>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut position = Position::default();

        for movement in self.0.iter() {
            match movement.direction {
                Direction::Down => position.vertical += movement.units,
                Direction::Forward => position.horizontal += movement.units,
                Direction::Up => position.vertical -= movement.units,
            }
        }

        position.horizontal * position.vertical
    }

    fn run_part2(&self) -> Self::Answer {
        let mut position = Position::default();

        for movement in self.0.iter() {
            match movement.direction {
                Direction::Down => position.aim += movement.units,
                Direction::Forward => {
                    position.horizontal += movement.units;
                    position.vertical += position.aim * movement.units;
                }
                Direction::Up => position.aim -= movement.units,
            }
        }

        position.horizontal * position.vertical
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

    fn parse_day_2() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_2_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 150;
        assert_eq!(parse_day_2().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_2_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 900;
        assert_eq!(parse_day_2().run_part2(), ANSWER);
    }
}
