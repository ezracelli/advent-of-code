use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input(Vec<usize>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut answer = 0usize;
        let mut prev: Option<usize> = None;

        for curr in self.0.iter() {
            if let Some(prev) = prev.as_mut() {
                if curr > prev {
                    answer += 1;
                }

                *prev = *curr;
            } else {
                prev = Some(*curr);
            }
        }

        answer
    }

    fn run_part2(&self) -> Self::Answer {
        const WINDOW_SIZE: usize = 3;

        let windows = self.0.windows(WINDOW_SIZE);
        let windowed_sums = windows.map(|window| window.iter().sum::<usize>());

        let mut answer = 0usize;
        let mut prev: Option<usize> = None;

        for curr in windowed_sums {
            if let Some(prev) = prev.as_mut() {
                if curr > *prev {
                    answer += 1;
                }

                *prev = curr;
            } else {
                prev = Some(curr);
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

    fn parse_day_1() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_1_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 7;
        assert_eq!(parse_day_1().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_1_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 5;
        assert_eq!(parse_day_1().run_part2(), ANSWER);
    }
}
