mod lanternfish;

use std::collections::HashMap;
use std::io::{self, BufRead};

use self::lanternfish::Lanternfish;

struct Environment<R>(R);

#[derive(Debug)]
struct Input {
    lanternfish: HashMap<Lanternfish, usize>,
}

impl Input {
    fn run(mut self) -> usize {
        for _ in 0..256 {
            let zeroes = *self.lanternfish.entry(Lanternfish::new(0)).or_default();

            for timer in 1..=8 {
                *self
                    .lanternfish
                    .entry(Lanternfish::new(timer - 1))
                    .or_default() = *self.lanternfish.entry(Lanternfish::new(timer)).or_default();
            }

            *self.lanternfish.entry(Lanternfish::new(6)).or_default() += zeroes;
            *self.lanternfish.entry(Lanternfish::new(8)).or_default() = zeroes;
        }

        self.lanternfish.values().sum()
    }
}

impl<R: BufRead> TryFrom<Environment<R>> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Environment<R>) -> Result<Self, Self::Error> {
        let mut lanternfish = HashMap::new();

        for line in value.0.lines() {
            for timer in line?.split(',') {
                let timer = timer.parse()?;
                let count = lanternfish.entry(Lanternfish::new(timer)).or_default();
                *count += 1;
            }
        }

        Ok(Self { lanternfish })
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

        assert_eq!(input.run(), 26984457539);
    }
}
