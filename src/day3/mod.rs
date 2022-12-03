const DATA: &str = include_str!("input");

fn split_rucksack(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    (&s[0..mid], &s[mid..])
}

fn item_in_all_sacks(sacks: &[&str]) -> char {
    let mut checked = [false; 125];
    for item in sacks[0].chars() {
        if checked[item as usize] {
            continue;
        }
        let item_is_common = sacks[1..]
            .into_iter()
            .map(|sack| sack.contains(item))
            .all(|c| c);
        if item_is_common {
            return item;
        }
        checked[item as usize] = true;
    }
    unreachable!()
}

fn priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    }
}

pub fn part1() -> i32 {
    DATA.lines()
        .map(|sack| {
            let (a, b) = split_rucksack(sack);
            let item_type = item_in_all_sacks(&[a, b]);
            priority(item_type)
        })
        .sum()
}

pub fn part2() -> i32 {
    DATA.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|sack_group| priority(item_in_all_sacks(sack_group)))
        .sum()
}
