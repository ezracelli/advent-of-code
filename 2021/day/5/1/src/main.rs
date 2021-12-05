use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum ParseCoordinateError {
    Int(std::num::ParseIntError),
    Other,
}

impl Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(e) => write!(f, "invalid coordinate: {}", e),
            Self::Other => write!(f, "invalid coordinate"),
        }
    }
}

impl std::error::Error for ParseCoordinateError {}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts
            .next()
            .ok_or(ParseCoordinateError::Other)?
            .parse()
            .map_err(ParseCoordinateError::Int)?;
        let y = parts
            .next()
            .ok_or(ParseCoordinateError::Other)?
            .parse()
            .map_err(ParseCoordinateError::Int)?;

        if parts.next().is_some() {
            return Err(ParseCoordinateError::Other);
        }

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
struct Line {
    from: Coordinate,
    to: Coordinate,
}

#[derive(Debug)]
enum ParseLineError {
    Coordinate(ParseCoordinateError),
    Other,
}

impl Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(e) => write!(f, "invalid line: {}", e),
            Self::Other => write!(f, "invalid line"),
        }
    }
}

impl std::error::Error for ParseLineError {}

impl FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let from = parts
            .next()
            .ok_or(ParseLineError::Other)?
            .parse()
            .map_err(ParseLineError::Coordinate)?;
        let to = parts
            .next()
            .ok_or(ParseLineError::Other)?
            .parse()
            .map_err(ParseLineError::Coordinate)?;

        if parts.next().is_some() {
            return Err(ParseLineError::Other);
        }

        Ok(Self { from, to })
    }
}

struct Floor(Vec<Vec<usize>>);

impl Default for Floor {
    fn default() -> Self {
        if cfg!(test) {
            Self(vec![vec![0; 10]; 10])
        } else {
            Self(vec![vec![0; 1024]; 1024])
        }
    }
}

impl Debug for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                match self.0[i][j] {
                    0 => write!(f, ".")?,
                    n => write!(f, "{}", n)?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

struct Environment<R>(R);

#[derive(Debug)]
struct Input(Vec<Line>);

impl Input {
    fn run(self) -> usize {
        let mut floor = Floor::default();

        for line in self.0 {
            for x in if line.from.x <= line.to.x {
                line.from.x..=line.to.x
            } else {
                line.to.x..=line.from.x
            } {
                for y in if line.from.y <= line.to.y {
                    line.from.y..=line.to.y
                } else {
                    line.to.y..=line.from.y
                } {
                    floor.0[y][x] += 1;
                }
            }
        }

        let mut answer = 0;
        for i in 0..floor.0.len() {
            for j in 0..floor.0[i].len() {
                if floor.0[i][j] >= 2 {
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
            .filter(|line| {
                if let Ok(line) = line {
                    line.from.x == line.to.x || line.from.y == line.to.y
                } else {
                    true
                }
            })
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

    #[test]
    fn test() {
        let environment = super::Environment(io::Cursor::new(INPUT));

        let input = super::Input::try_from(environment);
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(input.run(), 5);
    }
}
