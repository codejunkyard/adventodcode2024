use advent_lib::fetch_input;
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/8/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;

    // Convert input to Vec<Vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count_part_1 = get_part_1(&grid);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&grid);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(grid: &Vec<Vec<char>>) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    // Iterate over row indices
    for row_index in 0..grid.len() {
        // Iterate over column indices for each row
        for col_index in 0..grid[row_index].len() {
            if grid[row_index][col_index] != '.' {
                antinodes.extend(get_antinodes(&grid, row_index, col_index));
            }
        }
    }

    antinodes.len()
}

fn get_part_2(grid: &Vec<Vec<char>>) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    // Iterate over row indices
    for row_index in 0..grid.len() {
        // Iterate over column indices for each row
        for col_index in 0..grid[row_index].len() {
            if grid[row_index][col_index] != '.' {
                antinodes.extend(get_all_antinodes(&grid, row_index, col_index));
            }
        }
    }

    antinodes.len()
}

fn get_antinodes(
    grid: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[row_index].len() {
            let c_source: char = grid[row_index][col_index];
            let c_target: char = grid[row][col];

            if c_target == c_source && row_index != row && col_index != col {
                let y = 2 * row as i32 - row_index as i32;
                let x = 2 * col as i32 - col_index as i32;

                if x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[0].len() as i32 {
                    antinodes.insert((y as usize, x as usize));
                }
            }
        }
    }

    antinodes
}

fn get_all_antinodes(
    grid: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let c_source: char = grid[row_index][col_index];

    //println!("Source {} at ({},{})", c_source, row_index, col_index);

    for row in 0..grid.len() {
        for col in 0..grid[row_index].len() {
            let c_target: char = grid[row][col];

            if c_target == c_source && row_index != row && col_index != col {
                antinodes.insert((row, col));

                let mut y = 2 * row as i32 - row_index as i32;
                let mut x = 2 * col as i32 - col_index as i32;

                let addition_tuple = (row as i32 - row_index as i32, col as i32 - col_index as i32);
                // println!(
                //     "  Addition tuple: ({},{}) for target ({}, {})",
                //     addition_tuple.0, addition_tuple.1, row, col
                // );

                while x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[0].len() as i32 {
                    //println!("    with resonance at ({y},{x})");
                    antinodes.insert((y as usize, x as usize));
                    y += addition_tuple.0;
                    x += addition_tuple.1;
                }
            }
        }
    }

    antinodes
}
