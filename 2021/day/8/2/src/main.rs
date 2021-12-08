mod digit;
mod line;
mod mapping;

use std::collections::BTreeSet;
use std::io::{self, BufRead};

use self::digit::Digit;
use self::line::Line;
use self::mapping::Mapping;

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<Line>);

impl Input {
    fn run(self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut answer = 0;

        for line in self.0 {
            let mut mapping = Mapping::builder();

            let digits = line.input.iter().chain(line.output.iter());

            let one = digits.clone().find(|digit| digit.len() == Digit::ONE.len());
            let four = digits
                .clone()
                .find(|digit| digit.len() == Digit::FOUR.len());
            let seven = digits
                .clone()
                .find(|digit| digit.len() == Digit::SEVEN.len());
            let eight = digits
                .clone()
                .find(|digit| digit.len() == Digit::EIGHT.len());

            if let Some((((one, four), seven), eight)) = one.zip(four).zip(seven).zip(eight) {
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
            } else {
                return Err("didn't find all of: 1, 4, 7, 8".into());
            }

            let mut permutations = mapping.permutations();

            for digit in digits.clone() {
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
                return Err("more than one possible answer".into());
            }

            let mapping = permutations.into_iter().next().unwrap();

            answer += line
                .output
                .into_iter()
                .map(|mut digit| {
                    digit.map(&mapping);
                    digit.sort();
                    digit.as_usize()
                })
                .rev()
                .enumerate()
                .fold(0, |acc, (i, digit)| acc + digit * 10usize.pow(i as u32));
        }

        Ok(answer)
    }
}

impl<R: BufRead> TryFrom<Environment<R>> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Environment<R>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .0
                .lines()
                .map(|line| -> Result<Line, Self::Error> { Ok(line?.parse()?) })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let environment = Environment(stdin.lock());

    let input = Input::try_from(environment)?;

    println!("{}", input.run()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    fn test_oneline() {
        const INPUT: &str =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
";
        const ANSWER: usize = 5353;

        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());
        let input = input.unwrap();

        let answer = input.run();
        assert!(answer.is_ok());
        let answer = answer.unwrap();

        assert_eq!(answer, ANSWER);
    }

    #[test]
    fn test_multiline() {
        const INPUT: &str =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
        const ANSWER: usize = 61229;

        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());
        let input = input.unwrap();

        let answer = input.run();
        assert!(answer.is_ok());
        let answer = answer.unwrap();

        assert_eq!(answer, ANSWER);
    }
}
