#![allow(unused)]

use std::io;
use rand::Rng;
use std::io:: {Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


pub fn match_() {
    println!();
    println!("***** Match *****");
    let age: i32 = 48;
    
    match age {
        1..=18 => println!("Young man"),
        19 => println!("Good"),
        20 => println!("Nice"),
        21 | 50 => println!("Strong man"),
        65..=i32::MAX => println!("Hi there"),
        _=> println!("Unknown")
    };

    let voting_age = 18;

    match age.cmp(&voting_age) {
        Ordering:: Less => println!("Can't Vote"),
        Ordering:: Greater => println!("Can Vote"),
        Ordering:: Equal => println!("You gained the rigth to vote"),
        _=> println!("Hello World!")
    }
}


