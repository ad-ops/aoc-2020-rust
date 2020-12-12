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

#[derive(PartialEq, Debug, Copy, Clone)]
enum Instruction {
    Accumulator(i32),
    Jump(i32),
    NoOp(i32),
}
impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, argument) = s.split_once(' ').ok_or(Error::MissingWhitespace)?;
        let argument = argument.parse::<i32>().map_err(|_|Error::InvalidArgument)?;
        match operation {
            "acc" => Ok(Instruction::Accumulator(argument)),
            "jmp" => Ok(Instruction::Jump(argument)),
            "nop" => Ok(Instruction::NoOp(argument)),
            _ => Err(Error::UnknownOperation),
        }
    }
}

fn accumulate(count: i32, index: usize, instructions: &[Instruction], executions: &mut HashSet<usize>) -> (i32, bool) {
    if executions.contains(&index) {
        return (count, false);
    }
    let mut count = count;
    let instruction = &instructions[index];
    let next_instruction_index = match instruction {
        Instruction::Accumulator(arg) => {
            count += arg;
            index as i32 + 1
        },
        Instruction::Jump(arg) => index as i32 + arg,
        Instruction::NoOp(_) => index as i32 + 1,
    };
    let next_instruction_index = (next_instruction_index % instructions.len() as i32 ) as usize;
    executions.insert(index);

    let final_terminating_instruction = match instruction {
        Instruction::Accumulator(_) => true,
        Instruction::Jump(1) => true,
        Instruction::Jump(_) => false,
        Instruction::NoOp(_) => true,
    };
    if index + 1 == instructions.len() && final_terminating_instruction {
        return (count, true);
    }

    accumulate(count, next_instruction_index, instructions, executions)
}

fn accumulate_with_fix(count: i32, index: usize, fix_index: usize, instructions: &[Instruction], executions: &mut HashSet<usize>) -> (i32, bool) {
    if executions.contains(&index) {
        return (count, false);
    }
    let mut count = count;
    let instruction = if index == fix_index {
        match instructions[index] {
            Instruction::Accumulator(a) => Instruction::Accumulator(a),
            Instruction::Jump(a) => Instruction::NoOp(a),
            Instruction::NoOp(a) => Instruction::Jump(a),
        }
    }
    else {
        instructions[index]
    };
    let next_instruction_index = match instruction {
        Instruction::Accumulator(arg) => {
            count += arg;
            index as i32 + 1
        },
        Instruction::Jump(arg) => index as i32 + arg,
        Instruction::NoOp(_) => index as i32 + 1,
    };
    let next_instruction_index = (next_instruction_index % instructions.len() as i32 ) as usize;
    executions.insert(index);

    let final_terminating_instruction = match instruction {
        Instruction::Accumulator(_) => true,
        Instruction::Jump(1) => true,
        Instruction::Jump(_) => false,
        Instruction::NoOp(_) => true,
    };
    if index + 1 == instructions.len() && final_terminating_instruction {
        return (count, true);
    }

    accumulate_with_fix(count, next_instruction_index, fix_index, instructions, executions)
}

fn solver_part1(input: Vec<String>) -> String {
    let instructions: Vec<Instruction>  = input
        .iter()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .collect();
    let mut executions = HashSet::new();
    let (solution, _) = accumulate(0, 0, &instructions, &mut executions);
    
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let instructions: Vec<Instruction>  = input
        .iter()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .collect();
    let mut solution = -1;
    for (index, _) in instructions.iter().enumerate() {
        let mut executions = HashSet::new();
        let (count, terminated) = accumulate_with_fix(0, 0, index, &instructions, &mut executions);
        if terminated {
            solution = count;
            break;
        }
    }
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
        assert_eq!(Ok(Instruction::NoOp(5)), "nop +5".parse::<Instruction>());
        assert_eq!(Ok(Instruction::Accumulator(-5)), "acc -5".parse::<Instruction>());
    }

    #[test]
    fn instruction_from_error() {
        assert_eq!(Err(Error::MissingWhitespace), "jmp+5".parse::<Instruction>());
        assert_eq!(Err(Error::InvalidArgument), "jmp x".parse::<Instruction>());
        assert_eq!(Err(Error::UnknownOperation), "xxx +5".parse::<Instruction>());
    }

    #[test]
    fn accumulate_corrupt() {
        let instructions = vec![
            Instruction::Jump(3),
            Instruction::Accumulator(2), 
            Instruction::Accumulator(3), 
            Instruction::Jump(-2),
            Instruction::Accumulator(10),
        ];
        let mut executions = HashSet::new();
        
        assert_eq!((5, false), accumulate(0, 0, &instructions, &mut executions));
    }

    #[test]
    fn accumulate_terminated() {
        let instructions = vec![
            Instruction::Jump(3),
            Instruction::Accumulator(2), 
            Instruction::Accumulator(3), 
            Instruction::NoOp(5),
            Instruction::Accumulator(10),
        ];
        let mut executions = HashSet::new();
        
        assert_eq!((10, true), accumulate(0, 0, &instructions, &mut executions));
    }

    #[test]
    fn accumulate_with_fix_corrupt() {
        let instructions = vec![
            Instruction::Jump(3),
            Instruction::Accumulator(2), 
            Instruction::Accumulator(3), 
            Instruction::Jump(-2),
            Instruction::Accumulator(10),
        ];
        let mut executions = HashSet::new();
        
        assert_eq!((5, false), accumulate_with_fix(0, 0, 0, &instructions, &mut executions));
    }

    #[test]
    fn accumulate_with_fix_terminated() {
        let instructions = vec![
            Instruction::Jump(3),
            Instruction::Accumulator(2), 
            Instruction::Accumulator(3), 
            Instruction::Jump(-2),
            Instruction::Accumulator(10),
        ];
        let mut executions = HashSet::new();
        
        assert_eq!((10, true), accumulate_with_fix(0, 0, 3, &instructions, &mut executions));
    }
}