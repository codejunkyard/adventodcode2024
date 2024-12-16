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

    let url = "https://adventofcode.com/2024/day/14/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3";

    //let grid = Grid::new((101, 103)); // Define the grid size
    let grid = Grid::new((11, 7)); // Define the grid size
    let iteration: u8 = 100;
    let mut robots: Vec<RobotVector> = transform(&input, &grid);

    // let count_part_1 = get_part_1(&mut robots, grid.size, iteration);
    // println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&mut robots, grid.size);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

// fn get_part_1(robots: &mut Vec<RobotVector>, grid_size: (u8, u8), iteration: u8) -> u32 {
//     for robot in robots.iter_mut() {
//         for _ in 0..iteration {
//             robot.next();
//             //println!("Robot position: {:?}", robot.position);
//         }
//     }

//     quadrant_count(&robots, &grid_size)
// }

fn get_part_2(robots: &mut Vec<RobotVector>, grid_size: (u8, u8)) -> u32 {
    // Enable raw mode to capture input
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).expect("Failed to clear screen");

    println!("Press the right arrow key to increment iteration. Press 'q' to quit.");

    let mut last_pressed = Instant::now(); // Track the last time a key was processed
    let mut pressed_count = 0;

    loop {
        // Wait for a keyboard event
        if event::poll(std::time::Duration::from_millis(50)).expect("Failed to poll event") {
            if let Event::Key(key_event) = event::read().expect("Failed to read event") {
                let now = Instant::now();

                match key_event.code {
                    KeyCode::Right => {
                        // Ignore repeated presses within 100 ms
                        if now.duration_since(last_pressed) > Duration::from_millis(100) {
                            last_pressed = now; // Update last pressed time

                            // Move all robots
                            for robot in robots.iter_mut() {
                                robot.next();
                            }
                            pressed_count += 1;

                            print_robots(robots, grid_size, pressed_count);
                        }
                    }
                    KeyCode::Left => {
                        // Ignore repeated presses within 100 ms
                        if now.duration_since(last_pressed) > Duration::from_millis(100) {
                            last_pressed = now; // Update last pressed time

                            // Move all robots
                            for robot in robots.iter_mut() {
                                robot.prev();
                            }

                            pressed_count -= 1;

                            print_robots(robots, grid_size, pressed_count);
                        }
                    }
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal to normal mode
    disable_raw_mode().expect("Failed to disable raw mode");

    0
}

struct Grid {
    size: (u8, u8), // Grid dimensions
}

impl Grid {
    pub fn new(size: (u8, u8)) -> Self {
        Grid { size }
    }

    pub fn size(&self) -> (u8, u8) {
        self.size
    }
}

struct RobotVector<'a> {
    pub position: (u8, u8),
    velocity: (i16, i16),
    grid: &'a Grid, // Reference to the parent grid with a lifetime 'a
}

impl<'a> RobotVector<'a> {
    // pub fn new(position: (u8, u8), velocity: (i16, i16), grid: &'a Grid) -> Self {
    //     RobotVector {
    //         position,
    //         velocity,
    //         grid,
    //     }
    // }

    // pub fn position(&self) -> (u8, u8) {
    //     self.position
    // }

    // pub fn velocity(&self) -> (i16, i16) {
    //     self.velocity
    // }

    // pub fn grid_size(&self) -> (u8, u8) {
    //     self.grid.size()
    // }

    pub fn next(&mut self) {
        let (px, py) = self.position;
        let (vx, vy) = self.velocity;
        let (gx, gy) = self.grid.size();

        self.position = (
            ((px as i16 + vx + gx as i16) % gx as i16) as u8,
            ((py as i16 + vy + gy as i16) % gy as i16) as u8,
        );
    }

    pub fn prev(&mut self) {
        let (px, py) = self.position;
        let (vx, vy) = self.velocity;
        let (gx, gy) = self.grid.size();

        self.position = (
            ((px as i16 - vx + gx as i16) % gx as i16) as u8,
            ((py as i16 - vy + gy as i16) % gy as i16) as u8,
        );
    }
}

fn transform<'a>(input: &'a str, grid: &'a Grid) -> Vec<RobotVector<'a>> {
    // p=2,4 v=2,-3"
    let re = Regex::new(r"-?\d+").unwrap();

    input
        .lines()
        .map(|line| {
            //println!("{line}");
            // Collect all matches into a vector
            let captures: Vec<i16> = re
                .find_iter(line) // Iterator over matches
                .filter_map(|mat| mat.as_str().parse::<i16>().ok()) // Parse and filter valid integers
                .collect();

            let rv = RobotVector {
                position: (captures[0] as u8, captures[1] as u8),
                velocity: (captures[2], captures[3]),
                grid,
            };

            rv
        })
        .collect()
}

fn quadrant_count(robots: &Vec<RobotVector>, grid_size: &(u8, u8)) -> u32 {
    //println!("0: {}", grid_size.0); // 101 x
    //println!("1: {}", grid_size.1); // 103 y

    let mut q1: u32 = 0;
    let mut q2: u32 = 0;
    let mut q3: u32 = 0;
    let mut q4: u32 = 0;

    for robot in robots {
        // println!(
        //     "{:?}:{}x{}",
        //     robot.position,
        //     (grid_size.0 - 1 / 2),
        //     (grid_size.1 - 1 / 2)
        // );

        //println!("robot.position.0: {}", robot.position.0);
        //println!("robot.position.1: {}", robot.position.1);

        if robot.position.0 < ((grid_size.0 - 1) / 2) {
            if robot.position.1 < ((grid_size.1 - 1) / 2) {
                //println!("q1");
                q1 += 1;
            } else if robot.position.1 > ((grid_size.1 - 1) / 2) {
                //println!("q3");
                q3 += 1;
            }
        } else if robot.position.0 > ((grid_size.0 - 1) / 2) {
            if robot.position.1 < ((grid_size.1 - 1) / 2) {
                //println!("q2");
                q2 += 1;
            } else if robot.position.1 > ((grid_size.1 - 1) / 2) {
                //println!("q4");
                q4 += 1;
            }
        }

        //println!("{:?}", robot.position);
    }

    println!("{},{}", q1, q2);
    println!("{},{}", q3, q4);

    q1 * q2 * q3 * q4
}

fn print_robots(robots: &Vec<RobotVector>, grid_size: (u8, u8), iteration: i32) {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            let mut robot_found = false;
            for robot in robots {
                if robot.position == (x, y) {
                    robot_found = true;
                    break;
                }
            }

            if robot_found {
                print!("1");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!("[{}]", iteration);

    stdout.flush().unwrap();
}
