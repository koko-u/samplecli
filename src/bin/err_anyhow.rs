use anyhow::{Result, Context};
use std::fs;

fn main() {
    match get_int_from_file() {
        Ok(num) => println!("{}", num),
        Err(e) => println!("{:#?}", e),
    }
}

fn get_int_from_file() -> Result<i32> {

    let path = "number.txt";

    let num_str = fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

    num_str.trim()
        .parse::<i32>()
        .map(|i| i * 2)
        .context("failed to parse string")
}