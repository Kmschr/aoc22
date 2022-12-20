use std::collections::VecDeque;

const DATA: &str = include_str!("input");

fn parse_cargo() -> Vec<Vec<char>> {
    let mut stacks = vec![];
    let cargo: Vec<&str> = DATA.lines().take(8).collect();
    let cargo: Vec<&str> = cargo.into_iter().rev().collect();
    for stack_idx in 0 .. 9 {
        let mut stack = vec![];
        for crate_idx in 0 .. 8 {
            let c = cargo[crate_idx].chars().nth(stack_idx * 4 + 1).unwrap();
            if c == ' ' {
                break;
            }
            stack.push(c)
        }
        stacks.push(stack);
    }
    stacks
}

pub fn part1() -> String {
    let mut stacks = parse_cargo();
    DATA
        .lines()
        .skip(10)
        .map(|line| line.split(" ").filter_map(|num| num.parse().ok()).collect::<Vec<usize>>())
        .for_each(|step| {
            let n = step[0];
            let src = step[1] - 1;
            let dst = step[2] - 1;
            for _ in 0 .. n {
                let c = stacks[src].pop().expect("stack is empty");
                stacks[dst].push(c);
            }
        });
    stacks.into_iter().map(|stack| stack[stack.len() - 1]).collect()
}

pub fn part2() -> String {
    let mut stacks = parse_cargo();
    DATA
        .lines()
        .skip(10)
        .map(|line| line.split(" ").filter_map(|num| num.parse().ok()).collect::<Vec<usize>>())
        .for_each(|step| {
            let n = step[0];
            let src = step[1] - 1;
            let dst = step[2] - 1;
            let mut grabbed: VecDeque<char> = VecDeque::with_capacity(n);
            for _ in 0 .. n {
                let c = stacks[src].pop().expect("stack is empty");
                grabbed.push_front(c);
            }
            for _ in 0 .. n {
                let c = grabbed.pop_front().expect("stack is empty");
                stacks[dst].push(c);
            }
        });
    stacks.into_iter().map(|stack| stack[stack.len() - 1]).collect()
}
