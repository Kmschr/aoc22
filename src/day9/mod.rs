use std::collections::HashSet;

const DATA: &str = include_str!("input");

fn parse_moves() -> Vec<(&'static str, usize)> {
    DATA.lines().map(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        let dist = dist.parse::<usize>().unwrap();
        (dir, dist)
    }).collect()
}

#[derive(Default, Eq, Hash, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

struct Rope {
    knots: Vec<Vec2>,
    size: usize,
}

impl Rope {
    fn new(n: usize) -> Self {
        let mut knots = Vec::with_capacity(n);
        for _ in 0 .. n {
            knots.push(Vec2::default());
        }
        Self {
            knots,
            size: n,
        }
    }

    fn pull(&mut self, dir: &str) {
        let rope = &mut self.knots;
        let tail = self.size - 1;
        match dir {
            "R" => {
                rope[0].x += 1;
            },
            "L" => {
                rope[0].x -= 1;
            },
            "U" => {
                rope[0].y += 1;
            },
            "D" => {
                rope[0].y -= 1;
            },
            _ => unreachable!(),
        }
        for i in 0 .. tail {
            let dx = rope[i].x - rope[i + 1].x;
            let dy = rope[i].y - rope[i + 1].y;
            let move_x = if dx > 1 {
                1
            } else if dx < -1 {
                -1
            } else {
                0
            };
            let move_y = if dy > 1 {
                1
            } else if dy < -1 {
                -1
            } else {
                0
            };
            if move_x != 0 && move_y != 0 {
                rope[i + 1] = Vec2 {
                    x: rope[i + 1].x + move_x,
                    y: rope[i + 1].y + move_y,
                };
            } else if move_x != 0 {
                rope[i + 1] = Vec2 {
                    x: rope[i].x - move_x,
                    y: rope[i].y,
                };
            } else if move_y != 0 {
                rope[i + 1] = Vec2 {
                    x: rope[i].x,
                    y: rope[i].y - move_y,
                };
            }
        }
    }

    fn tail(&self) -> Vec2 {
        self.knots[self.size - 1].clone()
    }
}

pub fn part1() -> usize {
    let moves = parse_moves();
    let mut visited: HashSet::<Vec2> = HashSet::new();
    visited.insert(Vec2::default());
    let mut rope = Rope::new(2);
    for (dir, dist) in moves {
        for _ in 0 .. dist {
            rope.pull(dir);
            visited.insert(rope.tail());
        }
    }
    visited.len()
}

pub fn part2() -> usize {
    let moves = parse_moves();
    let mut visited: HashSet::<Vec2> = HashSet::new();
    visited.insert(Vec2::default());
    let mut rope = Rope::new(10);
    for (dir, dist) in moves {
        for _ in 0 .. dist {
            rope.pull(dir);
            visited.insert(rope.tail());
        }
    }
    visited.len()
}
