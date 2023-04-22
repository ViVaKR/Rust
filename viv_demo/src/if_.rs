use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn if_() {
    println!();

    let age: i32 = 8;

    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    let mut my_age = 47;
    let can_vote: bool = if my_age >= 18 { true } else { false };

    println!("Can Vote {}", can_vote);

    let n = 50;

    if n < 30 {
        println!("값은 30보다 작습니다!");
    } else if n == 30 {
        println!("값은 30입니다.");
    } else {
        println!("값은 30보다 큽니다!");
    }
}
