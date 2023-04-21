#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::btree_map::Values;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn array_() {
    println!();
    println!("***** Array *****");

    let arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("1st: {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
    let mut loop_idx: usize = 0;

    println!("\n***** loop *****");
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }

        if arr_1[loop_idx] == 9 {
            break;
        }

        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    println!("\n***** While Loop *****");
    loop_idx = 0;
    while loop_idx < arr_1.len() {
         println!("Arr : {}", arr_1[loop_idx]);

         loop_idx += 1;
    }

    println!("\nfor loop");
    for val in arr_1 {
        println!("Var : {}", val);
    }



}
