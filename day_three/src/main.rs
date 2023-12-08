use std::fs::{self};

fn main() {
    let special_characters = ['/', '#', '+', '*', '=', '@', '&', '%', '-', '$'];

    let char_matrix: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap()
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut locations_visited: Vec<(usize, usize)> = vec![];
    let mut count: u32 = 0;
    
    for (row, line) in char_matrix.iter().enumerate() {
        for (column, &cell) in line.iter().enumerate() {
            if special_characters.iter().any(|&special_character| special_character == cell) {
                println!("{} at: ({},{})", cell, row, column);
                for i in -1..=1 {
                    for j in -1..=1 {
                        let adjacent_row = row as i32 + i;
                        let adjacent_column = column as i32 + j;

                        println!("Checking: ({}, {})", adjacent_row, adjacent_column);

                        if (i == 0 && j == 0) ||
                        locations_visited.iter().any(|location| location.0 as i32 == adjacent_row && location.1 as i32 == adjacent_column) ||
                        !char_matrix[adjacent_row as usize][adjacent_column as usize].is_digit(10) {
                            continue;
                        }
    
                        if adjacent_row < char_matrix.len() as i32 &&
                        adjacent_row >= 0 &&
                        adjacent_column < line.len() as i32 &&
                        adjacent_column >= 0 {
                            let mut number_string = String::new();
                            let mut current_column = adjacent_column + 1;
                            for &char in char_matrix[adjacent_row as usize].iter().skip(adjacent_column as usize) {
                                if char.is_digit(10) {
                                    locations_visited.push((adjacent_row as usize, current_column as usize));
                                    number_string.insert(number_string.len(), char);
                                    current_column = current_column + 1;
                                } else {
                                    break;
                                }
                            }
                            current_column = adjacent_column;
                            for &char in char_matrix[adjacent_row as usize].iter().rev().skip(char_matrix[adjacent_row as usize].len() - adjacent_column as usize) {
                                if char.is_digit(10) {
                                    locations_visited.push((adjacent_row as usize, current_column as usize));
                                    number_string.insert(0, char);
                                    current_column = current_column - 1;
                                } else {
                                    break;
                                }
                            }

                            println!("Found: {}", number_string);
                            count = count + number_string.parse::<u32>().unwrap();
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}