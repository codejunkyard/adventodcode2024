use advent_lib::fetch_input;
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;
use std::error::Error;

//const DEBUG: bool = true;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    let url = "https://adventofcode.com/2024/day/6/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    //let sum_part_1 = get_part_1(&grid);
    //println!("Part 1: Total sum: {}", sum_part_1);

    let sum_part_2 = get_part_2(&grid);
    println!("Part 2: Total sum: {}", sum_part_2);

    Ok(())
}

fn get_part_1(grid: &Vec<Vec<char>>) -> usize {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn turn(current_index: usize) -> usize {
        (current_index + 1) % DIRECTIONS.len()
    }

    fn look(current_position: &(usize, usize), current_direction_index: usize) -> (i32, i32) {
        let current_direction = DIRECTIONS[current_direction_index];
        let new_destination = (
            current_position.0 as i32 + current_direction.0,
            current_position.1 as i32 + current_direction.1,
        );

        new_destination
    }

    let mut current_position: (usize, usize) = (0, 0);
    let mut current_direction_index: usize = 0; // Start with 'Up' (index 0)
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Find starting position
    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            if grid[row_index][col_index] == '^' {
                current_position = (row_index, col_index);
                println!("Starting at: {:?}", current_position);
                break;
            }
        }
    }

    loop {
        visited.insert(current_position);

        // look in front
        let (y, x) = look(&current_position, current_direction_index);

        // Out of bound (Up, Right, Down, Left)
        if y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 || x < 0 {
            break;
        }

        let y_usize: usize = y as usize;
        let x_usize: usize = x as usize;

        if grid[y_usize][x_usize] == '#' {
            current_direction_index = turn(current_direction_index);
        } else {
            // move to where you are looking
            current_position = (y_usize, x_usize);
        }
    }

    visited.len()
}

fn get_part_2(grid: &Vec<Vec<char>>) -> usize {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn turn(current_index: usize) -> usize {
        (current_index + 1) % DIRECTIONS.len()
    }

    fn look(current_position: &(usize, usize), current_direction_index: usize) -> (i32, i32) {
        let current_direction = DIRECTIONS[current_direction_index];
        let new_destination = (
            current_position.0 as i32 + current_direction.0,
            current_position.1 as i32 + current_direction.1,
        );

        new_destination
    }

    let mut current_position: (usize, usize) = (0, 0);
    let mut current_direction_index: usize = 0; // Start with 'Up' (index 0)
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Find starting position
    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            if grid[row_index][col_index] == '^' {
                current_position = (row_index, col_index);
                println!("Starting at: {:?}", current_position);
                break;
            }
        }
    }

    // Find starting position
    for row_index in 0..grid.len() {
        for col_index in 0..grid[row_index].len() {
            if grid[row_index][col_index] == '.' {
                let object_position = (row_index, col_index);

                loop {
                    visited.insert(current_position);

                    // look in front
                    let (y, x) = look(&current_position, current_direction_index);

                    // Out of bound (Up, Right, Down, Left)
                    if y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 || x < 0 {
                        break;
                    }

                    let y_usize: usize = y as usize;
                    let x_usize: usize = x as usize;

                    if object_position == (y_usize, x_usize) || grid[y_usize][x_usize] == '#' {
                        current_direction_index = turn(current_direction_index);
                    } else {
                        // move to where you are looking
                        current_position = (y_usize, x_usize);
                    }
                }
            }
        }
    }

    visited.len()
}

fn has_repeating_pattern(arr: &[i32]) -> bool {
    let n = arr.len();

    for pattern_length in 1..=n / 2 {
        // Check for a repeating pattern of this length
        for start in 0..=(n - 2 * pattern_length) {
            let first = &arr[start..start + pattern_length];
            let second = &arr[start + pattern_length..start + 2 * pattern_length];

            if first == second {
                return true; // Found a repeating pattern
            }
        }
    }

    false
}
