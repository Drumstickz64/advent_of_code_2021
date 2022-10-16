pub fn part_one() -> i64 {
    parse_input()
        .into_iter()
        .filter_map(find_illegal_char)
        .map(illegal_char_to_score)
        .sum()
}

pub fn part_two() -> i64 {
    let mut scores: Vec<i64> = parse_input()
        .into_iter()
        .filter_map(|line| find_closing_sequence(&line))
        .map(calculate_closing_sequence_score)
        .collect();

    scores.sort();
    let middle_idx = scores.len() / 2;
    scores[middle_idx]
}

fn parse_input() -> Vec<String> {
    let s = std::fs::read_to_string("src/day10/input.txt")
        .expect("ERROR: src/day10/input.txt is not found");
    s.split('\n').map(|line| line.to_owned()).collect()
}

fn find_illegal_char(line: String) -> Option<char> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                let opening_char = match stack.pop() {
                    Some(opening_char) => opening_char,
                    None => return None,
                };
                if get_closing_char(opening_char) != ch {
                    return Some(ch);
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn get_closing_char(opening_char: char) -> char {
    match opening_char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn illegal_char_to_score(illegal_char: char) -> i64 {
    match illegal_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        ch => panic!("Character '{ch}' is not an illegal character"),
    }
}

fn find_closing_sequence(line: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                let opening_char = stack.pop()?;
                if get_closing_char(opening_char) != ch {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }

    // do closing sequence
    let sequence: Vec<char> = stack.into_iter().rev().map(get_closing_char).collect();
    if sequence.is_empty() {
        None
    } else {
        Some(sequence)
    }
}

fn calculate_closing_sequence_score(sequence: Vec<char>) -> i64 {
    let mut total_score = 0;
    for closing_char in sequence.into_iter() {
        let char_score = closing_char_to_score(closing_char);
        total_score *= 5;
        total_score += char_score;
    }

    total_score
}

fn closing_char_to_score(closing_char: char) -> i64 {
    match closing_char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}
