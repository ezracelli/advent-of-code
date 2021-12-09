use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input(Vec<Vec<usize>>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let mut answer = 0;

        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                let point = self.0[i][j];

                let above = i
                    .checked_sub(1)
                    .and_then(|i| self.0.get(i).map(|row| row[j]))
                    .unwrap_or(usize::MAX);
                let below = self.0.get(i + 1).map(|row| row[j]).unwrap_or(usize::MAX);
                let left = j
                    .checked_sub(1)
                    .and_then(|j| self.0[i].get(j))
                    .copied()
                    .unwrap_or(usize::MAX);
                let right = self.0[i].get(j + 1).copied().unwrap_or(usize::MAX);

                if point < above && point < below && point < left && point < right {
                    answer += point + 1;
                }
            }
        }

        answer
    }

    fn run_part2(&self) -> Self::Answer {
        todo!()
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
                        .map(|char| char.to_string().parse().unwrap())
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

    fn parse_day_9() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_9_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 15;
        assert_eq!(parse_day_9().run_part1(), ANSWER);
    }

    // #[ignore]
    // #[test]
    // fn run_day_9_part_2() {
    //     const ANSWER: <Input as aoc::Input>::Answer = todo!();
    //     assert_eq!(parse_day_9().run_part2(), ANSWER);
    // }
}
