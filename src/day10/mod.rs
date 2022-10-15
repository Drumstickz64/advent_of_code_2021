pub fn part_one() -> i32 {
    let input = parse_input();
    input
        .into_iter()
        .filter_map(|line| find_illegal_char(&line))
        .map(char_to_score)
        .sum()
}

fn parse_input() -> Vec<String> {
    let s = std::fs::read_to_string("src/day10/input.txt")
        .expect("ERROR: src/day10/input.txt is not found");
    s.split('\n').map(|line| line.to_owned()).collect()
}

fn find_illegal_char(line: &str) -> Option<char> {
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

fn char_to_score(illegal_char: char) -> i32 {
    match illegal_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        ch => panic!("Character '{ch}' is not an illegal character"),
    }
}
