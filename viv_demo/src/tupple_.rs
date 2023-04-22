#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::btree_map::Values;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn tupple_() {

    println!("Hello Tupple");
    let my_tupple:(u8, String, f64) = (47, "Viv".to_string(), 50_000.34);
    println!("Age: {}, Name : {}, Etc : {}", my_tupple.0, my_tupple.1, my_tupple.2);

    let(age, name, etc) = my_tupple;
    println!("Age: {}, Name : {}, Etc : {}", age, name, etc);
}

