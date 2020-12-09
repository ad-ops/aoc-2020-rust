use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn invalid_preamble(index: usize, preamble: usize, numbers: &Vec<u64>) -> bool {
    if index >= numbers.len() || preamble >= numbers.len() || preamble > index {
        return false;
    }

    let number = numbers[index];
    let mut invalid_number = true;
    'outer: for (i, potential_first) in numbers[(index-preamble)..index].iter().enumerate() {
        for (j, potential_second) in numbers[(index-preamble)..index].iter().enumerate() {
            if i == j { continue; }
            if potential_first + potential_second == number {
                invalid_number = false;
                break 'outer;
            }
        }
    }

    invalid_number
}

fn find_encryption_weakness(illegal_value: u64, numbers: &[u64]) -> Option<u64> {
    for (i, _) in numbers.iter().enumerate() {
        for (j, _) in numbers.iter().enumerate() {
            let length: i64 = j as i64 - i as i64;
            if length < 2  { continue; }

            let length = length as usize;
            let slice_sum: u64 = numbers.iter().skip(i).take(length).sum();
            if slice_sum == illegal_value {
                let min = numbers.iter().skip(i).take(length).min().unwrap();
                let max = numbers.iter().skip(i).take(length).max().unwrap();
                return Some(min + max);
            }
        }
    }
    None
}

fn solver_part1(input: Vec<String>) -> String {
    let numbers: Vec<u64> = input
        .into_iter()
        .filter_map(|l| l.parse::<u64>().ok())
        .collect();
    let illegal_value: &Option<(usize, &u64)> = &numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| invalid_preamble(*i, 25, &numbers))
        .nth(0);
    let (_, solution) = illegal_value.unwrap();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let numbers: Vec<u64> = input
        .into_iter()
        .filter_map(|l| l.parse::<u64>().ok())
        .collect();
    let illegal_value: &Option<(usize, &u64)> = &numbers
        .iter()
        .enumerate()
        .filter(|(i, _)| invalid_preamble(*i, 25, &numbers))
        .nth(0);
    let (index, illegal_number) = illegal_value.unwrap();
    let encyption_weakness: Option<u64> = find_encryption_weakness(*illegal_number, &numbers[0..index]);
    match encyption_weakness {
        Some(i) => i.to_string(),
        None => "Error!!!".to_string(),
    }
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("454", "356");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_preamble_found() {
        let numbers = vec![1, 2, 2, 5, 8, 13];
        assert_eq!(true, invalid_preamble(3, 3, &numbers));
    }

    #[test]
    fn invalid_preamble_not_found() {
        let numbers = vec![1, 2, 4, 5, 8, 13];
        assert_eq!(false, invalid_preamble(3, 3, &numbers));
    }

    #[test]
    fn find_encryption_weakness_found() {
        let numbers: Vec<u64> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
        ];
        assert_eq!(Some(62), find_encryption_weakness(127, &numbers[..]));
    }

    #[test]
    fn find_encryption_weakness_not_found() {
        let numbers: Vec<u64> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
        ];
        assert_eq!(None, find_encryption_weakness(128, &numbers[..]));
    }
    

}