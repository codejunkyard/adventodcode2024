use advent_lib::fetch_input;
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/10/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    let input = fetch_input(url, &session_token)?;
    //let input = "0123\n1234\n8765\n9876"; // 1
    let topographic_map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    let count_part_1 = get_part_1(&topographic_map);
    println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&topographic_map);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(topographic_map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    // Iterate over row indices
    for row_index in 0..topographic_map.len() {
        // Iterate over column indices for each row
        for col_index in 0..topographic_map[row_index].len() {
            if topographic_map[row_index][col_index] == 0 {
                sum +=
                    count_hiking_trails((row_index as isize, col_index as isize), &topographic_map);
                //print!("{}", topographic_map[row_index][col_index]);
            }
        }
        //println!();
    }

    sum
}

fn get_part_2(topographic_map: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;

    // Iterate over row indices
    for row_index in 0..topographic_map.len() {
        // Iterate over column indices for each row
        for col_index in 0..topographic_map[row_index].len() {
            if topographic_map[row_index][col_index] == 0 {
                sum += count_distinct_hiking_trails(
                    (row_index as isize, col_index as isize),
                    &topographic_map,
                );
                //print!("{}", topographic_map[row_index][col_index]);
            }
        }
        //println!();
    }

    sum
}

fn count_hiking_trails(trailhead: (isize, isize), map: &Vec<Vec<u8>>) -> usize {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut summits: HashSet<(isize, isize)> = HashSet::new();

    climb(0, trailhead, map, &mut summits);

    // for all 4 positions, N,E,S,W
    fn climb(
        previous: u8,
        position: (isize, isize),
        map: &Vec<Vec<u8>>,
        summits: &mut HashSet<(isize, isize)>,
    ) -> () {
        // Traverse in all directions
        for (dx, dy) in &DIRECTIONS {
            let x = position.1 + *dx;
            let y = position.0 + *dy;

            if !(x < 0 || y < 0 || x >= map[0].len() as isize || y >= map.len() as isize) {
                let current = map[y as usize][x as usize];
                if current == previous + 1 {
                    if current == 9 {
                        //println!("Found a 9! @ ({},{})", y, x);
                        summits.insert((y, x)); // Insert into summits
                    } else {
                        climb(current, (y, x), map, summits); // Recursive call
                    }
                }
            }
        }
    }

    summits.len()
}

fn count_distinct_hiking_trails(trailhead: (isize, isize), map: &Vec<Vec<u8>>) -> usize {
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut distinct_hiking_trails: usize = 0;

    climb(0, trailhead, map, &mut distinct_hiking_trails);

    // for all 4 positions, N,E,S,W
    fn climb(
        previous: u8,
        position: (isize, isize),
        map: &Vec<Vec<u8>>,
        distinct_hiking_trails: &mut usize,
    ) -> () {
        // Traverse in all directions
        for (dx, dy) in &DIRECTIONS {
            let x = position.1 + *dx;
            let y = position.0 + *dy;

            if !(x < 0 || y < 0 || x >= map[0].len() as isize || y >= map.len() as isize) {
                let current = map[y as usize][x as usize];
                if current == previous + 1 {
                    if current == 9 {
                        println!("Found a 9! @ ({},{})", y, x);
                        *distinct_hiking_trails += 1;
                    } else {
                        climb(current, (y, x), map, distinct_hiking_trails);
                    }
                }
            }
        }
    }

    distinct_hiking_trails
}
