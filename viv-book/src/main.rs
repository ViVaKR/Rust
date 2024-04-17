#![allow(unused)]
use std::{
    env,
    fmt::format,
    io::{self, Write},
}; // env::set_var

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn draw_line(title: &str, count: usize) {
    // ':' <fill> <align> <width> //
    let t = '+';
    let s = '*';
    println!("{:=^1$}", title, count);
}

fn get_choice() -> i32 {
    let mut input = String::new();

    print!(">> ");
    io::stdout().flush().unwrap();
    let mut choice: i32 = 0;

    let read = io::stdin()
        .read_line(&mut input)
        .expect("잘못된 입력입니다.");

    if let Ok(val) = input.trim().parse::<i32>() {
        choice = val;
    } else {
        choice = 200;
    }
    choice
}

fn menu() {
    println!("\n\u{2766} 메뉴선택 (1 ~ 100, Quit: 0)");
    let space = 3;
    let line_count = 60;
    draw_line(" ( menu ) ", line_count);

    print!("{:space$}. {}\n", 0, "Quit");
    print!("{:space$}. {}\t", 1, "assert_eq!");
    print!("{:space$}. {}\t", 2, "format");
    print!("{:space$}. {}\t", 3, "struct");
    print!("{:space$}. {}\n", 4, "tuple");
    print!("{:space$}. {}\t", 5, "array");
    print!("{:space$}. {}\t", 6, "expressions");

    println!("");
    draw_line(" \u{2766} ", line_count);
}

fn main() {
    clear_screen();
    loop {
        menu();
        let choice = get_choice();
        clear_screen();

        let s = format!("{}{:-^60}", "", " \u{2766} start ");
        println!("{s}\n");
        match choice {
            0 => {
                clear_screen();
                return;
            }
            1 => {
                // note: run with `RUST_BACKTRACE=1`  (or full)
                // environment variable to display a backtrace
                env::set_var("RUST_BACKTRACE", "full");
                // RUST_BACKTRACE=1 cargo run
                // assert_eq!
                ex_assert_eq(32, 55);
            }
            2 => {
                // std::fmt
                ex_fmt();
            }
            3 => {
                ex_struct();
                let drink = Drink {
                    flavor: Flavor::Fuity,
                    fluid_oz: 6.0,
                };
                ex_drink(drink);
            }
            4 => {
                //--> tuple
                let mut s = String::new();
                s = "Hello World".into();
                let result = ex_tuple(3, 5, s);
                let (x, y) = (4, 5);
                println!(
                    "\u{2766} {} {} {} - x: {}, y: {}",
                    result.0, result.1, result.2, x, y
                );

                let user_info = ("Kim Bum Jun", 20);
                let favorites = ("margenta", 9, "TX", "Pizza", "Coding", "Home");

                let place = favorites.5;

                let (x, y) = ex_tuple_cordinate();
                println!("\u{2766} {} {} {}", x, y, place);
                if y > 5 {
                    print!("Yes greater than 5 -> {}", y);
                }
            }
            5 => {
                //--> array
                ex_array();
            }
            6 => {
                //--> expressions
                let my_num = 3;
                let is_it_5 = if my_num < 5 { true } else { false };

                let message = match my_num {
                    1 => "Hello",
                    2 => "World",
                    3 => "Thanks",
                    _ => "goodbye",
                };

                println!(
                    "\u{2766} {} is greater than 5? {}, {}",
                    my_num, is_it_5, message
                );
            }
            _ => break,
        } // match
        let s = format!("{}{:-^60}", "", " \u{2766} end ");
        println!("\n{s}\n");
    } // loop
}
//--> (5)
fn ex_array() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    for x in array {
        print!("{x} ");
    }

    let bytes: [u8; 3] = [7, 2, 9];

    let mut temp = String::new();
    temp = "Hello, World".into();
    let bytes: [u8; 3] = [1, 0, 2];
    assert_eq!(
        1,
        u16::from_le_bytes(<[u8; 2]>::try_from(&bytes[0..2]).unwrap())
    );
    assert_eq!(512, u16::from_le_bytes(bytes[1..3].try_into().unwrap()));
    let temp = u16::from_le_bytes(bytes[1..3].try_into().unwrap());
    print!("{}", temp);
    println!("");
    let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let t = &arr[1..8];
    for item in arr[0..9].into_iter().enumerate() {
        print!("\nindex: {}, value: {:}", item.0, item.1);
    }
}

//--> (4)
fn ex_tuple(a: i32, b: i32, c: String) -> (i32, i32, String) {
    (a * 10, b * 20, format!("== {:?} ==", c) as String)
}

fn ex_tuple_cordinate() -> (i32, i32) {
    (32, 89)
}

//--> (3)
enum Flavor {
    Sparkling,
    Sweeet,
    Fuity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn ex_struct() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!(
        "\u{21aa} Stock: {:?}, Cereal: {:?}",
        cereal.stock, cereal.price
    );
}

fn ex_drink(drink: Drink) {
    print!("\u{21aa} ");
    match drink.flavor {
        Flavor::Sparkling => println!(
            "{}. Sparkling {:?}",
            Flavor::Sparkling as i32,
            drink.fluid_oz
        ),
        Flavor::Sweeet => println!("{}. Sweeet {:?}", Flavor::Sweeet as i32, drink.fluid_oz),
        Flavor::Fuity => println!("{}. Fuity {:?}", Flavor::Fuity as i32, drink.fluid_oz),
    }
}

//--> (2)
fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
}

fn ex_fmt() {
    /*
    --> format_spec := [[fill]align][sign]['#']['0'][width]['.' precision][type]
    --> fill := character
    --> align := '<' | '^' | '>'
    --> width := count
     */
    println!("Hello, {}", "World");
    println!("The number is {}", 1);
    println!("{}", format!("{value}, {name}", value = 4, name = "Hi"));
    println!("{}", format!("{:#?}", (100, 200)));
    println!("\n{:-^1$}", "title", 30);
    println!("{:04}", 40);
    let people = "Rustanceans";
    println!("Hello {people}");

    println!(
        "{value}, {a}, {b}, {c}",
        value = 59,
        a = "a",
        b = 'b',
        c = 3.141593
    );
    println!("{:?}, {}", (3, 4), make_string(12345, "World "));

    // width
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 10);
    println!("Hello {1:0$}?", 25, 'x');

    // Fill/Algnment
    println!("World {:><30}!", "K");
    println!("World {:#>35}!", "Kim");

    for i in 0..=15 {
        println!("{0:#b} {0:#x} {0:#X} {0:#o}", i);
    }
}

// (1)
fn ex_assert_eq(a: i32, b: i32) {
    assert_eq!(a, b, "viv -> testing with {} and {}", a, b);
}
