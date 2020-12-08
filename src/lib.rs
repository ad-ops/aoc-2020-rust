
pub struct Puzzle<'a> {
    day: &'a str,
}
impl<'a> Puzzle<'a> {
    // pub fn new(day: u8) -> Self {
    //     let days = format!("day{}", day);
    //     Puzzle {
    //         day: &days[..]
    //     }
    // }

    pub fn new(day: &'a str) -> Self {
        Puzzle {
            day
        }
    }

    pub fn solve(&self, solver: fn(Vec<String>) -> String) -> String {
        let input = get_input(DayInput::Input(self.day));
        solver(input)
    }

    pub fn test(&self, solver: fn(Vec<String>) -> String) -> String {
        let input = get_input(DayInput::Test(self.day));
        solver(input)
    }
}

pub enum DayInput<'a> {
    Input(&'a str),
    Test(&'a str),
}

pub fn get_input(day: DayInput) -> Vec<String> {
    let path = match day {
        DayInput::Input(d) => format!("input/{}_input.txt", d),
        DayInput::Test(d) => format!("input/{}_test.txt", d),
    };
    let lines = std::fs::read_to_string(path)
        .expect("Could not find file!")
        .lines()
        .map(|l| l.to_string() )
        .collect::<Vec<String>>();
    lines
}

#[macro_export]
macro_rules! puzzle_main {
    ($solver_part1:expr, $solver_part2:expr) => {
        fn main() {
            let day = env!("CARGO_BIN_NAME");
            println!("Advent of Code 2020 - {}", day);
            let puzzle = Puzzle::new(day);
            let solution_part1 = puzzle.solve($solver_part1);
            println!("{} - Part 1 Solution:", day);
            println!("{}", solution_part1);
            let solution_part2 = puzzle.solve($solver_part2);
            println!("{} - Part 2 Solution:", day);
            println!("{}", solution_part2);
        }
    };
}

#[macro_export]
macro_rules! puzzle_tests {
    ($expected_part1:expr, $expected_part2:expr) => {
        #[cfg(test)]
        mod aoc_test {
            use super::*;

            #[test]
            fn puzzle_part1_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day);
                assert_eq!(puzzle.test(solver_part1), $expected_part1);
            }

            #[test]
            fn puzzle_part2_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day);
                assert_eq!(puzzle.test(solver_part2), $expected_part2);
            }
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;

    fn solver(data_input: Vec<String>) -> String {
        let solution: i32 = data_input
            .into_iter()
            .filter_map(|l| l.parse::<i32>().ok())
            .sum();
        solution.to_string()
    }

    #[test]
    fn puzzle_solve() {
        let puzzle = Puzzle::new("day0");
        assert_eq!("15".to_string(), puzzle.solve(solver));
    }

    #[test]
    fn puzzle_test() {
        let puzzle = Puzzle::new("day0");
        assert_eq!("30".to_string(), puzzle.test(solver));
    }
}
