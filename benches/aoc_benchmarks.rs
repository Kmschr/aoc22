
use criterion::{criterion_group, criterion_main, Criterion};
use aoc22::*;

pub fn day_1_part_1(c: &mut Criterion) {
    c.bench_function("Day 1 Part 1", |b| b.iter(|| day1::part1()));
}

pub fn day_1_part_2(c: &mut Criterion) {
    c.bench_function("Day 1 Part 2", |b| b.iter(|| day1::part1()));
}

criterion_group!(day_1, day_1_part_1, day_1_part_2);
criterion_main!(day_1);