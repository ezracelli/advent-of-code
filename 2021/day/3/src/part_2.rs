use std::collections::BTreeMap;

use crate::Input;

pub fn get_rating<F>(input: &Input, cmp: F) -> <Input as aoc::Input>::Answer
where
    F: Fn((usize, usize)) -> char,
{
    let mut lines = input.lines.clone();

    // column index -> (# occurences of 0, # occurences of 1)
    let mut bits = BTreeMap::<usize, (usize, usize)>::new();

    let mut i = 0;
    while i < input.bit_count {
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

    usize::from_str_radix(&*lines[0], 2).unwrap()
}
