use aoc_2020_rust::Puzzle;

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

fn main() {
    let day = env!("CARGO_BIN_NAME");
    println!("Advent of Code 2020 - {}", day);
    let puzzle = Puzzle::new(day);
    let solution_part1 = puzzle.solve(solver_part1);
    println!("{} - Part 1 Solution:", day);
    println!("{}", solution_part1);
    let solution_part2 = puzzle.solve(solver_part2);
    println!("{} - Part 2 Solution:", day);
    println!("{}", solution_part2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_part1_test() {
        let day = env!("CARGO_BIN_NAME");
        let puzzle = Puzzle::new(day);
        assert_eq!(puzzle.test(solver_part1), "30".to_string());
    }
    
    #[test]
    fn puzzle_part2_test() {
        let day = env!("CARGO_BIN_NAME");
        let puzzle = Puzzle::new(day);
        assert_eq!(puzzle.test(solver_part2), "9".to_string());
    }
}