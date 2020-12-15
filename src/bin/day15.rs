use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use std::collections::HashMap;

fn find_last_spoken_numbers(number: u32, previous_numbers: &HashMap<u32, Vec<u32>>) -> u32 {
    match previous_numbers.get(&number) {
        Some(turns) => {
            let mut last_turns = turns.iter();
            let last_turn = last_turns.next_back();
            let second_last_turn = last_turns.next_back();
            if last_turn.is_some() && second_last_turn.is_some() {
                return last_turn.unwrap() - second_last_turn.unwrap();
            }
            0
        }
        None => 0
    }
}

fn find_last_spoken_numbers_tuple(number: u32, previous_numbers: &HashMap<u32, (Option<u32>, Option<u32>)>) -> u32 {
    match previous_numbers.get(&number) {
        Some((second_last_turn, last_turn)) => {
            if last_turn.is_some() && second_last_turn.is_some() {
                return last_turn.unwrap() - second_last_turn.unwrap();
            }
            0
        }
        None => 0
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let input: Vec<u32> = input[0]
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let mut previous_numbers: HashMap<u32, Vec<u32>> = HashMap::new();
    for (turn, number) in input.iter().enumerate() {
        previous_numbers.insert(*number, vec![turn as u32 + 1]);
    }
    let mut turn: u32 = previous_numbers.len() as u32 + 1;
    let mut number = find_last_spoken_numbers(*input.iter().last().expect("empty input"), &previous_numbers);
    while turn < 2020 {
        previous_numbers
            .entry(number)
            .and_modify(|turns| turns.push(turn))
            .or_insert(vec![turn]);
        number = find_last_spoken_numbers(number, &previous_numbers);
        turn += 1;
    }
    let solution = number;
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let input: Vec<u32> = input[0]
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let mut previous_numbers: HashMap<u32, (Option<u32>, Option<u32>)> = HashMap::new();
    for (turn, number) in input.iter().enumerate() {
        previous_numbers.insert(*number, (None, Some(turn as u32 + 1)));
    }
    let mut turn: u32 = previous_numbers.len() as u32 + 1;
    let mut number = find_last_spoken_numbers_tuple(*input.iter().last().expect("empty input"), &previous_numbers);
    while turn < 30000000 {
        previous_numbers
            .entry(number)
            .and_modify(|previous_turns| {
                *previous_turns = (previous_turns.1, Some(turn));
            })
            .or_insert((None, Some(turn)));
        number = find_last_spoken_numbers_tuple(number, &previous_numbers);
        turn += 1;
    }
    let solution = number;
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("436", "175594");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_last_spoken_numbers_no_previous_number() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, vec![1,4,5]);
        assert_eq!(find_last_spoken_numbers(1, &previous_numbers), 0);
    }

    #[test]
    fn find_last_spoken_numbers_too_few_previous_numbers() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, vec![1,4,5]);
        previous_numbers.insert(3, vec![5]);
        assert_eq!(find_last_spoken_numbers(3, &previous_numbers), 0);
    }

    #[test]
    fn find_last_spoken_numbers_found_previous_numbers() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, vec![1,4,5]);
        previous_numbers.insert(3, vec![5]);
        assert_eq!(find_last_spoken_numbers(2, &previous_numbers), 1);
    }

    #[test]
    fn find_last_spoken_numbers_tuple_no_previous_number() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, (Some(4), Some(5)));
        assert_eq!(find_last_spoken_numbers_tuple(1, &previous_numbers), 0);
    }

    #[test]
    fn find_last_spoken_numbers_tuple_too_few_previous_numbers() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, (Some(4), Some(5)));
        previous_numbers.insert(3, (None, Some(5)));
        assert_eq!(find_last_spoken_numbers_tuple(3, &previous_numbers), 0);
    }

    #[test]
    fn find_last_spoken_numbers_tuple_found_previous_numbers() {
        let mut previous_numbers = HashMap::new();
        previous_numbers.insert(2, (Some(4), Some(5)));
        previous_numbers.insert(3, (None, Some(5)));
        assert_eq!(find_last_spoken_numbers_tuple(2, &previous_numbers), 1);
    }

    #[test]
    fn solver_part1_example1() {
        assert_eq!(solver_part1(vec!["1,3,2".to_string()]), 1.to_string());
    }

    #[test]
    fn solver_part1_example2() {
        assert_eq!(solver_part1(vec!["2,1,3".to_string()]), 10.to_string());
    }

    #[test]
    fn solver_part1_example3() {
        assert_eq!(solver_part1(vec!["1,2,3".to_string()]), 27.to_string());
    }

    #[test]
    fn solver_part2_example1() {
        assert_eq!(solver_part2(vec!["1,3,2".to_string()]), 2578.to_string());
    }

    #[test]
    fn solver_part2_example2() {
        assert_eq!(solver_part2(vec!["2,1,3".to_string()]), 3544142.to_string());
    }

    #[test]
    fn solver_part2_example3() {
        assert_eq!(solver_part2(vec!["1,2,3".to_string()]), 261214.to_string());
    }

}