use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy)]
pub struct Board {
    input: [[usize; 5]; 5],
    state: [[bool; 5]; 5],
}

impl Board {
    pub fn play(&mut self, number: usize) {
        let mut i = 0;
        'outer: while i < 5 {
            let mut j = 0;
            while j < 5 {
                if self.input[i][j] == number {
                    self.state[i][j] = true;
                    break 'outer;
                }

                j += 1;
            }

            i += 1;
        }
    }

    pub fn is_winner(&self) -> bool {
        // check horizontal bingos
        let mut i = 0;
        while i < 5 {
            let mut j = 0;
            while j < 5 {
                if self.state[i][j] != true {
                    break;
                }

                if j == 4 {
                    return true;
                }

                j += 1;
            }

            i += 1;
        }

        // check vertical bingos
        let mut i = 0;
        while i < 5 {
            let mut j = 0;
            while j < 5 {
                if self.state[j][i] != true {
                    break;
                }

                if j == 4 {
                    return true;
                }

                j += 1;
            }

            i += 1;
        }

        false
    }

    pub fn score(&self, number: usize) -> usize {
        let mut score = 0;

        let mut i = 0;
        while i < 5 {
            let mut j = 0;
            while j < 5 {
                if self.state[i][j] != true {
                    score += self.input[i][j];
                }

                j += 1;
            }

            i += 1;
        }

        score * number
    }
}

impl FromStr for Board {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input: [[usize; 5]; 5] = Default::default();

        for (i, line) in s.lines().enumerate() {
            for (j, number) in line.split_ascii_whitespace().enumerate() {
                input[i][j] = number.parse().unwrap();
            }
        }

        Ok(Self {
            input,
            ..Default::default()
        })
    }
}
