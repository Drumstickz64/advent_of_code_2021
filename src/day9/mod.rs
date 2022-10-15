const GRID_WIDTH: usize = 100;

pub fn part_one() -> i32 {
    let input = parse_input();
    let mut low_points = Vec::new();
    for y in 0..input.len() {
        for x in 0..GRID_WIDTH {
            let adj_items = get_adjacent_items(&input, x, y);
            let item = input[y][x];
            let is_smallest = adj_items.iter().all(|adj_item| match adj_item {
                Some(num) => item < *num,
                None => true,
            });
            if is_smallest {
                low_points.push(item);
            }
        }
    }

    low_points.iter().map(|&point| point + 1).sum()
}

fn parse_input() -> Vec<Vec<i32>> {
    let s = std::fs::read_to_string("src/day9/input.txt")
        .expect("ERROR: src/day9/input.txt is not found");
    s.split('\n')
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn get_adjacent_items(arr: &[Vec<i32>], x: usize, y: usize) -> [Option<i32>; 4] {
    let left = x.checked_sub(1).map(|pos| arr[y][pos]);
    let above = y.checked_sub(1).map(|pos| arr[pos][x]);
    let right = arr[y].get(x + 1).copied();
    let below = arr.get(y + 1).map(|row| row[x]);
    [left, above, right, below]
}
