use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Input(Vec<Vec<usize>>);

impl Input {
    fn flash(&mut self, x: usize, y: usize) -> usize {
        let mut count = 1;

        for y_ in y.checked_sub(1).unwrap_or(y)..=std::cmp::min(y + 1, self.len() - 1) {
            for x_ in x.checked_sub(1).unwrap_or(x)..=std::cmp::min(x + 1, self[y].len() - 1) {
                if self[y_][x_] > 0 {
                    self[y_][x_] += 1;

                    if self[y_][x_] > 9 {
                        self[y_][x_] = 0;
                        count += self.flash(x_, y_);
                    }
                }
            }
        }

        count
    }
}

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut count = 0;
        let mut input = self.clone();

        for _ in 0..100 {
            for y in 0..input.len() {
                for x in 0..input[y].len() {
                    input[y][x] += 1;
                }
            }

            for y in 0..input.len() {
                for x in 0..input[y].len() {
                    if input[y][x] > 9 {
                        input[y][x] = 0;
                        count += input.flash(x, y);
                    }
                }
            }
        }

        count
    }

    fn run_part2(&self) -> Self::Answer {
        let mut input = self.clone();

        for generation in 0..usize::MAX {
            for y in 0..input.len() {
                for x in 0..input[y].len() {
                    input[y][x] += 1;
                }
            }

            for y in 0..input.len() {
                for x in 0..input[y].len() {
                    if input[y][x] > 9 {
                        input[y][x] = 0;
                        input.flash(x, y);
                    }
                }
            }

            let mut all_flashed = true;

            'all_flashed: for y in 0..input.len() {
                for x in 0..input[y].len() {
                    if input[y][x] != 0 {
                        all_flashed = false;
                        break 'all_flashed;
                    }
                }
            }

            if all_flashed {
                return generation + 1;
            }
        }

        unreachable!()
    }
}

impl Deref for Input {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|char| (char.to_digit(10).unwrap() as usize))
                        .collect()
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_1_day_11() -> Input {
        const INPUT: &str = include_str!("../test.1.txt");
        INPUT.parse().unwrap()
    }

    fn parse_2_day_11() -> Input {
        const INPUT: &str = include_str!("../test.2.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_1_day_11_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 259;
        assert_eq!(parse_1_day_11().run_part1(), ANSWER);
    }

    #[test]
    fn run_2_day_11_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 1656;
        assert_eq!(parse_2_day_11().run_part1(), ANSWER);
    }

    #[test]
    fn run_1_day_11_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 6;
        assert_eq!(parse_1_day_11().run_part2(), ANSWER);
    }

    #[test]
    fn run_2_day_11_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 195;
        assert_eq!(parse_2_day_11().run_part2(), ANSWER);
    }
}
