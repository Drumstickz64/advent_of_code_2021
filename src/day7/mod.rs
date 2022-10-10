pub fn part_one() -> i64 {
    let mut input = parse_input();
    input.sort();
    let median = find_median_sorted(&input);
    input.iter().copied().map(|pos| (pos - median).abs()).sum()
}

pub fn part_two() -> i64 {
    let input = parse_input();
    let mean = find_mean(&input);
    let target_pos = mean.trunc() as i64;
    input
        .iter()
        .copied()
        .map(|pos| sum_1_to_n((pos - target_pos).abs()))
        .sum()
}

fn parse_input() -> Vec<i64> {
    let s = std::fs::read_to_string("src/day7/input.txt")
        .expect("ERROR: src/day7/input.txt is not found");
    s.split(',').map(|it| it.parse::<i64>().unwrap()).collect()
}

fn find_mean(arr: &[i64]) -> f64 {
    arr.iter().copied().sum::<i64>() as f64 / arr.len() as f64
}

fn find_median_sorted(arr: &[i64]) -> i64 {
    let middle = arr.len() / 2;
    if arr.len() % 2 != 0 {
        (arr[middle] + arr[middle + 1]) / 2
    } else {
        arr[middle]
    }
}

fn sum_1_to_n(n: i64) -> i64 {
    n * (n + 1) / 2
}
