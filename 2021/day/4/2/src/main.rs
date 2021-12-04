use std::collections::HashSet;
use std::io::{self, BufRead};

use itertools::Itertools as _;

#[derive(Debug, Default, Clone, Copy)]
struct Board {
    input: [[usize; 5]; 5],
    state: [[bool; 5]; 5],
}

impl Board {
    fn play(&mut self, number: usize) {
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

    fn is_winner(&self) -> bool {
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

    fn score(&self, number: usize) -> usize {
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

impl From<[[usize; 5]; 5]> for Board {
    fn from(input: [[usize; 5]; 5]) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct Input {
    numbers: Vec<usize>,
    boards: HashSet<[[usize; 5]; 5]>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let input = {
        let mut input = Input::default();

        let mut lines = stdin.lock().lines();

        input.numbers = lines
            .next()
            .unwrap()?
            .split(',')
            .map(|number| number.parse())
            .collect::<Result<Vec<_>, _>>()?;

        lines.next();
        for board in lines.chunks(6).into_iter() {
            input.boards.insert(
                board
                    .into_iter()
                    .take(5)
                    .map(|row| -> Result<_, Box<dyn std::error::Error>> {
                        Ok(row?
                            .split_ascii_whitespace()
                            .map(|number| number.parse())
                            .collect::<Result<Vec<_>, _>>()?
                            .try_into()
                            .map_err(|_| "invalid row")?)
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
                    .map_err(|_| "invalid board")?,
            );
        }

        input
    };

    let mut boards = input
        .boards
        .into_iter()
        .map(Board::from)
        .collect::<Vec<_>>();

    for number in input.numbers {
        let mut winning_indices = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            board.play(number);

            if board.is_winner() {
                winning_indices.push(i);
            }
        }

        if winning_indices.len() == boards.len() {
            println!("{}", boards.first().unwrap().score(number));
            break;
        }

        winning_indices.reverse();
        for i in winning_indices {
            boards.swap_remove(i);
        }
    }

    Ok(())
}
