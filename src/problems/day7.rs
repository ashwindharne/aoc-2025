use std::collections::HashSet;
use std::fs;

struct TachyonManifold {
    start_index: usize,
    splitter_positions: Vec<Vec<usize>>,
    width: usize,
}

fn read_input(input: &str) -> TachyonManifold {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let start_index = lines.first().unwrap().find("S").unwrap();
    let splitter_positions = lines
        .iter()
        .step_by(2)
        .map(|line| line.match_indices("^").map(|(index, _)| index).collect())
        .collect();
    TachyonManifold {
        start_index,
        splitter_positions,
        width: lines.first().unwrap().len(),
    }
}

fn evolve_beam_positions(
    beams: Vec<usize>,
    splitters: Vec<usize>,
    width: usize,
) -> (Vec<usize>, u64) {
    // convert splitters to hashset
    let splitter_indices = splitters.iter().map(|&pos| pos).collect::<HashSet<_>>();
    let mut new_beams = HashSet::new();
    let mut split_counter = 0;
    for beam in beams {
        if splitter_indices.contains(&beam) {
            split_counter += 1;
            if beam + 1 < width {
                new_beams.insert(beam + 1);
            }
            if (beam as i64 - 1) >= 0 {
                new_beams.insert(beam - 1);
            }
        } else {
            new_beams.insert(beam);
        }
    }
    (new_beams.into_iter().collect(), split_counter)
}

fn evolve_beam_positions_with_memoization(
    beams: Vec<usize>,
    splitters: Vec<usize>,
    width: usize,
    memo: &Vec<usize>,
) -> (Vec<usize>, Vec<usize>) {
    let splitter_indices = splitters.iter().map(|&pos| pos).collect::<HashSet<_>>();
    let mut new_beams = HashSet::new();
    let mut new_memo = vec![0; width];
    for beam in beams {
        if splitter_indices.contains(&beam) {
            if beam + 1 < width {
                let new_place = beam + 1;
                new_beams.insert(new_place);
                new_memo[new_place] += memo[beam];
            }
            if (beam as i64 - 1) >= 0 {
                let new_place = beam - 1;
                new_beams.insert(new_place);
                new_memo[new_place] += memo[beam];
            }
        } else {
            new_beams.insert(beam);
            new_memo[beam] += memo[beam];
        }
    }
    (new_beams.into_iter().collect(), new_memo)
}

fn solve_part1(input: &str) -> u64 {
    let manifold = read_input(input);
    let mut beams = Vec::new();
    let mut split_counter = 0;
    beams.push(manifold.start_index);
    for splitters in manifold.splitter_positions {
        let result = evolve_beam_positions(
            beams.into_iter().collect(),
            splitters.clone(),
            manifold.width,
        );
        beams = result.0;
        split_counter += result.1;
    }

    split_counter
}

fn solve_part2(input: &str) -> u64 {
    let manifold = read_input(input);
    let mut beams = Vec::new();
    let mut memo = vec![0; manifold.width];
    beams.push(manifold.start_index);
    memo[manifold.start_index] = 1;
    for splitters in manifold.splitter_positions {
        let result = evolve_beam_positions_with_memoization(
            beams.clone().into_iter().collect(),
            splitters.clone(),
            manifold.width,
            &memo,
        );
        beams = result.0;
        memo = result.1;
    }
    // sum memo values into u64
    memo.iter().map(|&x| x as u64).sum()
}

pub fn main() {
    let input = fs::read_to_string("src/input/day7.txt").expect("Failed to read input file");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 7:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
