mod part_1;
mod part_2;

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug)]
pub struct Input {
    bit_count: usize,
    lines: Vec<String>,
}

impl aoc::Input for Input {
    type Answer = usize;

    fn run_part1(&self) -> Self::Answer {
        let gamma_rate =
            part_1::get_rating(self, |(zeroes, ones)| if zeroes > ones { '0' } else { '1' });
        let epsilon_rate =
            part_1::get_rating(self, |(zeroes, ones)| if zeroes > ones { '1' } else { '0' });

        gamma_rate * epsilon_rate
    }

    fn run_part2(&self) -> Self::Answer {
        let oxygen_generator_rating =
            part_2::get_rating(self, |(zeroes, ones)| if zeroes > ones { '0' } else { '1' });
        let co2_scrubber_rating =
            part_2::get_rating(self, |(zeroes, ones)| if zeroes > ones { '1' } else { '0' });

        oxygen_generator_rating * co2_scrubber_rating
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lines = value.lines().map(str::to_string).collect::<Vec<_>>();
        let bit_count = lines[0].len();

        Ok(Self { bit_count, lines })
    }
}

#[cfg(test)]
mod tests {
    use aoc::Input as _;

    use super::Input;

    fn parse_day_3() -> Input {
        const INPUT: &str = include_str!("../test.txt");
        INPUT.parse().unwrap()
    }

    #[test]
    fn run_day_3_part_1() {
        const ANSWER: <Input as aoc::Input>::Answer = 198;
        assert_eq!(parse_day_3().run_part1(), ANSWER);
    }

    #[test]
    fn run_day_3_part_2() {
        const ANSWER: <Input as aoc::Input>::Answer = 230;
        assert_eq!(parse_day_3().run_part2(), ANSWER);
    }
}
