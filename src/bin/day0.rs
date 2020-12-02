use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn solver_part1(input: Vec<String>) -> String {
    let solution: i32 = input
        .into_iter()
        .filter_map(|l| l.parse::<i32>().ok())
        .sum();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let solution: i32 = input
        .into_iter()
        .filter_map(|l| l.parse::<i32>().ok())
        .max()
        .unwrap_or(-1);
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("30", "9");