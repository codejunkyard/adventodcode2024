use advent_lib::fetch_input;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::error::Error;

const DEBUG: bool = true;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/5/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    let sum_part_1 = get_part_1(&input);
    println!("Part 1: Total sum: {}", sum_part_1);

    let sum_part_2 = get_part_2(&input);
    println!("Part 2: Total sum: {}", sum_part_2);

    Ok(())
}

fn get_part_1(input: &str) -> i32 {
    let mut sum = 0;

    // Destructure the two parts into `rules_input` and `updates_input`
    if let [rules_input, updates_input] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        // Initialize an empty HashMap to store the data for rules
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

        // Parse the first part (rules_input) into the `rules` HashMap
        for line in rules_input.trim().lines() {
            // Split the line by '|', and directly parse the parts
            if let [key, value] = line.split('|').collect::<Vec<&str>>()[..] {
                if let (Ok(key), Ok(value)) = (key.parse(), value.parse()) {
                    rules.entry(key).or_insert_with(Vec::new).push(value);
                } else {
                    eprintln!("Error parsing numbers in pair: {}", line);
                }
            }
        }

        if DEBUG {
            // Print the map to see the result
            for (key, values) in &rules {
                println!("{}: {:?}", key, values);
            }
        }

        // Parse the second part (updates_input) into a Vec<Vec<i32>> for pages
        let updates: Vec<Vec<i32>> = updates_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        if DEBUG {
            println!("");
            // Print the updates to verify
            for update in &updates {
                println!("{:?}", update);
            }
        }

        for update in updates {
            let mut correct = true;
            let mut clone = update.clone();

            while clone.len() > 1 {
                let page = clone.pop().unwrap(); // Extract the last page element
                let rule: Option<&Vec<i32>> = rules.get(&page);

                if let Some(rule_vec) = rule {
                    if clone.iter().any(|&x| rule_vec.contains(&x)) {
                        correct = false;
                        break;
                    }
                }
            }

            if correct {
                let middle = update[update.len() / 2];
                sum += middle;
            }
        }
    } else {
        eprintln!("Input format is incorrect: expected two parts separated by a double newline.");
    }

    sum
}

fn get_part_2(input: &str) -> i32 {
    let mut sum = 0;

    // Destructure the two parts into `rules_input` and `updates_input`
    if let [rules_input, updates_input] = input.split("\n\n").collect::<Vec<&str>>()[..] {
        // Initialize an empty HashMap to store the data for rules
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

        // Parse the first part (rules_input) into the `rules` HashMap
        for line in rules_input.trim().lines() {
            // Split the line by '|', and directly parse the parts
            if let [key, value] = line.split('|').collect::<Vec<&str>>()[..] {
                if let (Ok(key), Ok(value)) = (key.parse(), value.parse()) {
                    rules.entry(key).or_insert_with(Vec::new).push(value);
                } else {
                    eprintln!("Error parsing numbers in pair: {}", line);
                }
            }
        }

        if DEBUG {
            // Print the map to see the result
            for (key, values) in &rules {
                println!("{}: {:?}", key, values);
            }
        }

        // Parse the second part (updates_input) into a Vec<Vec<i32>> for pages
        let updates: Vec<Vec<i32>> = updates_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|x| x.trim().parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        if DEBUG {
            println!("");
            // Print the updates to verify
            for update in &updates {
                println!("{:?}", update);
            }
        }

        for update in updates {
            let mut correct = true;
            let mut clone = update.clone();

            while clone.len() > 1 {
                let page = clone.pop().unwrap(); // Extract the last page element
                let rule: Option<&Vec<i32>> = rules.get(&page);

                if let Some(rule_vec) = rule {
                    if clone.iter().any(|&x| rule_vec.contains(&x)) {
                        correct = false;
                        break;
                    }
                }
            }

            if correct {
                let middle = update[update.len() / 2];
                sum += middle;
            }
        }
    } else {
        eprintln!("Input format is incorrect: expected two parts separated by a double newline.");
    }

    sum
}
