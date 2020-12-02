use aoc_2020_rust::{puzzle_tests, Puzzle, puzzle_main};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{min}-{max} {constraint_char}: {password}")]
struct Password {
  min: usize,
  max: usize,
  constraint_char: char,
  password: String,
}

impl Password {
    fn valid_part1(&self) -> bool {
        let num_of_constraint_char = self.password
            .chars()
            .filter(|c| c == &self.constraint_char)
            .count();

        num_of_constraint_char <= self.max && num_of_constraint_char >= self.min
    }

    fn valid_part2(&self) -> bool {
        let password_chars: Vec<char> = self.password.chars().collect();
        if self.min < 1 || self.max < 1 {
            return false;
        }
        let first_pos = self.min - 1;
        let second_pos = self.max - 1;
        if password_chars.len() < self.min || password_chars.len() < self.max {
            return false;
        }
        let first_position_matches = password_chars[first_pos] == self.constraint_char;
        let second_position_matches = password_chars[second_pos] == self.constraint_char;

        // iff single match then password is valid 
        !(first_position_matches && second_position_matches) && 
        (first_position_matches || second_position_matches)
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let solution: usize = input
        .iter()
        .filter_map(|l| l.parse::<Password>().ok())
        .filter(|p| p.valid_part1())
        .count();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let solution: usize = input
        .iter()
        .filter_map(|l| l.parse::<Password>().ok())
        .filter(|p| p.valid_part2())
        .count();
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("2", "1");
