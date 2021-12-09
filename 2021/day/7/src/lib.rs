use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input(Vec<usize>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let max = *self.0.iter().max_by_key(|position| *position).unwrap();
        let min = *self.0.iter().min_by_key(|position| *position).unwrap();

        (min..=max)
            .into_iter()
            .map(|position| -> usize {
                self.0
                    .iter()
                    .map(|p| std::cmp::max(position, *p) - std::cmp::min(position, *p))
                    .sum()
            })
            .min()
            .unwrap()
    }

    fn run_part2(&self) -> Self::Answer {
        let max = *self.0.iter().max_by_key(|position| *position).unwrap();
        let min = *self.0.iter().min_by_key(|position| *position).unwrap();

        (min..=max)
            .into_iter()
            .map(|position| -> usize {
                self.0
                    .iter()
                    .map(|p| -> usize {
                        // https://en.wikipedia.org/wiki/Faulhaber's_formula
                        let n = std::cmp::max(position, *p) - std::cmp::min(position, *p);
                        (n * (n + 1)) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap()
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            value
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(|line| line.parse().unwrap())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_7() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_7_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 37;
        assert_eq!(parse_day_7().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_7_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 168;
        assert_eq!(parse_day_7().run_part2(), ANSWER);
    }
}
