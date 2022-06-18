use std::fs;

pub fn part_one() -> i32 {
    let input = get_input_data();
    let mut prev_depth = input[0];
    let mut times_increased = 0;

    for x in input.iter().skip(1) {
        if *x > prev_depth {
            times_increased += 1;
        }
        prev_depth = *x;
    }

    times_increased
}

pub fn part_two() -> i32 {
    let input = get_input_data();
    let mut prev_window = input[0] + input[1] + input[2];
    let mut times_increased = 0;

    for x in input.windows(3).skip(1) {
        let sum: i32 = x.iter().sum();
        if sum > prev_window {
            times_increased += 1;
        }
        prev_window = sum;
    }

    times_increased
}

fn get_input_data() -> Vec<i32> {
    let input = fs::read_to_string("src/day1/input.txt").unwrap();
    input
        .lines()
        .filter_map(|item| item.parse::<i32>().ok())
        .collect()
}
