#[macro_use]
extern crate lazy_static;

use regex::Regex;
use aoc_2020_rust::{puzzle_tests, Puzzle, puzzle_main};

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

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("2", "1");
