use criterion::{black_box, Criterion};

use aoc::Input as _;
use aoc_2021_day_8::Input;

const INPUT_1: &str = include_str!("../test.1.txt");
const INPUT_2: &str = include_str!("../test.2.txt");

fn benchmark_parse_1_day_8(c: &mut Criterion) {
    c.bench_function("parse 1 day 8", |b| {
        b.iter(|| black_box(INPUT_1).parse::<Input>().unwrap())
    });
}

fn benchmark_parse_2_day_8(c: &mut Criterion) {
    c.bench_function("parse 2 day 8", |b| {
        b.iter(|| black_box(INPUT_2).parse::<Input>().unwrap())
    });
}

fn benchmark_run_1_day_8_part_1(c: &mut Criterion) {
    let input: Input = INPUT_1.parse().unwrap();
    c.bench_function("run 1 day 8.1", |b| b.iter(|| input.run_part1()));
}

fn benchmark_run_1_day_8_part_2(c: &mut Criterion) {
    let input: Input = INPUT_1.parse().unwrap();
    c.bench_function("run 1 day 8.2", |b| b.iter(|| input.run_part2()));
}

fn benchmark_run_2_day_8_part_1(c: &mut Criterion) {
    let input: Input = INPUT_2.parse().unwrap();
    c.bench_function("run 2 day 8.1", |b| b.iter(|| input.run_part1()));
}

fn benchmark_run_2_day_8_part_2(c: &mut Criterion) {
    let input: Input = INPUT_2.parse().unwrap();
    c.bench_function("run 2 day 8.2", |b| b.iter(|| input.run_part2()));
}

criterion::criterion_group!(
    benches,
    benchmark_parse_1_day_8,
    benchmark_run_1_day_8_part_1,
    benchmark_run_1_day_8_part_2,
    benchmark_parse_2_day_8,
    benchmark_run_2_day_8_part_1,
    benchmark_run_2_day_8_part_2
);
criterion::criterion_main!(benches);
