#![feature(str_split_once)]
use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("unknown instruction")]
    UnknownInstruction,
    #[error("argument contains illegal value")]
    InvalidArgument,
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    MemWrite {
        adress: usize,
        value: u64,
      },
}
impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask ") {
            let (_, mask_value) = s.split_once(" = ").unwrap();
            return Ok(Instruction::Mask(mask_value.to_string()));
        }
        else if s.starts_with("mem") {
            let (adress, value) = s.split_once(" = ").unwrap();
            let value = value.parse::<u64>().map_err(|_| Error::InvalidArgument)?;
            let adress = &adress[4..adress.len()-1]; // mem[123]
            let adress = adress.parse::<usize>().map_err(|_| Error::InvalidArgument)?;
            return Ok(Instruction::MemWrite {
                adress,
                value,
            });
        }
        Err(Error::UnknownInstruction)
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let instructions: Vec<Instruction> = input
        .into_iter()
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect();

    let mut current_mask = "X".repeat(36).to_string();
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(value) => current_mask = value,
            Instruction::MemWrite { adress, value } => {
                let mut binary_value: Vec<char> = format!("{:#038b}", value)[2..].chars().collect();
                for (i, mask_bit_rule) in current_mask.chars().enumerate() {
                    match mask_bit_rule {
                        '0' => binary_value[i] = '0',
                        '1' => binary_value[i] = '1',
                        'X' => {},
                        _ => println!("encountered strange mask char"),
                    }
                }
                let value = u64::from_str_radix(binary_value.iter().collect::<String>().as_str(), 2).expect("error parsing bits");
                memory.insert(adress, value); 
            },
        }
    }
    let solution: u64 = memory.values().sum();
    solution.to_string()
}

fn calculate_floating_adresses(floating_adress: Vec<char>) -> Vec<u64> {
    let mut floating_adresses = vec![];
    let floating_adress_spaces: Vec<usize> = floating_adress
        .iter()
        .enumerate()
        .filter(|(_, bit_mask)| bit_mask == &&'X')
        .map(|(i, _)| i)
        .collect();
    let permutations = 2u32.pow(floating_adress_spaces.len() as u32);
    for adress_permutation in 0..permutations {
        let permutation_bits: Vec<char> = format!("{:#038b}", adress_permutation)[(38 - floating_adress_spaces.len())..]
            .chars()
            .collect();
        let mut adress = floating_adress.clone();
        for (adress_index, permuted_bit) in floating_adress_spaces.iter().zip(permutation_bits.iter()) {
            adress[*adress_index] = *permuted_bit;
        }
        let adress = u64::from_str_radix(adress.iter().collect::<String>().as_str(), 2).expect("error parsing bits");
        floating_adresses.push(adress);
    }
    floating_adresses
}

fn solver_part2(input: Vec<String>) -> String {
    let instructions: Vec<Instruction> = input
        .into_iter()
        .filter_map(|line| line.parse::<Instruction>().ok())
        .collect();

    let mut current_mask = "0".repeat(36).to_string();
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(value) => current_mask = value,
            Instruction::MemWrite { adress, value } => {
                let mut binary_adress: Vec<char> = format!("{:#038b}", adress)[2..].chars().collect();
                for (i, mask_bit_rule) in current_mask.chars().enumerate() {
                    match mask_bit_rule {
                        '0' => {},
                        '1' => binary_adress[i] = '1',
                        'X' => binary_adress[i] = 'X',
                        _ => println!("encountered strange mask char"),
                    }
                }

                for floating_adress in calculate_floating_adresses(binary_adress) {
                    memory.insert(floating_adress, value);
                }
            },
        }
    }
    let solution: u64 = memory.values().sum();
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("51", "208");


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(calculate_floating_adresses("000000000000000000000000000000X1101X".chars().collect::<Vec<char>>()), vec![26,27,58,59])
    }

    #[test]
    fn test2() {
        assert_eq!(calculate_floating_adresses("00000000000000000000000000000001X0XX".chars().collect::<Vec<char>>()), vec![16,17,18,19,24,25,26,27])
    }
}