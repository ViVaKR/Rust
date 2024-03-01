#![allow(dead_code)]
#![allow(unused)]

use std::io;
use std::io::Write;
use std::mem::size_of_val;

fn main() {
    let mut string = String::new();
    loop {
        println!("Select Menu (x to escape)");
        print!("Enter Number: ");
        io::stdout().flush().unwrap();
        string.clear();
        let b = io::stdin().read_line(&mut string).expect("Not integer");
        let choice = string.trim().parse().expect("Input not an integer");

        println!("You typed: {:?}", choice);
        let v: (i32, i32) = (2, 3);
        let _v: () = ();

        match choice {
            0 => break,
            1 => size_of('H', '글'),
            2 => println!("hello"),
            3 => println!("Three"),
            4 => assert_eq!(_v, implicityly_ret_unit()),
            5 => assert_eq!(size_of_val(&_v), 0),
            6 => expr(),
            7 => expr_exercies(),
            8 => {
                let s = sum(1, 2); // 3
                assert_eq!(s, 3);
                println!("Success!");
            }
            _ => break,
        }
    }
}

fn sum(x: i32, y: i32) -> i32 {
    // (8)
    x + y
    // 쎄미 콜론 이 있으면 반환 값이 없이 끝남.
    // 리턴 타입은 쌔미 콜론을 넣지 않음.
}

fn implicityly_ret_unit() {
    println!("I will return a ()");
}
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
fn size_of(c1: char, c2: char) {
    println!("{:?}", size_of_val(&c1));
    println!("{:?}", size_of_val(&c2));
}
fn loop_statement(mut a: i32) {
    loop {
        if a == 100 {
            break;
        }
        a = a + 1;
    }
}
fn while_statement(mut a: i32) {
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}
fn if_statement() {
    let a = 99;
}
fn add(a: i32, b: i32) -> i32 {
    // body
    return a + b;
}

fn variables() {
    // Basic Types Recap //

    let two = 2;
    let hello = "hello";

    // Char : Single character of size 4 bytes
    let j = 'j';
    let my_half = 0.5;
    let mut my_name = "Viv";

    // Bool : Boolean value of true or false of size 1 byte
    let _tf: bool = false;
    let your_half = my_half;

    // implicitly_ret_unit
    // Unit : Empty tuple of size 0 bytes, used to return "nothing" in expressions or functions
    let _v: () = ();

    if !_tf {
        println!("Success!")
    }

    //-> Expression : Evaluate to a resultant value
    //-> Statment
    //      - Instructions tha perform some action but do not procuce a value
    //      - Function definitions are statements, as well as code that ends with ";" (usually)
}

// (6)
fn expr() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x // no semicolon -> return value
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x; // yes semicolon -> no return value -> ()
    };
    println!(
        "Expressiont Example\n- x is {:?}\n- y is {:?}\n- z is {:?}",
        x, y, z
    );
}

fn expr_exercies() {
    let v = {
        let mut x: i32 = 1;
        x = x + 2;
        x
    };

    assert_eq!(v, 3);
    println!("expr exercies => Success!");

    let a = {
        let x = 3;
        x
    };
}
