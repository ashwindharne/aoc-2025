use std::fs;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn solve_part1(input: &str) -> u64 {
    let lines = input.split('\n');
    let grid = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let operations = grid
        .last()
        .unwrap()
        .iter()
        .map(|op| match *op {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Invalid operator: {}", op),
        })
        .collect::<Vec<Operator>>();

    let grid_excluding_ops = grid.get(..grid.len() - 1).unwrap();
    let transposed = (0..grid_excluding_ops[0].len())
        .map(|col| {
            grid_excluding_ops
                .iter()
                .map(|row| row[col].parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let zipped = operations
        .iter()
        .zip(transposed.iter())
        .fold(0, |acc, (op, col)| {
            let mut sum = col[0];
            for num in col.iter().skip(1) {
                match op {
                    Operator::Add => sum += num,
                    Operator::Multiply => sum *= num,
                }
            }
            return acc + sum;
        });
    zipped
}

fn solve_part2(_input: &str) -> u64 {
    0
}

pub fn main() {
    let input = fs::read_to_string("src/input/day6.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 6:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
