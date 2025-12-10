use std::fs;

#[derive(Debug, Copy, Clone)]
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

fn solve_part2(_input: &str) -> u128 {
    let lines = _input.split('\n').filter(|line| !line.is_empty());
    let longest_line_len = lines
        .clone()
        .map(|line| line.chars().count())
        .max()
        .unwrap();
    let num_lines = lines.clone().count();
    let operations = lines.clone().last().unwrap();
    let mut op_index_slices: Vec<(usize, usize, Operator)> = Vec::new();
    let mut last_index = 0;
    let mut last_op = match operations.chars().next() {
        Some('+') => Operator::Add,
        Some('*') => Operator::Multiply,
        _ => panic!("Invalid operator"),
    };
    for (i, op) in operations.chars().enumerate().skip(1) {
        match op {
            '+' => {
                op_index_slices.push((last_index, i - 2, last_op));
                last_op = Operator::Add;
                last_index = i;
            }
            '*' => {
                op_index_slices.push((last_index, i - 2, last_op));
                last_op = Operator::Multiply;
                last_index = i;
            }
            _ => {}
        }
    }
    op_index_slices.push((last_index, longest_line_len - 1, last_op));

    let mut transposed_problems: Vec<(Operator, Vec<String>)> = op_index_slices
        .clone()
        .into_iter()
        .map(|(start, end, op)| (op, vec![String::new(); end - start + 1]))
        .collect();
    lines.take(num_lines - 1).for_each(|line| {
        for (j, (start, end, _)) in op_index_slices.clone().into_iter().enumerate() {
            for (operate_index, char_index) in (start..end + 1).enumerate() {
                if let Some(char_result) = line.chars().nth(char_index) {
                    transposed_problems[j].1[operate_index].push(char_result);
                }
            }
        }
    });
    transposed_problems
        .into_iter()
        .map(|(op, values)| match op {
            Operator::Add => values
                .into_iter()
                .map(|val| val.trim().parse::<u128>().unwrap())
                .sum::<u128>(),
            Operator::Multiply => values
                .into_iter()
                .map(|val| val.trim().parse::<u128>().unwrap())
                .product::<u128>(),
        })
        .sum::<u128>()
}

pub fn main() {
    let input = fs::read_to_string("src/input/day6.txt").expect("Failed to read input file");

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 6:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
