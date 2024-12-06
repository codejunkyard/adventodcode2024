// Import the modules for each day
mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure there is at least one argument (the day number)
    if args.len() < 2 {
        eprintln!("Please specify the day number (e.g., cargo run 2 for Day 2).");
        return;
    }

    let day = &args[1]; // Get the first argument (the day number)

    match day.as_str() {
        "1" => {
            println!("Running solution for Day 1:");
            let _ = day1::solve();
        }
        "2" => {
            println!("Running solution for Day 2:");
            let _ = day2::solve();
        }
        "3" => {
            println!("Running solution for Day 3:");
            let _ = day3::solve();
        }
        "4" => {
            println!("Running solution for Day 4:");
            let _ = day4::solve();
        }
        _ => eprintln!("No solution available for Day {}", day),
    }
}
