pub mod day1;
pub mod day2;

pub fn run() {
    println!("Day 1");
    println!("Part 1: {}", day1::part1());
    println!("Part 2: {}", day1::part2());
    println!();
    println!("Day 2");
    println!("Part 1: {}", day2::part1());
    println!("Part 2: {}", day2::part2());
    println!("(Lookup Table) Part 1: {}", day2::part1_lookup());
    println!("(Lookup Table) Part 2: {}", day2::part2_lookup());
}
