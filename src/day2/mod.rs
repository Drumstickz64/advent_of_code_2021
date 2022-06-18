pub fn part_one() -> i32 {
    let input = get_input_data();

    let mut horizontal_position = 0;
    let mut depth = 0;

    for (command, amount) in &input {
        match &command[..] {
            "forward" => horizontal_position += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => unreachable!(),
        }
    }

    horizontal_position * depth
}

pub fn part_two() -> i32 {
    let input = get_input_data();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (command, amount) in &input {
        match &command[..] {
            "forward" => {
                horizontal_position += amount;
                depth += amount * aim;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => unreachable!(),
        }
    }

    horizontal_position * depth
}

fn get_input_data() -> Vec<(String, i32)> {
    let input = std::fs::read_to_string("src/day2/input.txt").unwrap();
    input
        .lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut items| {
            (
                items.next().unwrap().to_owned(),
                items.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}
