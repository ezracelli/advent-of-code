use std::fs::File;
use std::io::{self, Read};

use clap::Parser as _;

use aoc::Input as _;
use aoc_2021_day_6::Input;

#[derive(Debug, clap::Parser)]
struct Args {
    /// Which part to run
    #[clap(long)]
    part: aoc::Part,
    /// Input filename. Passing "-" will read from STDIN
    filename: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut reader: Box<dyn Read> = match &*args.filename {
        "-" => Box::new(io::stdin()),
        path => Box::new(File::open(path)?),
    };

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let input: Input = input.parse().unwrap();

    println!(
        "{}",
        match args.part {
            aoc::Part::One => input.run_part1(),
            aoc::Part::Two => input.run_part2(),
        }
    );

    Ok(())
}
