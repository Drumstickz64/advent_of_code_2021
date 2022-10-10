mod day1;
mod day2;
mod day3;
mod day6;

const DAY_TO_RUN: u32 = 6;
const PART_TO_RUN: u32 = 2;

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
        _ => (),
    }
}
