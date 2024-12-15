// Import the modules for each day
// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
mod day13;

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
        // "1" => {
        //     println!("Running solution for Day 1:");
        //     let _ = day1::solve();
        // }
        // "2" => {
        //     println!("Running solution for Day 2:");
        //     let _ = day2::solve();
        // }
        // "3" => {
        //     println!("Running solution for Day 3:");
        //     let _ = day3::solve();
        // }
        // "4" => {
        //     println!("Running solution for Day 4:");
        //     let _ = day4::solve();
        // }
        // "5" => {
        //     println!("Running solution for Day 5:");
        //     let _ = day5::solve();
        // }
        // "6" => {
        //     println!("Running solution for Day 6:");
        //     let _ = day6::solve();
        // }
        // "7" => {
        //     println!("Running solution for Day 7:");
        //     let _ = day7::solve();
        // }
        // "8" => {
        //     println!("Running solution for Day 8:");
        //     let _ = day8::solve();
        // }
        // "9" => {
        //     println!("Running solution for Day 9:");
        //     let _ = day9::solve();
        // }
        // "10" => {
        //     println!("Running solution for Day 10:");
        //     let _ = day10::solve();
        // }
        // "11" => {
        //     println!("Running solution for Day 11:");
        //     let _ = day11::solve();
        // }
        // "12" => {
        //     println!("Running solution for Day 12:");
        //     let _ = day12::solve();
        // }
        "13" => {
            println!("Running solution for Day 13:");
            let _ = day13::solve();
        }
        _ => eprintln!("No solution available for Day {}", day),
    }
}
