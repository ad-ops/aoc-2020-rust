use aoc_2020_rust::{puzzle_main, Puzzle, puzzle_tests};

fn solver_part1(input: Vec<String>) -> String {
    let parsed: Vec<i32> =  input
        .iter()
        .filter_map(|f| f.parse::<i32>().ok())
        .collect();
    let mut solution = 0;
    'outer: for x in parsed.iter() {
        for y in parsed.iter() {
            if x + y == 2020 {
                solution = x * y;
                break 'outer;
            }
        }
    }
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let parsed: Vec<i32> =  input
        .iter()
        .filter_map(|f| f.parse::<i32>().ok())
        .collect();
    let mut solution = 0;
    'outer: for x in parsed.iter() {
        for y in parsed.iter() {
            for z in parsed.iter() {
                if x + y + z == 2020 {
                    solution = x * y * z;
                    break 'outer;
                }
            }
        }
    }
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("514579", "241861950");