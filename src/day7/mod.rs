pub fn part_one() -> i32 {
    let mut input = parse_input();
    input.sort();
    let median = find_median_sorted(&input);
    input.iter().copied().map(|pos| (pos - median).abs()).sum()
}

fn parse_input() -> Vec<i32> {
    let s = std::fs::read_to_string("src/day7/input.txt")
        .expect("ERROR: src/day7/input.txt is not found");
    s.split(',').map(|it| it.parse::<i32>().unwrap()).collect()
}

fn find_median_sorted(arr: &[i32]) -> i32 {
    let middle = arr.len() / 2;
    if arr.len() % 2 != 0 {
        (arr[middle] + arr[middle + 1]) / 2
    } else {
        arr[middle]
    }
}
