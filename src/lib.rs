pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn run() {
    println!("Day 1");
    println!("Part 1: {}", day1::part1());
    println!("Part 2: {}", day1::part2());
    println!();
    println!("Day 2");
    println!("Part 1: {}", day2::part1());
    println!("Part 2: {}", day2::part2());

    // println!("(Lookup Table) Part 1: {}", day2::part1_lookup()); println!("(Lookup Table)
    // Part 2: {}", day2::part2_lookup());
    println!();
    println!("Day 3");
    println!("Part 1: {}", day3::part1());
    println!("Part 2: {}", day3::part2());
    println!();
    println!("Day 4");
    println!("Part 1: {}", day4::part1());
    println!("Part 2: {}", day4::part2());
    println!();
    println!("Day 5");
    println!("Part 1: {}", day5::part1());
    println!("Part 2: {}", day5::part2());
    println!();
    println!("Day 6");
    println!("Part 1: {}", day6::part1());
    println!("Part 2: {}", day6::part2());
}
