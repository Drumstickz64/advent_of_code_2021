pub fn part_one() -> i32 {
    let input = get_input_data();
    let mut cols = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for item in input {
        for (bit, col) in item.chars().zip(&mut cols) {
            col.push(bit);
        }
    }

    let mut gamma_sequence: Vec<char> = Vec::new();
    let mut epsilon_sequence: Vec<char> = Vec::new();
    for col in cols {
        let (zeros_count, ones_count) = get_bit_counts(&col[..]);
        let most_common_bit = if zeros_count > ones_count { '0' } else { '1' };
        gamma_sequence.push(most_common_bit);
        if most_common_bit == '0' {
            epsilon_sequence.push('1');
        } else {
            epsilon_sequence.push('0');
        }
    }

    let gamma_sequence: String = gamma_sequence.iter().collect();
    let gamma_rate = i32::from_str_radix(&gamma_sequence, 2).unwrap();

    let epsilon_sequence: String = epsilon_sequence.iter().collect::<String>();
    let epsilon_rate = i32::from_str_radix(&epsilon_sequence, 2).unwrap();

    gamma_rate * epsilon_rate
}

pub fn part_two() -> i32 {
    get_oxygen_generator_rating() * get_co2_scrubber_rating()
}

fn get_oxygen_generator_rating() -> i32 {
    let mut report = get_input_data();

    for i in 0..12 {
        if report.len() == 1 {
            break;
        }
        let row_bits: Vec<char> = report
            .iter()
            .map(|item| item.chars().nth(i).unwrap())
            .collect();

        let (zeros_count, ones_count) = get_bit_counts(&row_bits[..]);
        let most_common_bit = if zeros_count > ones_count { '0' } else { '1' };
        report = report
            .into_iter()
            .filter(|item| item.chars().nth(i).unwrap() == most_common_bit)
            .collect();
    }

    i32::from_str_radix(&report[0], 2).unwrap()
}

fn get_co2_scrubber_rating() -> i32 {
    let mut report = get_input_data();

    for i in 0..12 {
        if report.len() == 1 {
            break;
        }
        let row_bits: Vec<char> = report
            .iter()
            .map(|item| item.chars().nth(i).unwrap())
            .collect();

        let (zeros_count, ones_count) = get_bit_counts(&row_bits[..]);
        let most_common_bit = if zeros_count > ones_count { '0' } else { '1' };
        report = report
            .into_iter()
            .filter(|item| item.chars().nth(i).unwrap() != most_common_bit)
            .collect();
    }

    i32::from_str_radix(&report[0], 2).unwrap()
}

fn get_bit_counts(slice: &[char]) -> (i32, i32) {
    let mut zeros_count = 0;
    let mut ones_count = 0;

    for item in slice {
        if *item == '0' {
            zeros_count += 1;
        } else {
            ones_count += 1;
        }
    }

    (zeros_count, ones_count)
}

fn get_input_data() -> Vec<String> {
    let input = std::fs::read_to_string("src/day3/input.txt").unwrap();
    input.lines().map(str::to_string).collect()
}
