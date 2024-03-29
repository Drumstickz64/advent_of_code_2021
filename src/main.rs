mod day1;
mod day10;
mod day2;
mod day3;
mod day6;
mod day7;
mod day9;

const DAY_TO_RUN: u32 = 10;
const PART_TO_RUN: u32 = 2;

#[allow(clippy::single_match)]
fn main() {
    match DAY_TO_RUN {
        1 => match PART_TO_RUN {
            1 => println!("Day 1 part 1: {}", day1::part_one()),
            2 => println!("Day 1 part 2: {}", day1::part_two()),
            _ => (),
        },
        2 => match PART_TO_RUN {
            1 => println!("Day 2 part 1: {}", day2::part_one()),
            2 => println!("Day 2 part 2: {}", day2::part_two()),
            _ => (),
        },
        3 => match PART_TO_RUN {
            1 => println!("Day 3 part 1: {}", day3::part_one()),
            2 => println!("Day 3 part 2: {}", day3::part_two()),
            _ => (),
        },
        6 => match PART_TO_RUN {
            1 => println!("Day 6 part 1: {}", day6::part_one()),
            2 => println!("Day 6 part 2: {}", day6::part_two()),
            _ => (),
        },
        7 => match PART_TO_RUN {
            1 => println!("Day 7 part 1: {}", day7::part_one()),
            2 => println!("Day 7 part 2: {}", day7::part_two()),
            _ => (),
        },
        9 => match PART_TO_RUN {
            1 => println!("Day 9 part 1: {}", day9::part_one()),
            _ => (),
        },
        10 => match PART_TO_RUN {
            1 => println!("Day 10 part 1: {}", day10::part_one()),
            2 => println!("Day 10 part 2: {}", day10::part_two()),
            _ => (),
        },
        _ => (),
    }
}
