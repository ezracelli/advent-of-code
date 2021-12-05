mod coordinate;
mod floor;
mod line;

use std::io::{self, BufRead};

use self::floor::Floor;
use self::line::Line;

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<Line>);

impl Input {
    fn run(self) -> usize {
        let mut floor = Floor::default();

        for line in self.0 {
            for coordinate in line.into_iter() {
                floor[coordinate.y][coordinate.x] += 1;
            }
        }

        let mut answer = 0;
        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                if floor[i][j] >= 2 {
                    answer += 1;
                }
            }
        }

        answer
    }
}

impl<R: BufRead> TryFrom<Environment<R>> for Input {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: Environment<R>) -> Result<Self, Self::Error> {
        let input = input
            .0
            .lines()
            .map(|line| -> Result<Line, Box<dyn std::error::Error>> { Ok(line?.parse()?) })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(input))
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

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    const ANSWER: usize = 12;

    #[test]
    fn test() {
        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(input.run(), ANSWER);
    }
}
