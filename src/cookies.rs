// cookie_parser.rs
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Cookie {
    name: String,
    value: String,
    // Other fields are omitted since we're only interested in name and value for the string
}

pub fn parse_cookies_from_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cookies: Vec<Cookie> = from_reader(reader)?;

    let cookie_strings: Vec<String> = cookies
        .into_iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect();

    Ok(cookie_strings.join("; "))
}