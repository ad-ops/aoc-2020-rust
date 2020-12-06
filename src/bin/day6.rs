use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::collections::{HashMap, HashSet};

fn solver_part1(input: Vec<String>) -> String {
    let solution: usize = input
        .split(|l| l.is_empty())
        .map(|g| 
            g
                .join("")
                .chars()
                .collect::<HashSet<char>>()
        )
        .map(|s| s.len())
        .sum();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let solution: usize = input
        .split(|l| l.is_empty())
        .map(|g| {
            let num_of_people = g.len();
            let answers = g
                .join("")
                .chars()
                .collect::<Vec<char>>();
            (num_of_people, answers)
        })
        .map(|(p, a)| {
            let mut counts = HashMap::new();
            for c in a {
                *counts.entry(c).or_insert(0) += 1;
            }
            (p, counts)
        })
        .map(|(p, h)| 
            h
                .into_iter()
                .filter(|c| c.1 == p)
                .collect::<HashMap<char, usize>>()
        )
        .map(|s| s.len())
        .sum();
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("11", "6");