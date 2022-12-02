const DATA: &str = include_str!("input");

const STARTX: usize = 'X' as usize;
const STARTA: usize = 'A' as usize;
const PART1: [[u32; 3]; 3] = [[4, 1, 7], [8, 5, 2], [3, 9, 6]];
const PART2: [[u32; 3]; 3] = [[3, 1, 2], [4, 5, 6], [8, 9, 7]];

fn parse_round(round: &str) -> (char, char) {
    let mut round = round.chars();
    (round.next().unwrap(), round.nth(1).unwrap())
}

fn score_round_part1_lookup(round: &str) -> u32 {
    let (opponent_move, my_move) = parse_round(round);
    PART1[my_move as usize - STARTX][opponent_move as usize - STARTA]
}

fn score_round_part1(round: &str) -> u32 {
    let (opponent_move, my_move) = parse_round(round);
    match my_move {
        'X' => {
            // Rock
            1 + match opponent_move {
                'A' => 3, // Rock (Tie)
                'B' => 0, // Paper (Lose)
                'C' => 6, // Scissors (Win)
                _ => unreachable!(),
            }
        }
        'Y' => {
            // Paper
            2 + match opponent_move {
                'A' => 6, // Rock (Win)
                'B' => 3, // Paper (Tie)
                'C' => 0, // Scissors (Lose)
                _ => unreachable!(),
            }
        }
        'Z' => {
            // Scissors
            3 + match opponent_move {
                'A' => 0, // Rock (Lose)
                'B' => 6, // Paper (Win)
                'C' => 3, // Scissors (Tie)
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn score_round_part2_lookup(round: &str) -> u32 {
    let (opponent_move, result) = parse_round(round);
    PART2[result as usize - STARTX][opponent_move as usize - STARTA]
}

fn score_round_part2(round: &str) -> u32 {
    let (opponent_move, result) = parse_round(round);
    match result {
        'X' => {
            // Lose
            match opponent_move {
                'A' => 3, // Rock (Scizzors to Lose)
                'B' => 1, // Paper (Rock to Lose)
                'C' => 2, // Scissors (Paper to Lose)
                _ => unreachable!(),
            }
        }
        'Y' => {
            // Draw
            3 + match opponent_move {
                'A' => 1, // Rock
                'B' => 2, // Paper
                'C' => 3, // Scissors
                _ => unreachable!(),
            }
        }
        'Z' => {
            // Win
            6 + match opponent_move {
                'A' => 2, // Rock (Paper to Win)
                'B' => 3, // Paper (Scissors to Win)
                'C' => 1, // Scissors (Rock to Win)
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

pub fn part1() -> u32 {
    DATA.lines().map(|round| score_round_part1(round)).sum()
}

pub fn part2() -> u32 {
    DATA.lines().map(|round| score_round_part2(round)).sum()
}

pub fn part1_lookup() -> u32 {
    DATA.lines()
        .map(|round| score_round_part1_lookup(round))
        .sum()
}

pub fn part2_lookup() -> u32 {
    DATA.lines()
        .map(|round| score_round_part2_lookup(round))
        .sum()
}
