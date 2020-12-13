use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::collections::HashSet;

fn solver_part1(input: Vec<String>) -> String {
    let earliest_time = &input[0].parse::<u32>().unwrap();
    let bus_lines: Vec<u32> = input[1]
        .split(',')
        .filter(|line| line != &"x")
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();

    let mut departing_time = *earliest_time;
    let bus_line = 'outer: loop {
        for line in &bus_lines {
            if departing_time % line == 0 {
                break 'outer line;
            }
        }
        departing_time = departing_time + 1;
        if departing_time == earliest_time + 100 {break &0;}
    };

    let solution = (departing_time - earliest_time) * bus_line;
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let bus_lines: Vec<(usize, usize)> = input[1]
        .split(',')
        .enumerate()
        .filter(|(_, line)| line != &"x")
        .filter_map(|(i, line)| {
            if let Ok(l) = line.parse::<usize>() {
                return Some((i, l))
            }
            None
        })
        .collect();
    
    let mut departure_time = 0;
    let mut line_multipliers: HashSet<usize> = HashSet::new();
    line_multipliers.insert(1); // initilize with 1
    let solution = 'outer: loop {
        fn is_divisable(departure_time: usize, order: usize, line: usize) -> bool {
            (departure_time + order) % line == 0
        }
        
        // the final answer is a combination of lcd of the bus lines.
        // when finding a line which is divisable update the step for departure_time to avoid search all numbers.
        for (order, line) in bus_lines.iter() {
            if is_divisable(departure_time, *order, *line) {
                line_multipliers.insert(*line);
            }
        }

        if bus_lines.iter().all(|(order, line)| is_divisable(departure_time, *order, *line)) {
            break 'outer departure_time;
        }
        departure_time += line_multipliers.iter().product::<usize>();
    };

    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("295", "1068781");