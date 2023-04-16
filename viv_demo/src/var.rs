use std::alloc::System;

use rand::Rng;

pub fn var() {

    println!();
    println!("***** Variable & Type *****");
    println!();
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and {}", age, ONE_MIL);

    println!("Max u32 :\t{}", u32::MAX);
    println!("Max u64 :\t{}", u64::MAX);
    println!("Max usize :\t{}", usize::MAX);
    println!("Max u128 :\t{}", u128::MAX);
    println!("Max f32 :\t{}", f32::MAX);
    println!("Max f64 :\t{}", f64::MAX);

    let is_true: bool = true; // or false
    let my_grade = 'A';
    let num_1: f32 = 1.1111111111111111;
    let num_2: f64 = 1.1111111111111111;
    println!(
        "\n( 32bit vs 64bit )\nnum_f32 : {}\nnum_f64 : {}",
        num_1 + 0.1111111111111111,
        num_2 + 0.1111111111111111
    );
    println!();
    let num_3:u32 = 5;
    let num_4: u32 = 3;
    println!("== 사칙연산 ==");
    println!("5 + 3 = {}", num_3 + num_4);
    println!("5 - 3 = {}", num_3 - num_4);
    println!("5 * 3 = {}", num_3 * num_4);
    println!("5 / 3 = {}", num_3 / num_4);
    println!("5 % 3 = {}", num_3 % num_4);

    println!("랜덤번호");
    let rnd = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", rnd);

}
