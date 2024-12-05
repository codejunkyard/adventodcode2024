use dotenv::dotenv;
use regex::Regex;
use reqwest::blocking::Client;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/3/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    let sum_part_1 = get_part_1(&input);
    println!("Part 1: Total sum of products: {}", sum_part_1);

    let sum_part_2 = get_part_2(&input);
    println!("Part 2: Total sum of products: {}", sum_part_2);

    Ok(())
}

fn fetch_input(url: &str, session_token: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_token))
        .header("User-Agent", "my-rust-app")
        .send()?;

    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("Failed to fetch input: HTTP {}", response.status()).into())
    }
}

fn get_part_1(input: &str) -> i32 {
    // Regex pattern to match mul(ddd, ddd) where ddd is 1 to 3 digits
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0; // Variable to accumulate the sum of products

    // Iterate through all matches
    for cap in re.captures_iter(&input) {
        // Parse the captured numbers from the regex groups
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();

        // Multiply the two numbers and add to the sum
        sum += num1 * num2;
    }

    sum
}

fn get_part_2(input: &str) -> i32 {
    let lines: Vec<&str> = input.split("do()").collect();
    let processed_lines: String = lines
        .iter()
        .map(|line| {
            if let Some(pos) = line.find("don't()") {
                line[..pos].to_string() // Chop after the word "don't()"
            } else {
                line.to_string() // Keep the line as-is if "don't()" is not found
            }
        })
        .collect::<Vec<String>>()
        .concat();

    get_part_1(&processed_lines)
}
