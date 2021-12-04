use std::io::{self, BufRead};

const WINDOW_SIZE: usize = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let lines = stdin.lock().lines();
    let numbers = lines
        .map(|line| -> Result<usize, Box<dyn std::error::Error>> { Ok(line?.parse()?) })
        .collect::<Result<Vec<_>, _>>()?;

    let windows = numbers.windows(WINDOW_SIZE);
    let windowed_sums = windows.map(|window| window.iter().sum::<usize>());

    let mut answer = 0usize;
    let mut prev: Option<usize> = None;

    for curr in windowed_sums {
        if let Some(prev) = prev.as_mut() {
            if curr > *prev {
                answer += 1;
            }

            *prev = curr;
        } else {
            prev = Some(curr);
        }
    }

    println!("{}", answer);
    Ok(())
}
