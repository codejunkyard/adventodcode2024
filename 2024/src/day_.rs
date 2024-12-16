use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/10/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    let topographic_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    let count_part_1 = get_part_1(&topographic_map);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&topographic_map);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(topographic_map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    sum
}

fn get_part_2(topographic_map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    sum
}
