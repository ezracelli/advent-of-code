use std::io::{self, BufRead};

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<usize>);

impl Input {
    fn run(self) -> usize {
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

impl<R: BufRead> TryFrom<Environment<R>> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Environment<R>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .0
                .lines()
                .next()
                .ok_or("invalid input")??
                .split(',')
                .map(|position| position.parse())
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

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14
";
    const ANSWER: usize = 168;

    #[test]
    fn test() {
        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(input.run(), ANSWER);
    }
}
