use std::{fs, io, num, fmt};
use std::fmt::Formatter;
use std::io::Error;
use std::num::ParseIntError;

fn main() {
    match get_int_from_file() {
        Ok(num) => println!("{}", num),
        Err(e) => eprintln!("{}", e),
    }
}

enum MyError {
    Io(io::Error),
    Num(num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO Error: {}", e),
            MyError::Num(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl From<io::Error> for MyError {
    fn from(e: Error) -> Self {
        MyError::Io(e)
    }
}

impl From<num::ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
       MyError::Num(e)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    let num_str = fs::read_to_string(path)?;
    let result = num_str.trim().parse::<i32>()?;

    Ok(result * 2)
}