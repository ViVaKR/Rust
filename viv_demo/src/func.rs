#![allow(unused)]

const MAXIMUM_NUMBER: u8 = 20;
use rand::Rng;
use std::cmp::Ordering;

pub fn func_() {
    println!("***** func_ *****");
    print_numbers_to(10);
}

pub fn print_numbers_to(num: u32) {

    // 
    for n in 1..num {
        println!("number : {}", n);
    }
}
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn print_() {
    println!("What is your name >> ");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    std::io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

pub fn while_() {
    println!("***** while_ *****");

    let mut n = 1;

    while n < 50 {
        println!("number = {}", n);
        n = n + 1;
    }
}

pub fn enum_() {
    let dir: Dir = Dir::Up;

    match dir {
        Dir::Up => println!("We are headking up!"),
        Dir::Down => println!("We are headking Down!"),
        Dir::Left => println!("We are headking Left!"),
        Dir::Right => println!("We are headking Right!"),
    }
}

pub fn tupple_() {
    println!("Hello Tupple");
    let my_tupple: (u8, String, f64) = (47, "Viv".to_string(), 50_000.34);
    println!(
        "Age: {}, Name : {}, Etc : {}",
        my_tupple.0, my_tupple.1, my_tupple.2
    );

    let (age, name, etc) = my_tupple;
    println!("Age: {}, Name : {}, Etc : {}", age, name, etc);

    let tup = (20, "Rust", 3.5, (1, 3, 5), true);
    println!(
        "tupple {} {} {} {} {}",
        tup.0,
        tup.1,
        (tup.3).2,
        (tup.3).1,
        tup.4
    );

    let tup_named = (45, 6.7, "Computer");
    let (a, b, c) = tup_named;

    println!("a: {}, b: {}, c: {}", a, b, c);
}

pub fn var_() {
    println!();
    println!("***** Variable & Type *****");
    println!();
    const ONE_MIL: u32 = 1_000_000;
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

    let num_1: f32 = 1.1111111111111111;
    let num_2: f64 = 1.1111111111111111;
    println!(
        "\n( 32bit vs 64bit )\nnum_f32 : {}\nnum_f64 : {}",
        num_1 + 0.1111111111111111,
        num_2 + 0.1111111111111111
    );
    println!();
    let num_3: u32 = 5;
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
        _ => println!("Unknown"),
    };

    let voting_age = 18;

    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the rigth to vote"),
    }
}

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

    let my_age = 47;
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

pub fn loop_() {
    println!("***** loop *****");
    let mut n = 0;
    loop {
        if n > 30 {
            break;
        };
        n += 1;
        if n == 7 {
            continue;
        }
        println!("n = {}", n);
    }

    let number = 30..51;
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for i in number {
        println!("i = {}", i);
    }

    for (index, name) in animals.iter().enumerate() {
        println!("{}. animal name is {}", index + 1, name);
    }

    for n in 1..MAXIMUM_NUMBER {
        println!("Maximum number {}", n);
    }
}
