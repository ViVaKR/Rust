use std::env;

fn main() {
    // this method needs to be inside main() method
    // env::set_var("RUST_BACKTRACE", "full");
    env::set_var("RUST_BACKTRACE", "1");

    let num = 5;
    match num {
        // switch syntax
        1 => mutable(),
        2 => scope(),
        3 => assert(),
        4 => unused_variable(),
        5 => tuple(),
        _ => println!("Hello, World"),
    }
}

fn tuple() {
    let (mut x, y) = (1, 2);
    x += 3;
    assert_eq!(x, 4);
    assert_eq!(y, 2);
    println!("Success");
}
#[allow(unused_variables)]
fn unused_variable() {
    let _x: i32 = 12; // under bar : no warning
    let _y: i32 = 345;
    let z: &str = "Hi, everyone";
}

fn assert() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x);
}

fn scope() {
    println!("Success");

    let a: i32 = 10;
    {
        let b: i32 = 5;
        println!("The value of a is {} and value of b is {}", a, b);
    }

    println!("The value of a is {}", a);

    let s: &str = "Hello";
    println!("{}, World", s);
}

fn mutable() {
    // mutable (뮤러블)
    let x: i32 = 123;

    let mut y = 1;
    y += 2;

    assert_eq!(x, y);
}
