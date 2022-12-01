use itertools::*;

const DATA: &str = include_str!("input");

pub fn part1() -> i32 {
    DATA.lines()
        .group_by(|line: &&str| line != &"")
        .into_iter()
        .map(|(is_calorie, calories_list)| {
            if !is_calorie {
                return 0;
            }
            calories_list.map(|calories: &str| str::parse::<i32>(calories).unwrap()).sum()
        }).max().expect("failed to get max calories")
}

pub fn part2() -> i32 {
    DATA.lines()
        .group_by(|line: &&str| line != &"")
        .into_iter()
        .map(|(is_calorie, calories_list)| {
            if !is_calorie {
                return 0;
            }
            calories_list.map(|calories: &str| str::parse::<i32>(calories).unwrap()).sum()
        }).sorted().rev().take(3).sum()
}
