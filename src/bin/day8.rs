#![feature(str_split_once)]
use std::collections::HashSet;
use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("unknown operation: ")]
    UnknownOperation,
    #[error("argument contains illegal value")]
    InvalidArgument,
    #[error("No whitespace in instruction")]
    MissingWhitespace,
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Accumulator(i32),
    Jump(i32),
    NoOp,
}
impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, argument) = s.split_once(' ').ok_or(Error::MissingWhitespace)?;
        let argument = argument.parse::<i32>().map_err(|_|Error::InvalidArgument)?;
        match operation {
            "acc" => Ok(Instruction::Accumulator(argument)),
            "jmp" => Ok(Instruction::Jump(argument)),
            "nop" => Ok(Instruction::NoOp),
            _ => Err(Error::UnknownOperation),
        }
    }
}

fn accumulate(count: i32, index: usize, instructions: &Vec<Instruction>, executions: &mut HashSet<usize>) -> i32 {
    if executions.contains(&index) {
        return count;
    }
    let mut count = count;
    let instruction = &instructions[index];
    let next_instruction_index = match instruction {
        Instruction::Accumulator(arg) => {
            count += arg;
            index as i32 + 1
        },
        Instruction::Jump(arg) => index as i32 + arg,
        Instruction::NoOp => index as i32 + 1,
    };
    let next_instruction_index = (next_instruction_index % instructions.len() as i32 ) as usize;
    executions.insert(index);
    // println!("{}, {}, {}, {:?}, {:?}", count, index, next_instruction_index, instruction, executions);
    accumulate(count, next_instruction_index, instructions, executions)
}

fn solver_part1(input: Vec<String>) -> String {
    let instructions: Vec<Instruction>  = input
        .iter()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .collect();
    let mut executions = HashSet::new();
    let solution = accumulate(0, 0, &instructions, &mut executions);
    
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

puzzle_tests!("5", "8");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instruction_from() {
        assert_eq!(Ok(Instruction::Jump(5)), "jmp +5".parse::<Instruction>());
        assert_eq!(Ok(Instruction::Jump(-5)), "jmp -5".parse::<Instruction>());
        assert_eq!(Ok(Instruction::NoOp), "nop +5".parse::<Instruction>());
        assert_eq!(Ok(Instruction::Accumulator(-5)), "acc -5".parse::<Instruction>());
    }

    #[test]
    fn instruction_from_error() {
        assert_eq!(Err(Error::MissingWhitespace), "jmp+5".parse::<Instruction>());
        assert_eq!(Err(Error::InvalidArgument), "jmp x".parse::<Instruction>());
        assert_eq!(Err(Error::UnknownOperation), "xxx +5".parse::<Instruction>());
    }

    #[test]
    fn accumulate_test() {
        let instructions = vec![
            Instruction::Jump(3),
            Instruction::Accumulator(2), 
            Instruction::Accumulator(3), 
            Instruction::Jump(-2),
            Instruction::Accumulator(10),
        ];
        let mut executions = HashSet::new();
        
        assert_eq!(5, accumulate(0, 0, &instructions, &mut executions));
    }
}