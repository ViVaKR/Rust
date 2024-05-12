use bootcamp::{
    array_check,
    closure::adds,
    closure_run, print_data, read_text,
    result::{adder, multiply, prints},
    structs::Data,
    user_error,
};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut menu: i32 = -1;
    if let Ok(temp) = config.query.parse::<i32>() {
        menu = temp;
    }

    match menu {
        1 => {
            let num = 10;
            println!(
                "\u{26EC} Hello, World! {num} + 1 = {}",
                bootcamp::add_one(num)
            )
        }
        2 => {
            read_text(&config.file_path);
        }
        3 => {
            let data = Data {
                id: 1,
                subject: String::from("vivakr"),
            };

            print_data(data);
        }
        4 => {
            closure_run();
            let sum = adds();
            match sum {
                Ok(val) => println!("\u{26EC} sum: {}", val),
                Err(_) => println!("Error"),
            }
        }
        5 => {
            prints(multiply(&"35", &"23"));
            prints(adder(&"53", &"47"));
            array_check();
            user_error();
        }

        _ => (),
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
