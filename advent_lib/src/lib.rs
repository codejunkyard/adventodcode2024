use reqwest::blocking::Client;
use std::error::Error;

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
