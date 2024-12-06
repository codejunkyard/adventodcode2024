use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

const WORD: &str = "XMAX";

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("Solving Day 4...");

    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/4/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
    //let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    // Convert input to Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    // Iterate over row indices
    for row_index in 0..grid.len() {
        // Iterate over column indices for each row
        for col_index in 0..grid[row_index].len() {
            println!("{},{}", row_index, col_index);
            count += count_xmas(&grid, row_index, col_index);
        }
    }

    println!("{}", count);

    Ok(())
}

fn count_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let mut count = 0;
    let word_length = WORD.len();

    if grid[row][col] != WORD.chars().nth(0).unwrap() {
        // check if this char is 'X'
        return 0;
    } // base case, not 'X'

    let inbound_n = || row >= word_length - 1;
    let inbound_e = || col < grid.len() - word_length + 1;
    let inbound_s = || row < grid.len() - word_length + 1;
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
