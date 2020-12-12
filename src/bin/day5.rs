use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

fn solver_part1(input: Vec<String>) -> String {
    let solution = input
        .into_iter()
        .map(|l| {
            let (row, column) = l.split_at(7);
            (row.replace('B', "1").replace('F', "0"), column.replace('R', "1").replace('L', "0"))
        })
        .map(|(r, c)| (u8::from_str_radix(r.as_str(), 2), u8::from_str_radix(c.as_str(), 2)))
        .filter(|(r, c)| r.is_ok() && c.is_ok())
        .map(|(r, c)| (r.unwrap(), c.unwrap()))
        .map(|(r, c)| r as i32 * 8 + c as i32)
        .max();
    solution.unwrap_or(-1).to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let mut seats: Vec<u32> = input
        .into_iter()
        .map(|l| {
            let (row, column) = l.split_at(7);
            (row.replace('B', "1").replace('F', "0"), column.replace('R', "1").replace('L', "0"))
        })
        .map(|(r, c)| (u8::from_str_radix(r.as_str(), 2), u8::from_str_radix(c.as_str(), 2)))
        .filter(|(r, c)| r.is_ok() && c.is_ok())
        .map(|(r, c)| (r.unwrap(), c.unwrap()))
        .map(|(r, c)| r as u32 * 8 + c as u32)
        .collect();
        seats.sort_unstable();

    let starting_seat_id = seats[0] as usize;
    let (_, neighbour) = seats
        .into_iter()
        .enumerate()
        .find(|(i, s)| i + starting_seat_id != *s as usize).unwrap();
    let solution = neighbour - 1;
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("820", "566"); // part2 is not "correct", but not good test data.