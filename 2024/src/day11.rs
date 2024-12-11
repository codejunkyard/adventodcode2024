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
    //let input = "6 11 33023 4134 564 0 8922422 688775"; // "125 17";

    let mut rocks: Vec<u64> = input
        //let rocks: Vec<u64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let start = Instant::now();
    let count_part_1 = get_part_1(&mut rocks);
    println!(
        "Part 1: Total count: {} in {:?}",
        count_part_1,
        start.elapsed()
    );

    // let start = Instant::now();
    //let count_part_2 = get_part_2(&rocks);
    //let count_part_2 = get_part_2(rocks);
    // println!(
    //     "Part 2: Total count: {} in {:?}",
    //     count_part_2,
    //     start.elapsed()
    // );

    let start = Instant::now();
    let count_part_2b = get_part_2b(rocks.clone(), 25);
    println!(
        "Part 2b: Total count: {} in {:?}",
        count_part_2b,
        start.elapsed()
    );

    Ok(())
}

fn get_part_1(rocks: &mut Vec<u64>) -> usize {
    let n = 25;

    for _ in 0..n {
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

fn get_part_2(rocks: &Vec<u64>) -> usize {
    let depth = 25;
    let mut count = 0;
    // Initial:         (2) 125 17
    // After 1 blink:   (3) 253000 1 7
    // After 2 blinks:  (4) 253 0 2024 14168
    // After 3 blinks:  (5) 512072 1 20 24 28676032
    // After 4 blinks:  (9) 512 72 2024 2 0 2 4 2867 6032
    // After 5 blinks: (13) 1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
    // After 6 blinks: (22) 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2

    // [6]       [11]        [33023]     [4134]      [564]       [0]   [8922422]       [688775]
    // [12144]   [1]   [1]   [66838552]  [41]  [34]  [1141536]   [1]   [18058982128]   [688]    [775]
    // [24579456][2024][2024][6683][8552][4][1][3][4][2310468864][2024][36551379827072][1392512][1568600]

    for rock in rocks {
        //print!("[{rock}]");
        count_rocks(*rock, depth, &mut count);
    }

    println!();

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
