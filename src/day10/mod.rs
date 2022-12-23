const DATA: &str = include_str!("input");

enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instructions() -> Vec<Instruction> {
    DATA.lines().map(|line| {
        match line {
            "noop" => Instruction::Noop,
            _ => {
                let (_, imm) = line.split_once(' ').unwrap();
                let imm = imm.parse::<i32>().unwrap();
                Instruction::Addx(imm)
            },
        }
    }).collect()
}

pub fn part1() -> i32 {
    let program = parse_instructions();
    let mut x: i32 = 1;
    let mut cycles: i32 = 0;
    let mut cycles_left: i32 = 1;
    let mut signal_strength = 0;
    let mut instruction = &program[0];
    let mut pc = 1;
    loop {
        if (cycles - 20) % 40 == 0 {
            signal_strength += cycles * x;
        }
        if cycles_left == 0 {
            match instruction {
                Instruction::Noop => { },
                Instruction::Addx(n) => {
                    x += n;
                },
            }
            if pc >= program.len() {
                break;
            }
            instruction = &program[pc];
            match instruction {
                Instruction::Noop => {
                    cycles_left = 1;
                },
                Instruction::Addx(_) => {
                    cycles_left = 2;
                },
            }
            pc += 1;
        }
        cycles += 1;
        cycles_left -= 1;
    }
    signal_strength
}

pub fn part2() -> String {
    let mut crt = [[' '; 40]; 6];
    let program = parse_instructions();
    let mut x: i32 = 1;
    let mut cycles: i32 = 0;
    let mut cycles_left: i32 = 1;
    let mut instruction = &program[0];
    let mut pc = 1;
    loop {
        if cycles_left == 0 {
            match instruction {
                Instruction::Noop => { },
                Instruction::Addx(n) => {
                    x += n;
                },
            }
            if pc >= program.len() {
                break;
            }
            instruction = &program[pc];
            match instruction {
                Instruction::Noop => {
                    cycles_left = 1;
                },
                Instruction::Addx(_) => {
                    cycles_left = 2;
                },
            }
            pc += 1;
        }
        let col = cycles % 40;
        if (col as i32 - x).abs() <= 1 {
            let row = cycles / 40;
            crt[row as usize][col as usize] = '█';
        }
        cycles += 1;
        cycles_left -= 1;
    }
    let mut output = String::from('\n');
    for row in 0 .. 6 {
        for col in 0 .. 40 {
            output.push(crt[row][col]);
        }
        output.push('\n');
    }
    output
}
