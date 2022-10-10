const BASE_DAYS_TILL_BIRTH: usize = 6;
const CHILD_DAYS_TILL_BIRTH: usize = 8;
const PART_ONE_SIMULATION_DAY_COUNT: u64 = 80;
const PART_TWO_SIMULATION_DAY_COUNT: u64 = 256;

pub fn part_one() -> u64 {
    calc_fish_at_day(PART_ONE_SIMULATION_DAY_COUNT)
}

pub fn part_two() -> u64 {
    calc_fish_at_day(PART_TWO_SIMULATION_DAY_COUNT)
}

fn calc_fish_at_day(day_count: u64) -> u64 {
    let mut fishes_at_days_till_birth = parse_input();
    for _ in (0..day_count).rev() {
        println!("{fishes_at_days_till_birth:?}");
        let fishes_to_birth = fishes_at_days_till_birth[0];
        fishes_at_days_till_birth[0] = 0;
        for i in 1..9 {
            fishes_at_days_till_birth[i - 1] += fishes_at_days_till_birth[i];
            fishes_at_days_till_birth[i] = 0;
        }
        fishes_at_days_till_birth[CHILD_DAYS_TILL_BIRTH] += fishes_to_birth;
        fishes_at_days_till_birth[BASE_DAYS_TILL_BIRTH] += fishes_to_birth;
    }

    fishes_at_days_till_birth.into_iter().sum()
}

fn parse_input() -> [u64; 9] {
    let s = std::fs::read_to_string("src/day6/input.txt")
        .expect("ERROR: input.txt not found in src/day6/input.txt");
    let mut fishes_at_days_till_birth = [0; 9];
    for days_till_birth in s.split(',').map(|num| num.parse::<usize>().unwrap()) {
        fishes_at_days_till_birth[days_till_birth] += 1;
    }
    fishes_at_days_till_birth
}
