use closure::{Inventory, ShirtColor};
use core::panic;
use interface::Print;
use std::{fs, path::Path};
use structs::Data;
use usererror::print_result;

use crate::{result::pulling_results, usererror::double_first};
pub mod closure;
pub mod interface;
pub mod result;
pub mod structs;
pub mod usererror;

pub fn closure_run() {
    //
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveway1 = store.giveway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveway1
    );

    let user_pref2 = None;
    let giveway2 = store.giveway(user_pref2);
    println!(
        "Ther user with preference {:?} gets {:?}",
        user_pref2, giveway2
    );
}

/// trait
pub fn print_data(data: Data) {
    data.print();
}

/// text reader
pub fn read_text(file_path: &str) {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Display` able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("assets");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("text");
    new_path.push(file_path);

    // `set_file_name` update the file name of the `PathBuf`
    new_path.set_file_name(file_path);

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => {
            let content = fs::read_to_string(s).expect("not found");
            println!("{} With Test:\n{}", new_path.display(), content);
        }
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn array_check() {
    let numbers = vec!["42", "93", "18"];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", pulling_results(numbers));
    println!("The first doubled is {:?}", pulling_results(strings));
}

pub fn user_error() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_result(double_first(numbers));
    print_result(double_first(empty));
    print_result(double_first(strings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(3, add_one(2));
    }
}
