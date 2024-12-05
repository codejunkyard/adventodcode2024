use _utils_::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/2/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;
    let data = parse_input(&input);
    let count = count_safe_reports(&data);

    println!("{count}");

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut rows: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace() // Split the line into parts
            .map(|part| part.parse::<i32>().unwrap()) // Parse each part as i32
            .collect(); // Collect into a Vec<i32>

        rows.push(row); // Push the parsed row into rows
    }

    rows
}

fn count_safe_reports(input: &Vec<Vec<i32>>) -> i32 {
    fn is_valid_report(row: &Vec<i32>) -> bool {
        if row.len() == 1 {
            return true;
        }

        let increasing = row.windows(2).all(|pair| pair[1] > pair[0]);
        let decreasing = row.windows(2).all(|pair| pair[1] < pair[0]);

        if !increasing && !decreasing {
            return false;
        }

        row.windows(2).all(|pair| {
            let diff = (pair[1] - pair[0]).abs();
            diff >= 1 && diff <= 3
        })
    }

    fn adjust_problem_dampener(row: &Vec<i32>) -> bool {
        for i in 0..row.len() {
            let vec: Vec<i32> = row
                .iter()
                .enumerate()
                .filter_map(
                    |(index, &value)| {
                        if index != i {
                            Some(value)
                        } else {
                            None
                        }
                    },
                )
                .collect();

            if is_valid_report(&vec) {
                return true;
            }
        }

        false
    }

    let mut count = 0;

    for vec in input {
        if is_valid_report(vec) || adjust_problem_dampener(vec) {
            count += 1;
        }
    }

    count
}
