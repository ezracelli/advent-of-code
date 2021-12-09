mod digit;
mod line;
mod mapping;

use std::collections::BTreeSet;
use std::convert::Infallible;
use std::str::FromStr;

use self::digit::Digit;
use self::line::Line;
use self::mapping::Mapping;

#[derive(Debug)]
pub struct Input(Vec<Line>);

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let valid_lengths: [usize; 4] = [
            Digit::ONE.len(),
            Digit::FOUR.len(),
            Digit::SEVEN.len(),
            Digit::EIGHT.len(),
        ];

        let mut answer = 0;

        for line in self.0.iter() {
            for digit in line.output.iter() {
                if valid_lengths.contains(&digit.len()) {
                    answer += 1;
                }
            }
        }

        answer
    }

    fn run_part2(&self) -> Self::Answer {
        let mut answer = 0;

        for line in self.0.iter() {
            let mut mapping = Mapping::builder();

            {
                let one = line
                    .input
                    .iter()
                    .find(|digit| digit.len() == Digit::ONE.len())
                    .unwrap();
                let four = line
                    .input
                    .iter()
                    .find(|digit| digit.len() == Digit::FOUR.len())
                    .unwrap();
                let seven = line
                    .input
                    .iter()
                    .find(|digit| digit.len() == Digit::SEVEN.len())
                    .unwrap();
                let eight = line
                    .input
                    .iter()
                    .find(|digit| digit.len() == Digit::EIGHT.len())
                    .unwrap();

                let cf = one.chars().collect::<BTreeSet<_>>();
                mapping.insert('c', cf.clone());
                mapping.insert('f', cf);

                let a = seven
                    .chars()
                    .filter(|char| !mapping.contains(char))
                    .collect::<BTreeSet<_>>();
                mapping.insert('a', a);

                let bd = four
                    .chars()
                    .filter(|char| !mapping.contains(char))
                    .collect::<BTreeSet<_>>();
                mapping.insert('b', bd.clone());
                mapping.insert('d', bd);

                let eg = eight
                    .chars()
                    .filter(|char| !mapping.contains(char))
                    .collect::<BTreeSet<_>>();
                mapping.insert('e', eg.clone());
                mapping.insert('g', eg);
            }

            let mut permutations = mapping.permutations();

            for digit in line.input.iter() {
                if permutations.len() == 1 {
                    break;
                }

                permutations = permutations
                    .into_iter()
                    .filter(|mapping| {
                        let mut digit = digit.clone();
                        digit.map(&mapping);
                        digit.sort();
                        digit.is_valid()
                    })
                    .collect();
            }

            if permutations.len() != 1 {
                panic!("more than one possible answer");
            }

            let mapping = permutations.into_iter().next().unwrap();

            answer += line
                .output
                .iter()
                .map(|digit| {
                    let mut digit = digit.clone();

                    digit.map(&mapping);
                    digit.sort();

                    digit.as_usize()
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (i, digit)| acc + digit * 10usize.pow(i as u32));
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

    fn parse_1_day_8() -> Input {
        const INPUT: &str = include_str!("../test.1.txt");
        INPUT.parse().unwrap()
    }

    fn parse_2_day_8() -> Input {
        const INPUT: &str = include_str!("../test.2.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_1_day_8_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 0;
        assert_eq!(parse_1_day_8().run_part1(), ANSWER);
    }

    #[test]
    fn run_1_day_8_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 5353;
        assert_eq!(parse_1_day_8().run_part2(), ANSWER);
    }

    #[test]
    fn run_2_day_8_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 26;
        assert_eq!(parse_2_day_8().run_part1(), ANSWER);
    }

    #[test]
    fn run_2_day_1_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 61229;
        assert_eq!(parse_2_day_8().run_part2(), ANSWER);
    }
}
