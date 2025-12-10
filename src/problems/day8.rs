use std::collections::{BTreeMap, HashMap};
use std::fs;

use disjoint::DisjointSetVec;
use itertools::Itertools;

fn read_vertices(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|c| c.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn squared_euclidean_distance(a: &(u64, u64, u64), b: &(u64, u64, u64)) -> u64 {
    a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
}

fn solve_part1(input: &str) -> u64 {
    let vertices = read_vertices(input);
    let mut dist_map = BTreeMap::new();
    for (i, a) in vertices.iter().enumerate() {
        for (j, b) in vertices.iter().enumerate() {
            if i != j {
                let dist = squared_euclidean_distance(a, b);
                dist_map.insert(dist, (i, j));
            }
        }
    }
    let mut iters = 0;
    let mut vertex_sets = DisjointSetVec::from(vertices.clone());
    for (_, (i, j)) in dist_map.iter() {
        iters += 1;
        vertex_sets.join(*i, *j);
        if iters == 1000 {
            break;
        }
    }
    let mut set_sizes = HashMap::new();
    for i in 0..vertex_sets.len() {
        let root = vertex_sets.root_of(i);
        *set_sizes.entry(root).or_insert(0) += 1;
    }

    set_sizes
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(3)
        .map(|(_, count)| count)
        .product()
}

fn solve_part2(input: &str) -> u64 {
    let vertices = read_vertices(input);
    let mut dist_map = BTreeMap::new();
    for (i, a) in vertices.iter().enumerate() {
        for (j, b) in vertices.iter().enumerate() {
            if i != j {
                let dist = squared_euclidean_distance(a, b);
                dist_map.insert(dist, (i, j));
            }
        }
    }
    let mut vertex_sets = DisjointSetVec::from(vertices.clone());
    let mut last_joined_x: (u64, u64) = (0, 0);
    for (_, (i, j)) in dist_map.iter() {
        if vertex_sets.join(*i, *j) {
            last_joined_x = (vertices[*i].0, vertices[*j].0);
        }
    }
    last_joined_x.0 * last_joined_x.1
}

pub fn main() {
    let input = fs::read_to_string("src/input/day8.txt").expect("Failed to read input file");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("Day 8:");
    println!("  Part 1: {}", part1);
    println!("  Part 2: {}", part2);
}
