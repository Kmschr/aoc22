use std::collections::VecDeque;

const DATA: &str = include_str!("input");
const TOTAL_DISK_SPACE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

fn directory_size(lines: &mut VecDeque<&str>, dir_sizes: &mut Vec<usize>) -> (usize, usize) {
    let mut dir_size = 0;
    let mut total_delete_size = 0;
    while let Some(line) = lines.pop_front() {
        let (line_start, line_remaining) = line.split_once(' ').unwrap();
        if line_start == "$" {
            if line_remaining != "ls" {
                let (_, dir) = line_remaining.split_once(' ').unwrap();
                if dir == ".." {
                    break;
                }
                let (sub_dir, sub_total) = directory_size(lines, dir_sizes);
                dir_size += sub_dir;
                total_delete_size += sub_total;
            }
        } else if line_start != "dir" {
            dir_size += line_start.parse::<usize>().unwrap();
        }
    }
    if dir_size <= 100000 {
        total_delete_size += dir_size;
    }
    dir_sizes.push(dir_size);
    (dir_size, total_delete_size)
}

pub fn part1() -> usize {
    let mut lines: VecDeque<&str> = DATA.lines().skip(1).collect();
    let (_total_size, delete_size) = directory_size(&mut lines, &mut vec![]);
    delete_size
}

pub fn part2() -> usize {
    let mut lines: VecDeque<&str> = DATA.lines().skip(1).collect();
    let mut dir_sizes = vec![];
    let (total_size, _delete_size) = directory_size(&mut lines, &mut dir_sizes);
    let delete_size = REQUIRED_SPACE - (TOTAL_DISK_SPACE - total_size);
    let mut min_delete_size = TOTAL_DISK_SPACE;
    for size in dir_sizes {
        if size > delete_size && size < min_delete_size {
            min_delete_size = size;
        }
    }
    min_delete_size
}
