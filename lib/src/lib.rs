use reqwest::blocking::Client;
use std::error::Error;
//use std::process::Command;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn fetch_input(url: &str, session_token: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_token))
        .header("User-Agent", "my-rust-app")
        .send()?;

    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(format!("Failed to fetch input: HTTP {}", response.status()).into())
    }
}

// pub fn clear_terminal() {
//     #[cfg(target_os = "windows")]
//     Command::new("cmd")
//         .arg("/C")
//         .arg("cls")
//         .output()
//         .expect("Failed to clear the terminal");

//     #[cfg(target_os = "unix")]
//     Command::new("clear")
//         .output()
//         .expect("Failed to clear the terminal");
// }

// A guard to ensure the cursor is restored on program exit
// struct CursorGuard;

// impl Drop for CursorGuard {
//     fn drop(&mut self) {
//         // Show the cursor
//         print!("\x1b[?25h");
//         let _ = io::stdout().flush();
//     }
// }

// fn cli_ui_test() {
//     let mut stdout = io::stdout();
//     //let animation = ["", ".", "..", "..."];

//     // Hide the cursor
//     print!("\x1b[?25l");
//     stdout.flush().unwrap();

//     // Ensure the cursor is shown again on program exit
//     let _cursor_guard = CursorGuard;

//     // Example animation frames for the grid
//     let frames = vec![
//         vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"],
//         vec!["9", "8", "7", "6", "5", "4", "3", "2", "1"],
//         vec![".", ".", ".", ".", ".", ".", ".", ".", "."],
//     ];

//     let grid_size = 3;

//     loop {
//         for frame in &frames {
//             // Move the cursor to the top-left corner
//             print!("\x1b[H"); // ANSI escape sequence to move the cursor to the top-left corner

//             // Clear the screen
//             print!("\x1b[2J"); // ANSI escape sequence to clear the screen

//             // Draw the grid
//             for row in 0..grid_size {
//                 for col in 0..grid_size {
//                     print!("{} ", frame[row * grid_size + col]);
//                 }
//                 println!(); // Move to the next line
//             }

//             stdout.flush().unwrap(); // Flush the output
//             sleep(Duration::from_millis(250)); // Wait for 1 second
//         }
//     }
// }
