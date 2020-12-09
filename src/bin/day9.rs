use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn invalid_preamble(index: usize, preamble: usize, numbers: &Vec<u64>) -> Option<u64> {
    if index >= numbers.len() || preamble >= numbers.len() || preamble > index {
        return None;
    }

    let number = numbers[index];
    let mut invalid_number = Some(number);
    'outer: for (i, potential_first) in numbers[(index-preamble)..index].iter().enumerate() {
        for (j, potential_second) in numbers[(index-preamble)..index].iter().enumerate() {
            // println!("{}, {}, {}, {}, {}", i, j, potential_first, potential_second, potential_first + potential_second == number);
            if i == j { continue; }
            if potential_first + potential_second == number {
                invalid_number = None;
                break 'outer;
            }
        }
    }

    invalid_number
}

fn solver_part1(input: Vec<String>) -> String {
    let numbers: Vec<u64> = input
        .into_iter()
        .filter_map(|l| l.parse::<u64>().ok())
        .collect();
    let solution: &Option<u64> = &numbers
        .iter()
        .enumerate()
        .filter_map(|(i, number)| invalid_preamble(i, 25, &numbers))
        .nth(0);
    solution.unwrap().to_string()
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

puzzle_tests!("20", "x");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_preamble_found() {
        let numbers = vec![1, 2, 2, 5, 8, 13];
        assert_eq!(Some(5), invalid_preamble(3, 3, &numbers));
    }

    #[test]
    fn invalid_preamble_not_found() {
        let numbers = vec![1, 2, 4, 5, 8, 13];
        assert_eq!(None, invalid_preamble(3, 3, &numbers));
    }
}