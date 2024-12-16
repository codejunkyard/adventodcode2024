use advent_lib::fetch_input;
use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::error::Error;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/15/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";

    let mut lanternfish: Lanternfish = transform(&input);

    let count_part_1 = get_part_1(&mut lanternfish);
    println!("Part 1: Total count: {}", count_part_1);

    // let count_part_2 = get_part_2(&mut robots, grid.size, iteration);
    // println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(lanternfish: &mut Lanternfish) -> u32 {
    0
}

// fn get_part_2(grid: &mut Vec<Vec<char>>) -> u32 {
//     0
// }

struct Lanternfish {
    grid: Vec<Vec<char>>,
    input: Vec<char>,
}

impl Lanternfish {
    // Constructor for Lanternfish
    fn new(grid: Vec<Vec<char>>, input: Vec<char>) -> Self {
        Lanternfish { grid, input }
    }
}

fn transform(input: &str) -> Lanternfish {
    // Destructure the split result into two parts
    let (grid_part, input_part) = input
        .trim()
        .split_once("\n\n")
        .expect("Input must contain a grid and an input separated by a blank line");

    // Convert the grid into a Vec<Vec<char>>
    let grid = grid_part
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Convert the remaining input into a Vec<char>
    let input = input_part.chars().collect();

    Lanternfish::new(grid, input)
}
