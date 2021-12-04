use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PowerConsumption {
    gamma: usize,
    epsilon: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    // column index -> (# occurences of 0, # occurences of 1)
    let mut bits = HashMap::<usize, (usize, usize)>::new();

    for line in stdin.lock().lines() {
        for (i, char) in line?.chars().enumerate() {
            let column = bits.entry(i).or_default();

            match char {
                '0' => column.0 += 1,
                '1' => column.1 += 1,
                _ => panic!("unsupported binary character: {:?}", char),
            }
        }
    }

    let mut bits = bits.into_iter().collect::<Vec<_>>();
    bits.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let power_consumption = PowerConsumption {
        gamma: usize::from_str_radix(
            &*bits
                .iter()
                .map(|(_, (zeroes, ones))| {
                    if zeroes > ones {
                        '0'
                    } else if ones > zeroes {
                        '1'
                    } else {
                        panic!("undefined behavior: zeroes == ones")
                    }
                })
                .collect::<String>(),
            2,
        )?,
        epsilon: usize::from_str_radix(
            &*bits
                .iter()
                .map(|(_, (zeroes, ones))| {
                    if zeroes > ones {
                        '1'
                    } else if ones > zeroes {
                        '0'
                    } else {
                        panic!("undefined behavior: zeroes == ones")
                    }
                })
                .collect::<String>(),
            2,
        )?,
    };

    let answer = power_consumption.gamma * power_consumption.epsilon;
    println!("{}", answer);
    Ok(())
}
