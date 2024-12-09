use reqwest::blocking::Client;
use std::error::Error;
//use std::process::Command;

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
