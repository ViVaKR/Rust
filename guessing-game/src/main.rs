#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The scecret number is {}", secret_number);
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small {}", guess),
        Ordering::Equal => println!("You win! {} == {}", &secret_number, guess),
        Ordering::Greater => println!("Too big {}", guess),
    }
}
