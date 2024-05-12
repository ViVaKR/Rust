use std::fmt::{self};

pub type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
pub struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|x| x.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

pub fn print_result(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
