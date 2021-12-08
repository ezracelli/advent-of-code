mod digit;
mod line;

use std::io::{self, BufRead};

use self::digit::Digit;
use self::line::Line;

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<Line>);

impl Input {
    fn run(self) -> usize {
        let valid_lengths: [usize; 4] = [
            Digit::ONE.len(),
            Digit::FOUR.len(),
            Digit::SEVEN.len(),
            Digit::EIGHT.len(),
        ];

        let mut answer = 0;

        for line in self.0 {
            for digit in line.output {
                if valid_lengths.contains(&digit.len()) {
                    answer += 1;
                }
            }
        }

        answer
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

    println!("{}", input.run());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io;

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
    const ANSWER: usize = 26;

    #[test]
    fn test() {
        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(input.run(), ANSWER);
    }
}
