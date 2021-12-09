use std::collections::BTreeMap;

use crate::Input;

pub fn get_rating<F>(input: &Input, cmp: F) -> <Input as aoc::Input>::Answer
where
    F: Fn((usize, usize)) -> char,
{
    // column index -> (# occurences of 0, # occurences of 1)
    let mut bits = BTreeMap::<usize, (usize, usize)>::new();

    for line in input.lines.iter() {
        for (i, char) in line.chars().enumerate() {
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

    usize::from_str_radix(
        &*bits
            .into_iter()
            .map(|(_, (zeroes, ones))| cmp((zeroes, ones)))
            .collect::<String>(),
        2,
    )
    .unwrap()
}
