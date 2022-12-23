use std::collections::HashSet;

const DATA: &[u8; 9899] = include_bytes!("input");

pub fn part1() -> usize {
    let mut visible: HashSet<usize> = HashSet::new();
    for row in 0 .. 99 {
        let mut max = 0;
        for col in 0 .. 99 {
            let i = col + row * 100;
            if DATA[i] > max {
                max = DATA[i];
                visible.insert(i);
            }
        }
        let mut max = 0;
        for col in (0 .. 99).rev() {
            let i = col + row * 100;
            if DATA[i] > max {
                max = DATA[i];
                visible.insert(i);
            }
        }
    }
    for col in 0 .. 99 {
        let mut max = 0;
        for row in 0 .. 99 {
            let i = col + row * 100;
            if DATA[i] > max {
                max = DATA[i];
                visible.insert(i);
            }
        }
        let mut max = 0;
        for row in (0 .. 99).rev() {
            let i = col + row * 100;
            if DATA[i] > max {
                max = DATA[i];
                visible.insert(i);
            }
        }
    }
    visible.len()
}

fn score_location(row: usize, col: usize) -> usize {
    let height = DATA[col + row * 100];
    let mut left_score = 0;
    while let Some(left) = col.checked_sub(left_score + 1) {
        left_score += 1;
        if height <= DATA[left + row * 100] {
            break;
        }
    }
    let mut right_score = 0;
    loop {
        let right = col + (right_score + 1);
        if right < 99 {
            right_score += 1;
            if height <= DATA[right + row * 100] {
                break;
            }
        } else {
            break;
        }
    }
    let mut up_score = 0;
    while let Some(up) = row.checked_sub(up_score + 1) {
        up_score += 1;
        if height <= DATA[col + up * 100] {
            break;
        }
    }
    let mut down_score = 0;
    loop {
        let down = row + (down_score + 1);
        if down < 99 {
            down_score += 1;
            if height <= DATA[col + down * 100] {
                break;
            }
        } else {
            break;
        }
    }
    left_score * right_score * up_score * down_score
}

pub fn part2() -> usize {
    let mut max_score = 0;
    for row in 0 .. 99 {
        for col in 0 .. 99 {
            let score = score_location(row, col);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}
