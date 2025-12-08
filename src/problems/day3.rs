use std::fs;

fn get_max_with_index(ints: Vec<u64>) -> (usize, u64) {
    ints.iter()
        .enumerate()
        .fold((0, ints[0]), |(max_index, max_value), (index, value)| {
            if *value > max_value {
                (index, *value)
            } else {
                (max_index, max_value)
            }
        })
}

fn get_max_iters(line: &str, digits: u64) -> u64 {
    let ints = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();
    let ints_len = ints.len();
    let mut start_idx = 0;
    let mut places: Vec<u64> = Vec::new();
    for i in 0..digits {
        // let last_idx = ints_len - (digits - i + 1) as usize;
        let last_idx = ints_len - (digits - i - 1) as usize;
        let (max_index, max_value) =
            get_max_with_index(ints.get(start_idx..last_idx).unwrap().to_vec());
        start_idx += max_index + 1;
        places.push(max_value)
    }
    places.iter().fold(0, |acc, &place| acc * 10 + place)
}

fn solve_part1(input: &str) -> u64 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| get_max_iters(line, 2))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| get_max_iters(line, 12))
        .sum()
}

pub fn main() {
    let input = fs::read_to_string("src/input/day3.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 2:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
