const DATA: &[u8; 4095] = include_bytes!("input");

fn all_unique(seq: &[u8]) -> bool {
    for i in 0 .. seq.len() - 1 {
        for j in i + 1 .. seq.len() {
            if seq[i] == seq[j] {
                return false;
            }
        }
    }
    true
}

fn find_sequence_n(n: usize) -> usize {
    let mut start = 0;
    let mut end = n;
    while end < DATA.len() {
        let seq = &DATA[start .. end];
        if all_unique(seq) {
            return end;
        }
        start += 1;
        end += 1;
    }
    unreachable!()
}

pub fn part1() -> usize {
    find_sequence_n(4)
}

pub fn part2() -> usize {
    find_sequence_n(14)
}
