mod ast;

use std::convert::Infallible;
use std::str::FromStr;

use nom::error::{VerboseError, VerboseErrorKind};

use crate::ast::parsers;

#[derive(Debug)]
pub struct Input(Vec<String>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut answer = 0;

        for line in self.0.iter() {
            match parsers::chunks::<VerboseError<_>>(&**line) {
                Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                    answer += match e.errors.get(0).and_then(|e| e.0.chars().next()) {
                        Some(')') => 3,
                        Some(']') => 57,
                        Some('}') => 1197,
                        Some('>') => 25137,
                        _ => 0,
                    };
                }
                _ => {}
            }
        }

        answer
    }

    fn run_part2(&self) -> Self::Answer {
        let mut scores = Vec::new();

        for line in self.0.iter() {
            match parsers::chunks::<VerboseError<_>>(&**line) {
                Err(nom::Err::Error(e) | nom::Err::Failure(e)) => match e.errors.get(0) {
                    Some((input, _)) if input.len() == 0 => {
                        let score = e
                            .errors
                            .iter()
                            .skip(1)
                            .map(|(_, kind)| match kind {
                                VerboseErrorKind::Context(ctx) => ctx,
                                _ => unreachable!(),
                            })
                            .map(|ctx| match ctx {
                                &parsers::contexts::ANGLE_BRACKET => 4,
                                &parsers::contexts::CURLY_BRACKET => 3,
                                &parsers::contexts::PAREN => 1,
                                &parsers::contexts::SQUARE_BRACKET => 2,
                                _ => unreachable!(),
                            })
                            .fold(0, |acc, curr| acc * 5 + curr);

                        scores.push(score);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        scores.sort();
        scores[scores.len() / 2]
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.lines().map(str::to_string).collect()))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_10() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_10_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 26397;
        assert_eq!(parse_day_10().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_10_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 288957;
        assert_eq!(parse_day_10().run_part2(), ANSWER);
    }
}
