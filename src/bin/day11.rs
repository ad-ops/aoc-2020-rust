use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};

const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

fn find_neighbours(x: usize, y: usize, layout: &Vec<Vec<char>>) -> Vec<char> {
    let mut neighbours = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; } // center
            let y = y as i32 + i;
            let x = x as i32 + j;
            if x < 0 || y < 0  { continue; } // low out of bounds
            if x as usize > layout[0].len() - 1 || y as usize > layout.len() - 1 { continue; } // high out of bounds
            let neighbour = layout[y as usize][x as usize];
            neighbours.push(neighbour);
        }
    }

    neighbours
}

fn find_visiable_seats(x: usize, y: usize, layout: &Vec<Vec<char>>) -> Vec<char> {
    let max_x = layout[0].len() as i32 - 1;
    let max_y = layout.len() as i32 - 1;
    let mut neighbours = vec![];

    // Column
    for i in (-max_y..0).rev() {
        let y = y as i32 + i;
        if y < 0  { continue; } // low out of bounds
        if y > max_y { continue; } // high out of bounds
        let neighbour = layout[y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }
    for i in 1..=max_y {
        let y = y as i32 + i;
        if y < 0  { continue; } // low out of bounds
        if y > max_y { continue; } // high out of bounds
        let neighbour = layout[y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }

    // Row
    for i in (-max_x..0).rev() {
        let x = x as i32 + i;
        if x < 0  { continue; } // low out of bounds
        if x > max_x { continue; } // high out of bounds
        let neighbour = layout[y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }
    for i in 1..=max_x {
        let x = x as i32 + i;
        if x < 0  { continue; } // low out of bounds
        if x > max_x { continue; } // high out of bounds
        let neighbour = layout[y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }

    // North-East
    for i in (-max_x..0).rev() {
        let x = x as i32 + i;
        let ne_y = y as i32 + i;
        if x < 0 || ne_y < 0  { continue; } // low out of bounds
        if x > max_x || ne_y > max_y { continue; } // high out of bounds
        let neighbour = layout[ne_y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }
    for i in 1..=max_x {
        let x = x as i32 + i;
        let ne_y = y as i32 + i;
        if x < 0 || ne_y < 0  { continue; } // low out of bounds
        if x > max_x || ne_y > max_y { continue; } // high out of bounds
        let neighbour = layout[ne_y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }

    // South-West
    for i in (-max_x..0).rev() {
        let x = x as i32 + i;
        let sw_y = y as i32 - i;
        if x < 0 || sw_y < 0  { continue; } // low out of bounds
        if x > max_x || sw_y > max_y { continue; } // high out of bounds
        let neighbour = layout[sw_y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }
    for i in 1..=max_x {
        let x = x as i32 + i;
        let sw_y = y as i32 - i;
        if x < 0 || sw_y < 0  { continue; } // low out of bounds
        if x > max_x || sw_y > max_y { continue; } // high out of bounds
        let neighbour = layout[sw_y as usize][x as usize];
        if neighbour == OCCUPIED || neighbour == EMPTY {
            neighbours.push(neighbour);
            break;
        }
    }

    neighbours
}

fn change_seating(x: usize, y: usize, layout: &Vec<Vec<char>>, tolerance: usize) -> char {
    let tile = layout[y][x];
    if tile == FLOOR {
        return tile;
    }
    let num_occupied_neighbours = find_neighbours(x, y, layout)
        .iter()
        .filter(|seat| seat == &&OCCUPIED)
        .count();
    
    match (tile, num_occupied_neighbours) {
        (EMPTY, 0) => OCCUPIED,
        (EMPTY, _) => EMPTY,
        (OCCUPIED, n) if n >= tolerance => EMPTY,
        (OCCUPIED, _) => OCCUPIED,
        (t, _) => t,
    }
}

fn mutate_seating(layout: &Vec<Vec<char>>, tolerance: usize) -> Vec<Vec<char>> {
    layout
        .iter()
        .enumerate()
        .map(|(row, seats)| {
            seats
                .iter()
                .enumerate()
                .map(|(column, _)| change_seating(column, row, &layout, tolerance))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn change_visiable_seating(x: usize, y: usize, layout: &Vec<Vec<char>>, tolerance: usize) -> char {
    let tile = layout[y][x];
    if tile == FLOOR {
        return tile;
    }
    let num_occupied_neighbours = find_visiable_seats(x, y, layout)
        .iter()
        .filter(|seat| seat == &&OCCUPIED)
        .count();
    
    match (tile, num_occupied_neighbours) {
        (EMPTY, 0) => OCCUPIED,
        (EMPTY, _) => EMPTY,
        (OCCUPIED, n) if n >= tolerance => EMPTY,
        (OCCUPIED, _) => OCCUPIED,
        (t, _) => t,
    }
}

fn mutate_visiable_seating(layout: &Vec<Vec<char>>, tolerance: usize) -> Vec<Vec<char>> {
    layout
        .iter()
        .enumerate()
        .map(|(row, seats)| {
            seats
                .iter()
                .enumerate()
                .map(|(column, _)| change_visiable_seating(column, row, &layout, tolerance))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn solver_part1(input: Vec<String>) -> String {
    let layout: Vec<Vec<char>> = input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut seating = layout;
    let solution: usize = loop {
        let mutate = mutate_seating(&seating, 4);
        let seating_flatten = seating.iter().flatten().collect::<String>();
        let mutate_flatten = mutate.iter().flatten().collect::<String>();
        if seating_flatten == mutate_flatten {
            break seating_flatten.chars().filter(|seat| seat == &OCCUPIED).count();
        }
        seating = mutate;
    };

    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let layout: Vec<Vec<char>> = input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut seating = layout;
    let solution: usize = loop {
        let mutate = mutate_visiable_seating(&seating, 5);
        let seating_flatten = seating.iter().flatten().collect::<String>();
        let mutate_flatten = mutate.iter().flatten().collect::<String>();
        if seating_flatten == mutate_flatten {
            break seating_flatten.chars().filter(|seat| seat == &OCCUPIED).count();
        }
        seating = mutate;
    };

    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("37", "26");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_neighbours_low_bounds() {
        let layout = vec![
            vec!['.','L','.','.','.',],
            vec!['L','L','.','.','.',],
            vec!['.','.','.','.','.',],
            vec!['.','.','.','.','.',],
        ];

        assert_eq!("LLL", find_neighbours(0, 0, &layout).iter().collect::<String>());
    }

    #[test]
    fn find_neighbours_high_bounds() {
        let layout = vec![
            vec!['.','.','.','.','.',],
            vec!['.','.','.','.','.',],
            vec!['.','.','.','L','L',],
            vec!['.','.','.','L','.',],
        ];

        assert_eq!("LLL", find_neighbours(4, 3, &layout).iter().collect::<String>());
    }

    #[test]
    fn find_neighbours_out_of_bounds() {
        let layout = vec![
            vec!['.','.','.','.','.',],
            vec!['.','.','.','.','.',],
            vec!['.','.','.','L','L',],
            vec!['.','.','.','L','.',],
        ];

        assert_eq!("", find_neighbours(10, 10, &layout).iter().collect::<String>());
    }

    #[test]
    fn find_neighbours_middle() {
        let layout = vec![
            vec!['.','.','.','.','.',],
            vec!['.','.','#','.','.',],
            vec!['.','.','#','L','L',],
            vec!['.','.','#','L','.',],
        ];

        assert_eq!("#..#L#L.", find_neighbours(3, 2, &layout).iter().collect::<String>());
    }


    #[test]
    fn find_visiable_seats_test() {
        let layout = vec![
            vec!['.','.','.','.','.','.','.','#','.'],
            vec!['.','.','.','#','.','.','.','.','.'],
            vec!['.','#','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','#','L','.','.','.','.','#'],
            vec!['.','.','.','.','#','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['#','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','#','.','.','.','.','.'],
        ];

        assert_eq!("########", find_visiable_seats(3, 4, &layout).iter().collect::<String>());
    }
}