use aoc_2020_rust::Puzzle;

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
        assert_eq!(puzzle.test(solver_part1), "514579".to_string());
    }
    
    #[test]
    fn puzzle_part2_test() {
        let day = env!("CARGO_BIN_NAME");
        let puzzle = Puzzle::new(day);
        assert_eq!(puzzle.test(solver_part2), "241861950".to_string());
    }
}