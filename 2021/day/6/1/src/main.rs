mod lanternfish;

use std::io::{self, BufRead};

use self::lanternfish::Lanternfish;

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<Lanternfish>);

impl Input {
    fn run(mut self) -> usize {
        for _ in 0..80 {
            for i in 0..self.0.len() {
                let lanternfish = self.0.get_mut(i).unwrap();
                if lanternfish.timer == 0 {
                    lanternfish.timer = 6;
                    self.0.push(Lanternfish::new(8));
                } else {
                    lanternfish.timer -= 1;
                }
            }
        }

        self.0.len()
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
                .map(|lanternfish| lanternfish.parse())
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

    const INPUT: &str = "3,4,3,1,2
";

    #[test]
    fn test() {
        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(input.run(), 5934);
    }
}
