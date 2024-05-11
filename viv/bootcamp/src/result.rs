use std::num::ParseIntError;
// Define a generic alias for a `Result` with the error type `ParseIntError`
type AliasedResult<T> = Result<T, ParseIntError>;

pub fn multiply(first_num_str: &str, second_num_str: &str) -> AliasedResult<i32> {
    first_num_str
        .parse::<i32>()
        .and_then(|f| second_num_str.parse::<i32>().map(|s| f * s))
}

pub fn adder(first: &str, second: &str) -> AliasedResult<i32> {
    let first_num = match first.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    let second_num = match second.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };

    Ok(first_num + second_num)
}

pub fn pulling_results(v: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = v.first().map(|x| x.parse::<i32>().map(|n| n * 2));

    opt.map_or(Ok(None), |r| r.map(Some))
}

pub fn prints(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("result is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
