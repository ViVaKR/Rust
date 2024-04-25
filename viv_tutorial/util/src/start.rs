use ansi_term::Colour;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn viv_menu() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// HashMap
pub fn viv_hashmap() {
    let mut menus: HashMap<u32, String> = HashMap::new();
    menus.insert(0, String::from("Quit"));
    menus.insert(1, String::from("Random"));
    menus.insert(2, String::from("Array"));

    for (key, value) in menus.iter().sorted() {
        println!("{}. {}", key, value);
    }
    println!("\n\x1b[31m\u{2766} Select Menu\x1b[0m");
    print!("{}  ", Colour::Yellow.paint("\u{26DF}"));
    io::stdout().flush().unwrap();
}

/// Format
pub fn viv_format_print() {
    //
    println!("\u{2766} {temp}", temp = "Hi Everyone");
}

/*
   $ restup update
   $ rustc --version
   $ rustup doc
   $ rust main.rs

   $ cargo --version
   $ cargo new hello_world --bin
   $ cargo build
   $ cargo check
   $ cargo run -q
*/
