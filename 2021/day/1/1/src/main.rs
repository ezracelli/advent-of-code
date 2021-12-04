use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    let mut answer = 0usize;
    let mut prev: Option::<usize> = None;

    for line in stdin.lock().lines() {
        let curr = line?.parse::<usize>()?;

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
