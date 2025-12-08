use std::{fs, panic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RollOrNot {
    Roll,
    NotRoll,
}

fn read_input(input: &str) -> Vec<Vec<RollOrNot>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => RollOrNot::NotRoll,
                    '@' => RollOrNot::Roll,
                    _ => {
                        println!("Invalid input: {}", c);
                        panic!("Invalid input")
                    }
                })
                .collect()
        })
        .collect()
}

fn is_oob(row: i64, col: i64, map: &Vec<Vec<RollOrNot>>) -> bool {
    row < 0 || col < 0 || row >= map.len() as i64 || col >= map[0].len() as i64
}

fn is_accessible(map: &Vec<Vec<RollOrNot>>, row: i64, col: i64) -> bool {
    if map[row as usize][col as usize] == RollOrNot::NotRoll {
        return false;
    }
    let pos_to_check: Vec<(i64, i64)> = vec![
        (row + 1, col),
        (row, col + 1),
        (row - 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col - 1),
        (row + 1, col + 1),
        (row - 1, col),
        (row, col - 1),
    ];
    let num_around: u64 = pos_to_check
        .iter()
        .map(|&(r, c)| {
            if is_oob(r, c, map) {
                return 0;
            }
            return match map[r as usize][c as usize] {
                RollOrNot::Roll => 1,
                RollOrNot::NotRoll => 0,
            };
        })
        .sum();
    return num_around < 4;
}

fn get_accessible_pairs(map: &Vec<Vec<RollOrNot>>) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    for (r, row) in map.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if is_accessible(map, r as i64, c as i64) {
                accessible.push((r, c));
            }
        }
    }
    accessible
}

fn solve_part1(input: &str) -> u64 {
    let map = read_input(input);
    // println!("Map size: {}x{}", map.len(), map[0].len());
    // println!("Map:");
    // print_map(&map);

    get_accessible_pairs(&map).len() as u64
}

fn solve_part2(input: &str) -> u64 {
    let mut map = read_input(input);
    let mut new_accessible: Vec<(usize, usize)>;
    let mut counter = 0;
    loop {
        new_accessible = get_accessible_pairs(&map);
        if new_accessible.is_empty() {
            break;
        }
        counter += new_accessible.len();
        for (r, c) in new_accessible {
            map[r][c] = RollOrNot::NotRoll;
        }
    }
    counter as u64
}

pub fn main() {
    let input = fs::read_to_string("src/input/day4.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 4:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
