use std::fs;

struct IngredientDatabase {
    fresh: Vec<(u64, u64)>,
    list: Vec<u64>,
}

fn read_input(input: &str) -> IngredientDatabase {
    let mut fresh = Vec::new();
    let mut list = Vec::new();

    let lines: Vec<&str> = input.split('\n').collect();
    let empty_line_index = lines.iter().position(|line| line.is_empty()).unwrap();

    // Parse the fresh pairs (before empty line)
    for line in &lines[0..empty_line_index] {
        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() == 2 {
            let a = parts[0].parse::<u64>().unwrap();
            let b = parts[1].parse::<u64>().unwrap();
            fresh.push((a, b));
        }
    }
    fresh.sort_by(|(a, _), (c, _)| a.cmp(c));

    // Parse the list (after empty line)
    for line in &lines[(empty_line_index + 1)..] {
        if !line.is_empty() {
            let nums: Vec<u64> = line
                .split(',')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            list.extend(nums);
        }
    }
    list.sort();

    IngredientDatabase { fresh, list }
}

fn solve_part1(_input: &str) -> u64 {
    let db = read_input(_input);

    let mut counter: u64 = 0;
    for id in db.list {
        for (min, max) in db.fresh.iter() {
            if id >= *min && id <= *max {
                counter += 1;
                break;
            }
        }
    }
    // Implement part 1 solution here
    counter
}

fn solve_part2(_input: &str) -> u64 {
    let db = read_input(_input);

    let ranges = db.fresh;
    let mut merged_ranges: Vec<u64> = Vec::new();
    let mut current_range = ranges[0];

    for range in &ranges[1..] {
        if range.0 <= current_range.1 + 1 {
            current_range.1 = current_range.1.max(range.1);
        } else {
            merged_ranges.push(current_range.1 - current_range.0 + 1);
            current_range = *range;
        }
    }
    merged_ranges.push(current_range.1 - current_range.0 + 1);
    merged_ranges.iter().sum()
}

pub fn main() {
    let input = fs::read_to_string("src/input/day5_toy.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 4:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
