use ansi_term::Colour;

use chrono::{DateTime, Local};
// ansi color
use itertools::Itertools; // sort
use std::collections::HashMap;
use std::io::{self, Write};

/// HashMap
pub fn display_menu() {
    let mut menus: HashMap<u32, String> = HashMap::new();

    menus.insert(0, String::from("Quit"));
    menus.insert(1, String::from("Random"));
    menus.insert(2, String::from("Array"));
    menus.insert(3, String::from("Format"));
    menus.insert(4, String::from("Option"));

    let now: DateTime<Local> = Local::now();

    println!(
        "\n\u{2772} {current} \u{2773}\n\u{2728} 실행할 메뉴를 선택하세요 \u{2728}",
        current = now.format("%Y-%m-%d %H:%M:%S")
    );
    for (key, value) in menus.iter().sorted() {
        print!("{}. {}\t\t", key, value);
        if (key + 1) % 5 == 0 {
            println!();
        }
    }

    println!("\n\x1b[31m\u{2766} Select Menu\x1b[0m");
    print!("{}  ", Colour::Yellow.paint("\u{26DF}"));
    io::stdout().flush().unwrap();
}

pub fn choice_menu() -> i32 {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if let Ok(result) = choice.trim().parse::<i32>() {
        result
    } else {
        -1
    }
}