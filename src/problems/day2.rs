use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/input/day2.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 2:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}

#[derive(Debug)]
struct IdRange {
    min: i64,
    max: i64,
}

fn is_invalid(id: i64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }
    let (front, back) = id_str.split_at(id_str.len() / 2);
    return front == back;
}

fn is_invalid_new(id: i64) -> bool {
    let id_str = id.to_string();
    let mut candidate_patterns: Vec<String> = Vec::new();
    for i in 1..((id_str.len() / 2) + 1) {
        candidate_patterns.push(id_str[..i].to_string());
    }
    for pattern in candidate_patterns {
        if id_str.len() % pattern.len() != 0 {
            continue;
        }
        let mut valid = true;
        for substr in id_str
            .as_bytes()
            .chunks(pattern.len())
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
        {
            if substr != pattern {
                valid = false;
                break;
            }
        }
        if valid {
            return true;
        }
    }
    false
}

fn evaluate_range(range: IdRange, evaluator: fn(i64) -> bool) -> Vec<i64> {
    let mut result = Vec::new();
    for i in range.min..=range.max {
        if evaluator(i) {
            result.push(i);
        }
    }
    result
}

fn solve_part1(input: &str) -> String {
    let sum = input
        .split(',')
        .filter(|range| !range.is_empty())
        .map(|range| {
            let min_max_vec = range
                .trim()
                .split('-')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>();
            IdRange {
                min: min_max_vec[0],
                max: min_max_vec[1],
            }
        })
        .map(|range| evaluate_range(range, is_invalid))
        .flatten()
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>();

    sum.to_string()
}

fn solve_part2(input: &str) -> String {
    let sum = input
        .split(',')
        .filter(|range| !range.is_empty())
        .map(|range| {
            let min_max_vec = range
                .trim()
                .split('-')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>();
            IdRange {
                min: min_max_vec[0],
                max: min_max_vec[1],
            }
        })
        .map(|range| evaluate_range(range, is_invalid_new))
        .flatten()
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(solve_part1(input), "expected");
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(solve_part2(input), "expected");
    }
}
