use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn count_trees(input: &[String], right: usize, down: usize) -> usize {
    let solution: usize = input
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(i, _)| i % down == 0)
        .map(|(i, l)| 
            ( i, l.chars().nth((i / down * right) % l.len()).unwrap() )
        )
        .filter(|(_, c)| c == &'#')
        .count();
    solution
}

fn solver_part1(input: Vec<String>) -> String {
    count_trees(&input, 3, 1).to_string()
}


fn solver_part2(input: Vec<String>) -> String {
    let solution = 
        count_trees(&input, 1, 1) *
        count_trees(&input, 3, 1) *
        count_trees(&input, 5, 1) *
        count_trees(&input, 7, 1) *
        count_trees(&input, 1, 2);
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("7", "336");