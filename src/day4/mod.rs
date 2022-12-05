const DATA: &str = include_str!("input");

fn parse_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
    let elves = line.split_once(',').unwrap();
    let first_range = elves.0.split_once('-').unwrap();
    let second_range = elves.1.split_once('-').unwrap();
    let first_range = (
        first_range.0.parse::<i32>().unwrap(),
        first_range.1.parse::<i32>().unwrap(),
    );
    let second_range = (
        second_range.0.parse::<i32>().unwrap(),
        second_range.1.parse::<i32>().unwrap(),
    );
    (first_range, second_range)
}

fn one_in_another(line: &str) -> bool {
    let (first_range, second_range) = parse_ranges(line);
    range_contains_other(first_range, second_range)
        || range_contains_other(second_range, first_range)
}

fn overlaps(line: &str) -> bool {
    let (range1, range2) = parse_ranges(line);
    let max_start = std::cmp::max(range1.0, range2.0);
    let min_end = std::cmp::min(range1.1, range2.1);
    min_end >= max_start
}

fn range_contains_other(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

pub fn part1() -> usize {
    DATA.lines().filter(|line| one_in_another(line)).count()
}

pub fn part2() -> usize {
    DATA.lines().filter(|line| overlaps(line)).count()
}
