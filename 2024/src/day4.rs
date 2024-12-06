use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

const WORD: &str = "XMAS";

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/4/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    // Convert input to Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count_part_1 = get_part_1(&grid);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&grid);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    // Iterate over row indices
    for row_index in 0..grid.len() {
        // Iterate over column indices for each row
        for col_index in 0..grid[row_index].len() {
            count += count_xmas(&grid, row_index, col_index);
        }
    }

    count
}

fn get_part_2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    // Iterate over row indices
    for row_index in 0..grid.len() {
        // Iterate over column indices for each row
        for col_index in 0..grid[row_index].len() {
            if count_x_mas(&grid, row_index, col_index) {
                count += 1;
            }
        }
    }

    count
}

fn count_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut count = 0;
    let word_length = WORD.len();

    if grid[row][col] != WORD.chars().nth(0).unwrap() {
        // check if this char is 'X'
        return 0;
    } // base case, not 'X'

    let inbound_n = || row >= word_length - 1;
    let inbound_e = || col <= grid[row].len() - word_length;
    let inbound_s = || row <= grid.len() - word_length;
    let inbound_w = || col >= word_length - 1;

    // N: North
    if inbound_n() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row - n][col]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // NE: North-East
    if inbound_n() && inbound_e() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row - n][col + n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // E: East
    if inbound_e() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row][col + n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // SE: South-East
    if inbound_e() && inbound_s() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row + n][col + n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // S: South
    if inbound_s() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row + n][col]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // SW: South-West
    if inbound_s() && inbound_w() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row + n][col - n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // W: West
    if inbound_w() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row][col - n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    // WN: West-North
    if inbound_w() && inbound_n() {
        let mut result = String::from("X");
        for n in 1..word_length {
            result.push(grid[row - n][col - n]);
        }

        if result == WORD {
            count += 1;
        }
    }

    count
}

fn count_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    // not a generic solution, but much easier to solve
    if grid[row][col] != 'A' {
        // check if this char is 'A'
        return false;
    } // base case, not 'A'

    // find A, get 4 corners
    // if 2 'S' and 2 'M' and not on same diagonal, count += 1

    //SMMS
    //SSMM
    //MSSM
    //MMSS

    // Four characters to compare
    let coordinates: [(isize, isize); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];
    let mut characters = Vec::new();

    for &(dx, dy) in &coordinates {
        // Cast row and col to isize for arithmetic
        let new_row = row as isize + dy;
        let new_col = col as isize + dx;

        // Ensure the indices are within bounds (after casting back to usize)
        if new_row >= 0
            && new_col >= 0
            && (new_row as usize) < grid.len()
            && (new_col as usize) < grid[0].len()
        {
            characters.push(grid[new_row as usize][new_col as usize]);
        }
    }

    // Collect the characters into a String
    let collected: String = characters.iter().collect();

    // Words to compare against
    let words = vec!["SMMS", "SSMM", "MSSM", "MMSS"];

    // Check if the collected string matches one of the words
    if words.contains(&collected.as_str()) {
        return true;
    }

    false
}
