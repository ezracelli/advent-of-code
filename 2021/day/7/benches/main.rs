use criterion::{black_box, Criterion};

use aoc::Input as _;
use aoc_2021_day_7::Input;

const INPUT: &str = include_str!("../test.txt");

fn benchmark_parse_day_7(c: &mut Criterion) {
    c.bench_function("parse day 7", |b| {
        b.iter(|| black_box(INPUT).parse::<Input>().unwrap())
    });
}

fn benchmark_run_day_7_part_1(c: &mut Criterion) {
    let input: Input = INPUT.parse().unwrap();
    c.bench_function("run day 7.1", |b| b.iter(|| input.run_part1()));
}

fn benchmark_run_day_7_part_2(c: &mut Criterion) {
    let input: Input = INPUT.parse().unwrap();
    c.bench_function("run day 7.2", |b| b.iter(|| input.run_part2()));
}

criterion::criterion_group!(
    benches,
    benchmark_parse_day_7,
    benchmark_run_day_7_part_1,
    benchmark_run_day_7_part_2
);
criterion::criterion_main!(benches);
