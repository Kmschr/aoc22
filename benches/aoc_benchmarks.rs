use aoc22::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn day_1_part_1(c: &mut Criterion) {
    c.bench_function("Day 1 Part 1", |b| b.iter(|| day1::part1()));
}

pub fn day_1_part_2(c: &mut Criterion) {
    c.bench_function("Day 1 Part 2", |b| b.iter(|| day1::part2()));
}

pub fn day_2_part_1(c: &mut Criterion) {
    c.bench_function("Day 2 Part 1", |b| b.iter(|| day2::part1()));
}

pub fn day_2_part_2(c: &mut Criterion) {
    c.bench_function("Day 2 Part 2", |b| b.iter(|| day2::part2()));
}

pub fn day_2_part_1_lookup(c: &mut Criterion) {
    c.bench_function("Day 2 Part 1", |b| b.iter(|| day2::part1_lookup()));
}

pub fn day_2_part_2_lookup(c: &mut Criterion) {
    c.bench_function("Day 2 Part 2", |b| b.iter(|| day2::part2_lookup()));
}

criterion_group!(day_1, day_1_part_1, day_1_part_2);
criterion_group!(
    day_2,
    day_2_part_1,
    day_2_part_2,
    day_2_part_1_lookup,
    day_2_part_2_lookup
);
criterion_main!(day_1, day_2);
