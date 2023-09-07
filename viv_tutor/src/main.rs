#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::{io, string};

fn main() {

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gined the right to vote"),
    }

    // 10. Match (Switch) Statement
    // let age2: i32 = 70;
    // match age2 {
    //     1..=18 => println!("1 ~ 18 살 입니다."),
    //     21 | 50 => println!("21 ~50 살 입니다."),
    //     65..=i32::MAX => println!("65 살 이상입니다."),
    //     _=> println!("No Matched")
    // }
    // 9. 삼항연산
    // let mut my_age = 47;
    // let can_vote: bool = if my_age >= 18 { true } else { false };
    // println!("Can Vote : {}", can_vote);

    // 8. if statement
    // let age: i32 = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Importtant Birthday {age}");
    // } else if (age == 21) || (age == 50) {
    //     println!("Hi {age}");
    // } else if age >= 65 {
    //     println!("Too Old {age}")
    // } else {
    //     println!("Not an Important Birthday");
    // }

    // 7. Random Number
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random : {}", random_num);

    // 6. f32 vs f64 자릿수 : 7, 15
    // let num_1: f32 = 1.111111111111111;
    // println!("f32 : {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64 : {}", num_2 + 0.111111111111111);
    // 5. 사칙연산
    // let mut num_3 = 5;
    // let num_4 = 4;
    // println!("5 + 4 = {}", num_3 + num_4);
    // println!("5 - 4 = {}", num_3 - num_4);
    // println!("5 * 4 = {}", num_3 * num_4);
    // println!("5 / 4 = {}", num_3 / num_4);
    // println!("5 % 4 = {}", num_3 % num_4);
    // num_3 += 1;
    // println!("{num_3}");

    // 4. boolean
    // let is_true = true; // or false
    // let my_grade = 'A';

    //3. Max of Variable
    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}",f32::MAX);
    // println!("Max f64 : {}", f64::MAX);

    //2.
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse()
    // .expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    //1.
    // println!("What is your name?");
    // let mut name: String = String::new();
    // let greeting: &str = "Nice to meet you";
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Din't Receive INput");
    // println!("Hi {}! {}", name.trim_end(), greeting);
}
