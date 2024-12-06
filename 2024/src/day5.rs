use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/5/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    // // Convert input to Vec<Vec<char>>
    // let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // let count_part_1 = get_part_1(&grid);
    // println!("Part 1: Total count: {}", count_part_1);

    // let count_part_2 = get_part_2(&grid);
    // println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}
