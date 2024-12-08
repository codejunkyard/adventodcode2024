use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

//const DEBUG: bool = true;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/7/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;
    let dataset: Vec<Row> = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            let mut parts = line.split(':'); // Split into key and values
            let key = parts.next().unwrap().trim().parse::<i64>().unwrap();
            let value = parts
                .next()
                .unwrap()
                .split_whitespace() // Split the values by whitespace
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            Row { key, value }
        })
        .collect();

    let sum_part_1 = get_part_1(&dataset);
    println!("Part 1: Total sum: {}", sum_part_1);

    let sum_part_2 = get_part_2(&dataset);
    println!("Part 2: Total sum: {}", sum_part_2);

    Ok(())
}

fn get_part_1(dataset: &Vec<Row>) -> i64 {
    let mut sum = 0;

    fn evaluate_combinations(nums: &Vec<i32>, target: i64) -> bool {
        fn helper(nums: &[i32], current_result: i64, idx: usize, target: i64) -> bool {
            if idx == nums.len() {
                return current_result == target;
            }

            // Try addition
            if helper(nums, current_result + nums[idx] as i64, idx + 1, target) {
                return true;
            }

            // Try multiplication
            if helper(nums, current_result * nums[idx] as i64, idx + 1, target) {
                return true;
            }

            false
        }

        if nums.is_empty() {
            return false;
        }

        // Start recursion from the first number
        helper(&nums, nums[0] as i64, 1, target)
    }

    for row in dataset {
        if evaluate_combinations(&row.value, row.key) {
            sum += row.key;
        }
    }

    sum
}

fn get_part_2(dataset: &Vec<Row>) -> i64 {
    let mut sum = 0;

    fn evaluate_combinations(nums: &Vec<i32>, target: i64) -> bool {
        fn helper(nums: &[i32], current_result: String, idx: usize, target: i64) -> bool {
            if idx == nums.len() {
                // Convert the concatenated string result back to i64 and compare with target
                return current_result.parse::<i64>().unwrap_or(0) == target;
            }

            // Try addition
            if helper(
                nums,
                (current_result.parse::<i64>().unwrap_or(0) + nums[idx] as i64).to_string(),
                idx + 1,
                target,
            ) {
                return true;
            }

            // Try multiplication
            if helper(
                nums,
                (current_result.parse::<i64>().unwrap_or(0) * nums[idx] as i64).to_string(),
                idx + 1,
                target,
            ) {
                return true;
            }

            // Try concatenation
            let concatenated_result = current_result.clone() + &nums[idx].to_string();
            if helper(nums, concatenated_result, idx + 1, target) {
                return true;
            }

            false
        }

        if nums.is_empty() {
            return false;
        }

        // Start recursion from the first number
        helper(&nums, nums[0].to_string(), 1, target)
    }

    for row in dataset {
        if evaluate_combinations(&row.value, row.key) {
            sum += row.key;
        }
    }

    sum
}

struct Row {
    key: i64,
    value: Vec<i32>,
}
