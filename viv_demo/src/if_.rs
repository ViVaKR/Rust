use std::io;
use rand::Rng;
use std::io:: {Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn if_() {

    println!();
    let age: i32 = 8;
    if (age >=1) && (age <= 18){
        println!("Important Birthday");
    }else if (age == 21) || (age == 50){
        println!("Important Birthday");
    }else if age >= 65 {
        println!("Important Birthday");
    }else{
        println!("Not an Important Birthday");
    }

    
    let mut my_age = 47;
    let can_vote: bool = if my_age >=18 {
        true    
    }else{
        false
    };

    println!("Can Vote {}", can_vote);
}
