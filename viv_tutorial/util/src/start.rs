use ansi_term::Colour;

use chrono::{DateTime, Local};
// ansi color
use itertools::Itertools; // sort
use std::collections::HashMap;
use std::io::{self, Write};

/// HashMap
pub fn display_menu() {
    let mut menus: HashMap<u32, String> = HashMap::new();

    menus.insert(0, String::from("Quit\t"));
    menus.insert(1, String::from("Random"));
    menus.insert(2, String::from("Array"));
    menus.insert(3, String::from("Format"));
    menus.insert(4, String::from("Option"));
    menus.insert(5, String::from("DataType"));
    menus.insert(6, String::from("Operation"));
    menus.insert(7, String::from("Function"));
    menus.insert(8, String::from("Loop"));
    menus.insert(9, String::from("OwnerShip"));
    menus.insert(10, String::from("PrimeNumber"));
    menus.insert(11, String::from("Fibonacci"));
    menus.insert(12, String::from("Closuers"));
    menus.insert(13, String::from("부동소숫점"));
    menus.insert(14, String::from("반복자"));
    menus.insert(15, String::from("Vector"));
    menus.insert(16, String::from("Option"));
    menus.insert(17, String::from("Trait"));
    menus.insert(18, String::from("Structs"));
    menus.insert(19, String::from("PanicResult"));
    menus.insert(20, String::from("Generic"));
    menus.insert(21, String::from("Bits"));
    menus.insert(22, String::from("Sort"));
    menus.insert(23, String::from("Colour"));

    let now: DateTime<Local> = Local::now();

    println!(
        "\n\u{2772} {current} \u{2773}\n\u{2728} 실행할 메뉴를 선택하세요 \u{2728}",
        current = now.format("%Y-%m-%d %H:%M:%S")
    );

    for (key, value) in menus.iter().sorted() {
        print!(
            "\x1b[32m{:>2}.\x1b[0m {}\t\t",
            key,
            Colour::Green.paint(value)
        );
        if (key + 1) % 5 == 0 {
            println!();
        }
    }

    println!("\n\x1b[32m\u{2766} Select Menu\x1b[0m");
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
