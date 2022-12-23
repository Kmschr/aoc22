use aoc22::*;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

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

pub fn day_3_part_1(c: &mut Criterion) {
    c.bench_function("Day 3 Part 1", |b| b.iter(|| day3::part1()));
}

pub fn day_3_part_2(c: &mut Criterion) {
    c.bench_function("Day 3 Part 2", |b| b.iter(|| day3::part2()));
}

pub fn day_4_part_1(c: &mut Criterion) {
    c.bench_function("Day 4 Part 1", |b| b.iter(|| day4::part1()));
}

pub fn day_4_part_2(c: &mut Criterion) {
    c.bench_function("Day 4 Part 2", |b| b.iter(|| day4::part2()));
}

pub fn day_5_part_1(c: &mut Criterion) {
    c.bench_function("Day 5 Part 1", |b| b.iter(|| day5::part1()));
}

pub fn day_5_part_2(c: &mut Criterion) {
    c.bench_function("Day 5 Part 2", |b| b.iter(|| day5::part2()));
}

pub fn day_6_part_1(c: &mut Criterion) {
    c.bench_function("Day 6 Part 1", |b| b.iter(|| day6::part1()));
}

pub fn day_6_part_2(c: &mut Criterion) {
    c.bench_function("Day 6 Part 2", |b| b.iter(|| day6::part2()));
}

pub fn day_7_part_1(c: &mut Criterion) {
    c.bench_function("Day 7 Part 1", |b| b.iter(|| day7::part1()));
}

pub fn day_7_part_2(c: &mut Criterion) {
    c.bench_function("Day 7 Part 2", |b| b.iter(|| day7::part2()));
}

criterion_group!(day_1, day_1_part_1, day_1_part_2);

criterion_group!(day_2, day_2_part_1, day_2_part_2, day_2_part_1_lookup, day_2_part_2_lookup);

criterion_group!(day_3, day_3_part_1, day_3_part_2);

criterion_group!(day_4, day_4_part_1, day_4_part_2);

criterion_group!(day_5, day_5_part_1, day_5_part_2);

criterion_group!(day_6, day_6_part_1, day_6_part_2);

criterion_group!(day_7, day_7_part_1, day_7_part_2);

criterion_main!(day_1, day_2, day_3, day_4, day_5, day_6, day_7);
