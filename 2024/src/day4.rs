use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    println!("Solving Day 4...");

    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/4/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    // Convert input to Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &col) in row.iter().enumerate() {
            if col == 'X' {
                count += count_xmas(&grid, row_index, col_index);
            }
        }
    }

    println!("{}", count);

    Ok(())
}

fn count_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    println!("{}:{},{}", grid[row][col], row, col);

    0
}
