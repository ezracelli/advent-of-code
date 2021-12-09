mod board;

use std::convert::Infallible;
use std::str::FromStr;

use crate::board::Board;

#[derive(Debug)]
pub struct Input {
    boards: Vec<Board>,
    numbers: Vec<usize>,
}

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut boards = self.boards.clone();

        for number in self.numbers.iter() {
            for board in boards.iter_mut() {
                board.play(*number);

                if board.is_winner() {
                    return board.score(*number);
                }
            }
        }

        unreachable!()
    }

    fn run_part2(&self) -> Self::Answer {
        let mut boards = self.boards.clone();

        for number in self.numbers.iter() {
            let mut winning_indices = Vec::new();
            for (i, board) in boards.iter_mut().enumerate() {
                board.play(*number);

                if board.is_winner() {
                    winning_indices.push(i);
                }
            }

            if winning_indices.len() == boards.len() {
                return boards.first().unwrap().score(*number);
            }

            winning_indices.reverse();
            for i in winning_indices {
                boards.swap_remove(i);
            }
        }

        unreachable!()
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut lines = value.lines();

        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect();
        lines.next();

        let mut boards = Vec::new();
        for chunk in lines.collect::<Vec<_>>().chunks(6) {
            boards.push(
                chunk
                    .into_iter()
                    .take(5)
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
                    .parse()
                    .unwrap(),
            );
        }

        Ok(Self { boards, numbers })
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_4() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_4_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 4512;
        assert_eq!(parse_day_4().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_4_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 1924;
        assert_eq!(parse_day_4().run_part2(), ANSWER);
    }
}
