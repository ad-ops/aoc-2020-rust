pub struct Puzzle<'a> {
    day: &'a str,
    pre_processor: fn(String) -> String,
}
impl<'a> Puzzle<'a> {
    pub fn new(day: &'a str, pre_processor: fn(String) -> String) -> Self {
        Puzzle {
            day,
            pre_processor,
        }
    }

    pub fn solve(&self, solver: fn(Vec<String>) -> String) -> String {
        let input = get_input(DayInput::Input(self.day), self.pre_processor);
        solver(input)
    }

    pub fn test(&self, solver: fn(Vec<String>) -> String) -> String {
        let input = get_input(DayInput::Test(self.day), self.pre_processor);
        solver(input)
    }
}

pub enum DayInput<'a> {
    Input(&'a str),
    Test(&'a str),
}

pub fn get_input(day: DayInput, pre_processor: fn(String) -> String) -> Vec<String> {
    let path = match day {
        DayInput::Input(d) => format!("input/{}_input.txt", d),
        DayInput::Test(d) => format!("input/{}_test.txt", d),
    };
    let input = std::fs::read_to_string(path)
        .expect("Could not find file!");
    let lines = pre_processor(input)
        .lines()
        .map(|l| l.to_string() )
        .collect::<Vec<String>>();
    lines
}

#[macro_export]
macro_rules! puzzle_main {
    ($solver_part1:expr, $solver_part2:expr, $preprocess:tt) => {
        fn main() {
            let day = env!("CARGO_BIN_NAME");
            println!("Advent of Code 2020 - {}", day);
            let puzzle = Puzzle::new(day, $preprocess);
            let part1_start = std::time::Instant::now();
            let solution_part1 = puzzle.solve($solver_part1);
            let part1_time = part1_start.elapsed().as_micros();
            let part2_start = std::time::Instant::now();
            let solution_part2 = puzzle.solve($solver_part2);
            let part2_time = part2_start.elapsed().as_micros();
            println!("{} - Part 1 solution took {}µs", day, part1_time);
            println!("{}", solution_part1);
            println!("{} - Part 2 solution took {}µs", day, part2_time);
            println!("{}", solution_part2);
        }
    };
    ($solver_part1:expr, $solver_part2:expr) => {
        fn main() {
            let day = env!("CARGO_BIN_NAME");
            println!("Advent of Code 2020 - {}", day);
            let puzzle = Puzzle::new(day, |x| x);
            let part1_start = std::time::Instant::now();
            let solution_part1 = puzzle.solve($solver_part1);
            let part1_time = part1_start.elapsed().as_micros();
            let part2_start = std::time::Instant::now();
            let solution_part2 = puzzle.solve($solver_part2);
            let part2_time = part2_start.elapsed().as_micros();
            println!("{} - Part 1 solution took {}µs", day, part1_time);
            println!("{}", solution_part1);
            println!("{} - Part 2 solution took {}µs", day, part2_time);
            println!("{}", solution_part2);
        }
    };
}

#[macro_export]
macro_rules! puzzle_tests {
    ($expected_part1:expr, $expected_part2:expr, $preprocess:tt) => {
        #[cfg(test)]
        mod aoc_test {
            use super::*;

            #[test]
            fn puzzle_part1_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day, $preprocess);
                assert_eq!(puzzle.test(solver_part1), $expected_part1);
            }

            #[test]
            fn puzzle_part2_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day, $preprocess);
                assert_eq!(puzzle.test(solver_part2), $expected_part2);
            }
        }
    };
    ($expected_part1:expr, $expected_part2:expr) => {
        #[cfg(test)]
        mod aoc_test {
            use super::*;

            #[test]
            fn puzzle_part1_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day, |x| x);
                assert_eq!(puzzle.test(solver_part1), $expected_part1,);
            }

            #[test]
            fn puzzle_part2_test() {
                let day = env!("CARGO_BIN_NAME");
                let puzzle = Puzzle::new(day, |x| x);
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
        let puzzle = Puzzle::new("day0", |x| x);
        assert_eq!("15".to_string(), puzzle.solve(solver));
    }

    #[test]
    fn puzzle_test() {
        let puzzle = Puzzle::new("day0", |x| x);
        assert_eq!("30".to_string(), puzzle.test(solver));
    }
}
