use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn solver_part1(input: Vec<String>) -> String {
    let mut adapters: Vec<u32> = input
        .into_iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect();
    adapters.push(0);
    adapters.sort_unstable();

    let mut adapters = adapters.iter().peekable();
    let mut jolt_differences: Vec<u32> = vec![];
    loop {
        let current = adapters.next();
        let next = adapters.peek();
        if current.is_none() || next.is_none() {
            break;
        }

        let current = current.unwrap();
        let next = next.unwrap();
        jolt_differences.push(*next - current);
    }
    let (jolt_1_diff, jolt_3_diff): (Vec<u32>, Vec<u32>) = jolt_differences
        .iter()
        .filter(|jolt_diff| jolt_diff == &&1 || jolt_diff == &&3)
        .partition(|jolt_diff| jolt_diff == &&1);

    let solution = jolt_1_diff.len()  * (jolt_3_diff.len() + 1);
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let mut adapters: Vec<u32> = input
        .into_iter()
        .filter_map(|l| l.parse::<u32>().ok())
        .collect();
    adapters.push(0);
    adapters.sort_unstable();

    let mut adapters = adapters.iter().peekable();
    let mut jolt_differences: Vec<u32> = vec![];
    loop {
        let current = adapters.next();
        let next = adapters.peek();
        if current.is_none() || next.is_none() {
            break;
        }

        let current = current.unwrap();
        let next = next.unwrap();
        jolt_differences.push(*next - current);
    }
    let solution: usize = jolt_differences
        .split(|jolt| jolt == &3)
        .filter(|x| !x.is_empty())
        .map(|ones| {
            let flexible_ones = ones.len() - 1;
            // ways to move adapters grows as triangular numbers. https://en.wikipedia.org/wiki/Triangular_number
            ( flexible_ones * (flexible_ones + 1) ) / 2 + 1 
        })
        .product();
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("220", "19208");