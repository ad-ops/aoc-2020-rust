use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::{num::ParseIntError, str::FromStr, fmt::Display, f32::consts::PI};

#[derive(Debug)]
enum Instruction {
  North(u32),
  South(u32),
  East(u32),
  West(u32),
  RotateLeft(u32),
  RotateRight(u32),
  Forward(u32),
  Unknown,
}
impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.chars().nth(0).expect("Empty instruction input");
        let value = s[1..].parse::<u32>()?;
        Ok(match command {
            'N' => Instruction::North(value),
            'S' => Instruction::South(value),
            'E' => Instruction::East(value),
            'W' => Instruction::West(value),
            'L' => Instruction::RotateLeft(value),
            'R' => Instruction::RotateRight(value),
            'F' => Instruction::Forward(value),
            _ => Instruction::Unknown,
        })
    }
}

struct Waypoint {
    longitude: f32,
    latitude: f32,
}
impl Waypoint {
    fn new(longitude: f32, latitude: f32) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
}

struct Ship {
    longitude: f32,
    latitude: f32,
    facing: f32,
    waypoint: Waypoint,
}
impl Ship {
    fn new() -> Self {
        Self {
            longitude: 0.,
            latitude: 0.,
            facing: 90., // 0 degrees is North
            waypoint: Waypoint::new(10., 1.),
        }
    }

    fn perform_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(v) => self.latitude += v as f32,
            Instruction::South(v) => self.latitude -= v as f32,
            Instruction::East(v) => self.longitude += v as f32,
            Instruction::West(v) => self.longitude -= v as f32,
            Instruction::RotateLeft(d) => self.facing = self.facing - d as f32,
            Instruction::RotateRight(d) => self.facing = self.facing + d as f32,
            Instruction::Forward(v) => {
                let radian = (self.facing / 180.) * PI;
                let (dlong, dlat  ) = radian.sin_cos();
                self.longitude += dlong * v as f32;
                self.latitude += dlat * v as f32;
            }
            Instruction::Unknown => println!("Ignoring unknown instruction..."),
        }
    }

    fn perform_waypoint_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(v) => self.waypoint.latitude += v as f32,
            Instruction::South(v) => self.waypoint.latitude -= v as f32,
            Instruction::East(v) => self.waypoint.longitude += v as f32,
            Instruction::West(v) => self.waypoint.longitude -= v as f32,
            Instruction::RotateLeft(d) => {
                let radian = (d as f32 / 180.) * PI;
                let (sin, cos) = radian.sin_cos();
                let rotated_long = self.waypoint.longitude * cos - self.waypoint.latitude * sin;
                let rotated_lat = self.waypoint.longitude * sin + self.waypoint.latitude * cos;
                self.waypoint.longitude = rotated_long;
                self.waypoint.latitude = rotated_lat;
            },
            Instruction::RotateRight(d) => {
                let radian = -(d as f32 / 180.) * PI;
                let (sin, cos) = radian.sin_cos();
                let rotated_long = self.waypoint.longitude * cos - self.waypoint.latitude * sin;
                let rotated_lat = self.waypoint.longitude * sin + self.waypoint.latitude * cos;
                self.waypoint.longitude = rotated_long;
                self.waypoint.latitude = rotated_lat;
            },
            Instruction::Forward(v) => {
                self.longitude += self.waypoint.longitude * v as f32;
                self.latitude += self.waypoint.latitude * v as f32;
            }
            Instruction::Unknown => println!("Ignoring unknown instruction..."),
        };
    }
}
impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ship is at (long: {}, lat: {}) and facing {} degrees with waypoint at (long: {}, lat: {})", self.longitude, self.latitude, self.facing, self.waypoint.longitude, self.waypoint.latitude)
    }
    
}


fn solver_part1(input: Vec<String>) -> String {
    let mut ship = Ship::new();
    input
        .into_iter()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .for_each(|instruction| ship.perform_instruction(instruction));

    let solution = ( ship.longitude.round().abs() + ship.latitude.round().abs() ) as u32;
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let mut ship = Ship::new();
    input
        .into_iter()
        .filter_map(|l| l.parse::<Instruction>().ok())
        .for_each(|instruction| ship.perform_waypoint_instruction(instruction));

    let solution = ( ship.longitude.round().abs() + ship.latitude.round().abs() ) as u32;
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("25", "286");