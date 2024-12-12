use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::f64;
use std::time::Instant;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/11/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;
    //let input = "125 17";

    let rocks: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let rocks_32: Vec<u32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let depth = 25;

    // if depth <= 25 {
    //     let start = Instant::now();
    //     let count_part_1 = get_part_1(rocks.clone(), depth);
    //     println!(
    //         "Part 1: Total count: {} in {:?}",
    //         count_part_1,
    //         start.elapsed()
    //     );
    // }

    // if depth <= 40 {
    //     let start = Instant::now();
    //     let count_part_2 = get_part_2(rocks.clone(), depth);
    //     println!(
    //         "Part 2: Total count: {} in {:?}",
    //         count_part_2,
    //         start.elapsed()
    //     );
    // }

    // if depth <= 40 {
    //     let start = Instant::now();
    //     let count_part_2b = get_part_2b(rocks.clone(), depth);
    //     println!(
    //         "Part 2b: Total count: {} in {:?}",
    //         count_part_2b,
    //         start.elapsed()
    //     );
    // }

    // if depth <= 40 {
    //     let start = Instant::now();
    //     let count_part_2c = get_part_2c(rocks.clone(), depth);
    //     println!(
    //         "Part 2c: Total count: {} in {:?}",
    //         count_part_2c,
    //         start.elapsed()
    //     );
    // }

    let start = Instant::now();
    let count_part_2d = get_part_2d(rocks_32.clone(), depth);
    println!(
        "Part 2d: Total count: {} in {:?}",
        count_part_2d,
        start.elapsed()
    );

    Ok(())
}

fn get_part_1(mut rocks: Vec<u64>, depth: u8) -> usize {
    for _ in 0..depth {
        let mut i = 0;
        while i < rocks.len() {
            let rock = rocks[i];
            let rock_string = rock.to_string();
            let rock_size = rock_string.len();

            if rock == 0 {
                rocks[i] = 1;
            } else if rock_size % 2 == 0 {
                let split = rock_string.split_at(rock_size / 2);
                let split_rock: Vec<u64> = vec![split.0, split.1]
                    .into_iter()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                rocks.splice(i..=i, split_rock.iter().cloned());

                i += 1;
            } else {
                rocks[i] *= 2024;
            }

            i += 1;
        }
    }

    rocks.len()
}

fn get_part_2(rocks: Vec<u64>, depth: u8) -> usize {
    let mut count = 0;

    for rock in rocks {
        count_rocks(rock, depth, &mut count);
    }

    return count;
}

fn count_rocks(rock: u64, depth: u8, count: &mut usize) {
    if depth == 0 {
        //print!("[{rock}]");
        *count += 1;
        return; // Return after counting the leaf
    } else {
        if rock == 0 {
            count_rocks(1, depth - 1, count);
        } else if count_digits(rock) % 2 == 0 {
            let (first_integer, second_integer) = split_integer(rock, count_digits(rock) / 2);

            count_rocks(first_integer as u64, depth - 1, count);
            count_rocks(second_integer as u64, depth - 1, count);
        } else {
            count_rocks(rock * 2024, depth - 1, count);
        }
    }
}

fn count_digits(n: u64) -> usize {
    if n == 0 {
        return 1; // Special case: 0 has 1 digit
    }
    // Using log10 to calculate the number of digits
    (n as f64).log(10.0).floor() as usize + 1
}

fn split_integer(n: u64, split_position: usize) -> (u64, u64) {
    let divisor = 10u64.pow(split_position as u32);
    let first_part = n / divisor; // First part of the split
    let second_part = n % divisor; // Second part of the split

    (first_part, second_part)
}

fn get_part_2b(initial_rocks: Vec<u64>, depth: u8) -> usize {
    let mut stack = initial_rocks
        .into_iter()
        .map(|rock| (rock, depth))
        .collect::<Vec<_>>();
    let mut count = 0;

    while let Some((rock, current_depth)) = stack.pop() {
        if current_depth == 0 {
            count += 1;
        } else {
            let digits = count_digits(rock);
            if rock == 0 {
                stack.push((1, current_depth - 1));
            } else if digits % 2 == 0 {
                let (first_integer, second_integer) = split_integer(rock, digits / 2);
                stack.push((first_integer, current_depth - 1));
                stack.push((second_integer, current_depth - 1));
            } else {
                stack.push((rock * 2024, current_depth - 1));
            }
        }
    }

    count
}

