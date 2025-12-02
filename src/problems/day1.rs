use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/input/day1.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 1:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
}

fn rotate(initial_value: i32, direction: Direction, value: i32) -> i32 {
    let actual_value = value % 100;
    match direction {
        Direction::L => {
            let newval = initial_value - actual_value;
            if newval < 0 { 100 + newval } else { newval }
        }
        Direction::R => (initial_value + value) % 100,
    }
}

fn rotate_count_clicks(initial_value: i32, direction: Direction, value: i32) -> (i32, i32) {
    let mod_value = value / 100;
    let actual_value = value % 100;
    let starts_at_0 = if initial_value == 0 { -1 } else { 0 };
    match direction {
        Direction::L => {
            let newval = initial_value - actual_value;
            if newval < 0 {
                (100 + newval, mod_value + 1 + starts_at_0)
            } else if newval == 0 {
                (newval, mod_value + 1 + starts_at_0)
            } else {
                (newval, mod_value + starts_at_0)
            }
        }
        Direction::R => {
            let newval = initial_value + actual_value;
            if newval >= 100 {
                (newval - 100, mod_value + 1 + starts_at_0)
            } else {
                (newval, mod_value)
            }
        }
    }
}

fn parse_line(line: &str) -> (Direction, i32) {
    let dir = line.chars().next().unwrap();
    let value: i32 = line
        .chars()
        .skip(1)
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .expect("Invalid value");

    let direction = match dir {
        'L' => Direction::L,
        'R' => Direction::R,
        _ => panic!("Invalid direction"),
    };
    (direction, value)
}

fn solve_part1(input: &str) -> String {
    // TODO: Implement part 1
    let lines: Vec<(Direction, i32)> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect();

    let mut val = 50;
    let mut zero_counter = 0;
    for (dir, value) in lines {
        val = rotate(val, dir, value);
        if val == 0 {
            zero_counter += 1;
        }
    }
    zero_counter.to_string()
}

fn solve_part2(_input: &str) -> String {
    let lines: Vec<(Direction, i32)> = _input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect();

    // TODO: Implement part 2
    let mut val = 50;
    let mut click_counter = 0;
    for (dir, value) in lines {
        let (new_val, clicks) = rotate_count_clicks(val, dir, value);
        val = new_val;
        click_counter += clicks;
    }
    click_counter.to_string()
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
