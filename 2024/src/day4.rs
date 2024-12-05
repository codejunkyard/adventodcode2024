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

    //let grid: Vec<Vec<char>> = input.

    Ok(())
}
