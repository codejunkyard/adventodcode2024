use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/1/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    // Parse the input into two vectors
    let (list1, list2) = parse_input(&input);

    let mut sum = 0;

    let mut similarity_score = 0;

    // Iterate over the vectors by index
    for i in 0..list1.len() {
        sum += (list1[i] - list2[i]).abs();
    }

    println!("Part 1: Total distance: {sum}");

    for i in list1.iter() {
        let count = list2.iter().filter(|&n| *n == *i).count();
        similarity_score += count as i32 * *i; // Convert count to i32 and dereference i
    }

    println!("Part 2: Similarity score: {similarity_score}");

    Ok(())
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    // Initialize the two vectors to hold the two lists
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        // Slpit the line into two parts
        let mut parts = line.split_whitespace();

        // Parse the first and second value from the line
        if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
            // Push the parsed values to the respective vectors
            if let (Ok(first_num), Ok(second_num)) = (first.parse::<i32>(), second.parse::<i32>()) {
                list1.push(first_num);
                list2.push(second_num);
            }
        }
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}
