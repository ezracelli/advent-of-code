use std::collections::HashMap;
use std::io::{self, BufRead};

fn get_rating<I, S, F>(lines: I, cmp: F) -> Result<usize, Box<dyn std::error::Error>>
where
    I: AsRef<[S]>,
    S: AsRef<str>,
    F: Fn((usize, usize)) -> char,
{
    let mut lines = lines
        .as_ref()
        .iter()
        .map(|s| s.as_ref())
        .collect::<Vec<_>>();
    let bit_count = lines.first().unwrap().len();

    // column index -> (# occurences of 0, # occurences of 1)
    let mut bits = HashMap::<usize, (usize, usize)>::new();

    let mut i = 0;
    while i < bit_count {
        if lines.len() <= 1 {
            break;
        }

        for line in lines.iter() {
            let char = line.chars().nth(i).unwrap();
            let column = bits.entry(i).or_default();

            match char {
                '0' => column.0 += 1,
                '1' => column.1 += 1,
                _ => panic!("unsupported binary character: {:?}", char),
            }
        }

        let (zeroes, ones) = bits.get(&i).unwrap();
        lines = lines
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == cmp((*zeroes, *ones)))
            .collect();

        i += 1;
    }

    Ok(usize::from_str_radix(lines.get(0).unwrap(), 2)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<Result<Vec<_>, _>>()?;

    let oxygen_generator_rating = get_rating(
        &lines,
        |(zeroes, ones)| {
            if zeroes > ones {
                '0'
            } else {
                '1'
            }
        },
    )?;

    let co2_scrubber_rating = get_rating(
        &lines,
        |(zeroes, ones)| {
            if zeroes > ones {
                '1'
            } else {
                '0'
            }
        },
    )?;

    let answer = oxygen_generator_rating * co2_scrubber_rating;
    println!("{}", answer);
    Ok(())
}
