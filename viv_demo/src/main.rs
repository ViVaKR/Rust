#![allow(unused)]
mod print;
mod var;
mod if_;
mod match_;
mod array_;
mod tupple_;
mod loop_;

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!();
    println!("***** 메뉴선택 (숫자) *****");
    println!("1. Print");
    println!("2. Variable");
    println!("3. If Statement");
    println!("4. Match Statement");
    println!("5. Array");
    println!("6. tupple");
    println!("7. loop");
    println!("100. Exit");
    println!("************************");
    print!(">> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    // 화면정리
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    match input.trim_end() {
        "1" => print::print(),
        "2" => var::var(),
        "3" => if_::if_(),
        "4" => match_::match_(),
        "5" => array_::array_(),
        "6" => tupple_::tupple_(),
        "7" => loop_::loop_(),
        "100" => println!("***** 프로그램 종료! *****"),
        _ => println!("잘못된 입력입니다! 숫자만 선택하세요"),
    };
}