fn get_part_2c(initial_rocks: Vec<u64>, depth: u8) -> usize {
    use rayon::prelude::*;

    initial_rocks
        .into_par_iter()
        .map(|rock| {
            let mut stack = vec![(rock, depth)];
            let mut count = 0;

            while let Some((rock, current_depth)) = stack.pop() {
                if current_depth == 0 {
                    count += 1;
                } else {
                    let digits = count_digits(rock);
                    if rock == 0 {
                        stack.push((1, current_depth - 1));
                    } else if digits % 2 == 0 {
                        let (first_integer, second_integer) = split_integer(rock, digits / 2);
                        stack.push((first_integer, current_depth - 1));
                        stack.push((second_integer, current_depth - 1));
                    } else {
                        stack.push((rock * 2024, current_depth - 1));
                    }
                }
            }

            count
        })
        .sum()
}

fn get_part_2d(rocks: Vec<u32>, depth: u8) -> usize {
    println!("{:?}, Depth: {}", rocks, depth);

    let magic_table: Vec<(Vec<u8>, Vec<u8>)> = vec![
        (vec![1, 1, 2, 4], vec![2, 0, 2, 4]), // Row for digit 0
        (vec![1, 2, 4], vec![2, 0, 2, 4]),    // Row for digit 1
        (vec![1, 2, 4], vec![4, 0, 4, 8]),    // Row for digit 2
        (vec![1, 2, 4], vec![6, 0, 7, 2]),    // Row for digit 3
        (vec![1, 2, 4], vec![8, 0, 9, 6]),    // Row for digit 4
        (vec![1, 1, 2, 4, 8], vec![2, 0, 4, 8, 2, 8, 8, 0]), // Row for digit 5
        (vec![1, 1, 2, 4, 8], vec![2, 4, 5, 7, 9, 4, 5, 6]), // Row for digit 6
        (vec![1, 1, 2, 4, 8], vec![2, 8, 6, 7, 6, 0, 3, 2]), // Row for digit 7
        (vec![1, 1, 2, 4, 8], vec![3, 2, 7, 7, 2, 6, 0, 8]), // Row for digit 8
        (vec![1, 1, 2, 4, 8], vec![3, 6, 8, 5, 9, 1, 8, 4]), // Row for digit 9
    ];

    let mut count = 0;

    for rock in rocks {
        count_rocks(rock, depth, &mut count);
    }

    fn count_rocks(rock: u32, current_depth: u8, count: &mut usize) {
        if current_depth == 0 {
            *count += 1;
            return; // Return after counting the leaf
        } else {
            if count_digits(rock) == 1 {
                //short circuit the algorithm, hopefully drastically increase performance
                *count += count_via_magic_table(rock, current_depth);
                return;
            } else {
                if rock == 0 {
                    count_rocks(1, current_depth - 1, count);
                } else if count_digits(rock) % 2 == 0 {
                    let (first_integer, second_integer) =
                        split_integer(rock, count_digits(rock) / 2);

                    count_rocks(first_integer, current_depth - 1, count);
                    count_rocks(second_integer, current_depth - 1, count);
                } else {
                    count_rocks(rock * 2024, current_depth - 1, count);
                }
            }
        }
    }

    fn count_via_magic_table(rock: u32, current_depth: u8) -> usize {
        // magic_table
        println!(
            "{rock}, Current depth: {}, Remaining depth: {}",
            current_depth,
            current_depth - 25
        );

        0
    }

    fn count_digits(n: u32) -> usize {
        if n == 0 {
            return 1; // Special case: 0 has 1 digit
        }
        // Using log10 to calculate the number of digits
        (n as f32).log(10.0).floor() as usize + 1
    }

    fn split_integer(n: u32, split_position: usize) -> (u32, u32) {
        let divisor = 10u32.pow(split_position as u32);
        let first_part = n / divisor; // First part of the split
        let second_part = n % divisor; // Second part of the split

        (first_part, second_part)
    }

    println!();

    count
}
