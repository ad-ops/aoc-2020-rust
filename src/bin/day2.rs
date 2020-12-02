#[macro_use]
extern crate lazy_static;

use regex::Regex;
use aoc_2020_rust::Puzzle;

#[derive(Debug)]
struct Constraint {
    min: usize,
    max: usize,
    constraint_char: char,
}
#[derive(Debug)]
struct Password {
    password: String,
    constraint: Constraint,
}
impl Password {
    fn new(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let password = &caps[4];
        let min = &caps[1];
        let max = &caps[2];
        let constraint_char = &caps[3];

        Password {
            password: password.to_string(),
            constraint: Constraint {
                min: min.parse::<usize>().unwrap(),
                max: max.parse::<usize>().unwrap(),
                constraint_char: constraint_char
                    .chars()
                    .nth(0)
                    .unwrap(),
            }
        }
    }

    fn valid_part1(&self) -> bool {
        let num_of_constraint_char = self.password
            .chars()
            .filter(|c| c == &self.constraint.constraint_char)
            .count();
        num_of_constraint_char <= self.constraint.max && num_of_constraint_char >= self.constraint.min
    }

    fn valid_part2(&self) -> bool {
        let password_chars: Vec<char> = self.password.chars().collect();
        if self.constraint.min < 1 || self.constraint.max < 1 {
            return false;
        }
        let first_pos = self.constraint.min - 1;
        let second_pos = self.constraint.max - 1;
        if password_chars.len() < self.constraint.min || password_chars.len() < self.constraint.max {
            return false;
        }

        println!("{:?}", self);
        let first_position_matches = password_chars[first_pos] == self.constraint.constraint_char;
        let second_position_matches = password_chars[second_pos] == self.constraint.constraint_char;

        // iff single match then password is valid 
        !(first_position_matches && second_position_matches) && 
        (first_position_matches || second_position_matches)
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let solution: usize = input
        .iter()
        .map(|l| Password::new(l))
        .filter(|p| p.valid_part1())
        .count();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let solution: usize = input
        .iter()
        .map(|l| Password::new(l))
        .filter(|p| p.valid_part2())
        .count();
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
        assert_eq!(puzzle.test(solver_part1), "2".to_string());
    }
    
    #[test]
    fn puzzle_part2_test() {
        let day = env!("CARGO_BIN_NAME");
        let puzzle = Puzzle::new(day);
        assert_eq!(puzzle.test(solver_part2), "1".to_string());
    }
}