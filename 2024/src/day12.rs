use advent_lib::fetch_input;
use dotenv::dotenv;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/10/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    // RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE

    let garden_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count_part_1 = get_part_1(&garden_map);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&garden_map);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(garden_map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut inventory: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut group_id: u32 = 0;

    fn get_region(garden_map: &Vec<Vec<char>>, row: usize, col: usize) -> (u32, u32) {
        let mut perimeter = 0;

        for (dx, dy) in &DIRECTIONS {
            let x = col as isize + *dx;
            let y = row as isize + *dy;

            if x == -1
                || y == -1
                || x == garden_map[0].len() as isize
                || y == garden_map.len() as isize
            {
                perimeter += 1;
            } else {
                let x = x as usize;
                let y = y as usize;

                if garden_map[y][x] != plant_type {
                    perimeter += 1;
                } else {
                    //(area, perimeter) = get_region(garden_map, row, col);
                }
            }
        }
    }

    for row in 0..garden_map.len() {
        for col in 0..garden_map[row].len() {
            // Traverse in all directions
            let plant_type = garden_map[col][row];
            let mut perimeter: usize = 0;

            for (dx, dy) in &DIRECTIONS {
                let x = col as isize + *dx;
                let y = row as isize + *dy;

                if x == -1
                    || y == -1
                    || x == garden_map[0].len() as isize
                    || y == garden_map.len() as isize
                {
                    perimeter += 1;
                } else {
                    let x = x as usize;
                    let y = y as usize;

                    if garden_map[y][x] != plant_type {
                        perimeter += 1;
                    }
                }
            }

            if let Some(entry) = inventory.get_mut(&plant_type) {
                entry.0 += 1; // Update the area
                entry.1 += perimeter; // Update the perimeter
            } else {
                inventory.insert(plant_type, (1, perimeter));
            }
        }
    }

    println!("{:?}", inventory);

    for entry in inventory {
        sum += (entry.1).1 * (entry.1).0;
    }

    sum
}

fn get_part_2(garden_map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    sum
}
