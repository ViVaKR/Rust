use std::io;
use std::io::Write;
use std::mem::size_of_val;

fn main() {
    let mut input = String::new();
    loop {
        println!("Select Menu (1 ~ 100, Exit: 0)");
        print!("Enter Number: ");
        io::stdout().flush().unwrap();
        input.clear();
        let mut choice: u32 = 0;
        let b = io::stdin().read_line(&mut input).expect("Not integer");
        println!("Input {}, Size: {}", choice, b);
        if let Ok(val) = input.trim().parse::<u32>() {
            choice = val;
        } else {
            println!("Please enter a valid number.");
            continue;
        }
        match choice {
            0 => {
                println!("=== 프로그램을 종료합니다. ===");
                return;
            }
            1 => size_of('H', '글'),
            2 => println!("Hello, World!"),
            3 => println!("Three"),
            4 => variables(),
            5 => println!("{}", add(25, 35)),
            6 => expr(),
            7 => expr_exercies(),
            8 => {
                let (x, y) = (1, 2);
                let s = sum(x, y); // 3
                assert_eq!(s, 3);
                println!("Success!");
            }
            9 => while_statement(10),
            10 => matches_fn(),
            11 => var_01(),
            12 => var_02(),
            13 => data_types_01(),
            14 => loop_statement(10),
            15 => user_input(),
            16 => arithmethic(),
            17 => condition(),
            _ => break,
        }
    }
}

fn condition() {
    println!("{}", (2f32) < 3f32);

    // compound condition
}
fn arithmethic() {
    // Type Conversion
    let x: u8 = 255;
    let y: u8 = 10;
    let a = 255f32;
    let b = 15f32;
    let c = 34_i16;
    let d = 568_i16;
    let e = i128::MAX;
    let f = (i32::MAX as i64) + 1;
    let j = 125i32;
    let k = f as i32 / j; // not cached compil time

    let z = x / y;
    println!("{} {} {} {} {} {}", z, x % y, a / b, c * d, e, k);

    // String to Integer

    let mut input = String::new();
    println!("Enter Number: ");
    print!(">> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");

    if let Ok(val) = input.trim().parse::<i64>() {
        println!("Your Number => {} + {} = {}", val, 234, val + 234);
    } else {
        println!("Is Not Number Type!");
        return;
    }
}

fn user_input() {
    println!("=== Input String ===");
    print!("Enter => ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    let size = io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("Input String: {}Input String Size: {}", input, size);
    /*
        [ Prelude ]
        Prelude 은 Rust가 모든 Rust 프로그램에 자동으로 가져오는 것들의 목록입니다.
        가능한 한 작은 크기로 유지되며, 사물에 초점을 맞추고 있으며, 특히 모든 Rust 프로그램에 사용됩니다.
    */
}

fn data_types_01() {
    // Data Type
    // (1) Scalar Data Type
    //-> integer, floating-point, boolean, cahracter/char
    let x = 2; // i8, i16, i32, i64, i128 (signed integer)
    let y: u32 = 972; // u8, u16, u32, u64, u128 (unsigned integer)
    let flooating_point: f32 = 10.9f32;
    let true_or_false = 1; // 0 or 1 enable
    let letter: char = ';';
    println!(
        "{} {} {} {} {}",
        x, y, flooating_point, true_or_false, letter
    );
    // (2) Compound Data Type
    //-> array, tuple

    let tup = (1, true, 's');
    let tup2: (i8, bool, char) = (124, false, 'A');

    println!("{} {} {}", tup2.1, tup2.0, tup2.2);
    println!("{} {} {}", tup.1, tup.0, tup.2);

    // Array
    let arr = [1, 2, 3, 4, 5];

    for j in arr {
        println!("{}", j);
    }
    let arr2 = 1..=10;

    for i in arr2 {
        println!("{:?}", i);
    }
}
fn var_02() {
    // constant
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}

fn var_01() {
    let x = 4;
    println!("x is: {}", x);
    {
        let x = 2;
        println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is: {}", x);
}

// =================================================== //
fn matches_fn() {
    let b = false;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("We hanve no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
// Implement this function, don't modify the fn signatures

// ( macros ) //
// panic!() or
// unimplemented!() or

fn sum(x: i32, y: i32) -> i32 {
    // (8)
    x + y
    // 쎄미 콜론 이 있으면 반환 값이 없이 끝남.
    // 리턴 타입은 쌔미 콜론을 넣지 않음.
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
    let my_name = "Viv";

    // Bool : Boolean value of true or false of size 1 byte
    let _tf: bool = false;
    let your_half = my_half;

    // implicitly_ret_unit
    // Unit : Empty tuple of size 0 bytes, used to return "nothing" in expressions or functions
    if !_tf {
        println!("Success!")
    }

    // String Type
    // name
    //
    // (1) prt : Pointer to data stored on the heap
    // (2) len : Data size in bytes
    // (3) capacity : Total amount of memory received from the allocator
    //--> 64bit : 3 * 8 bytes (usize) => s1 is 24 bytes. (1, 2, 3) , fixed size to stack
    //--> 실제 데이터는 힙에 저장 되면서 연속적인 동적인 크기를 가진다.
    let s1 = String::from("Hi ->");
    println!("{} {} {} {} {} {}", s1, two, hello, j, your_half, my_name);
    //-> Expression : Evaluate to a resultant value
    //-> Statment
    //      - Instructions tha perform some action but do not procuce a value
    //      - Function definitions are statements, as well as code that ends with ";" (usually)

    // Copy vs. Move
    //-->
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
        2 * x // yes semicolon -> no return value -> ()
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
    println!("{}", a);
}
