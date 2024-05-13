#![allow(unused)]
use bootcamp::{
    array_check, cfg_attr,
    closure::adds,
    closure_run, print_data, read_text,
    result::{adder, multiply, prints},
    structs::Data,
    thread_run, user_error, vector_iterator,
};
use std::{env, io::Write, net::IpAddr, process};

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
            );

            let mut input = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("fail to read");
            let number = input.trim().parse::<i32>().unwrap();
            println!("-> {:#32b}", number);
            println!();
            let bit_length = 24;
            let bits: u32 = (!0) << (32 - bit_length);
            let net_mask = IpAddr::V4(bits.into());
            println!("{:?}\n{:#b}\n{:#32b}\n{:#32b}", net_mask, bits, !0, !3);

            println!("Get bit\n{:032b}", -47);
            for i in (0..32).rev() {
                print!("{}", (number >> i) & 1);
            }
            println!("\n");

            /*
            --> Get a bit : (i >> n) & 1
            --> Set to 1  : i | (i << n)
            --> Set to 0  : i & !(1 << n)
            --> Toggle a bit : i ^ (1 << n)

            -47
            0b_00000000_00000000_00000000_00101110
            0b_11111111_11111111_11111111_11010001
            */
        }
        2 => {
            read_text(&config.file_path);
        } // Read Text by file path,
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

        6 => {
            // iter
            vector_iterator();
            // let t = vec![Settings {
            //     menu_id: String::from("100"),
            //     file_path: String::from("data.txt"),
            //     ignore_case: true,
            // }];
            // let mut vec: Vec<Settings> = "helloworld".chars().collect();
            // let settings = Settings::build(vec);
            cfg_attr();
        }

        7 => {
            let mut length: Vec<usize> = Vec::new();
            for _ in (0..2).into_iter() {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("fail to read");
                let temp = input.trim().parse::<usize>().unwrap();
                length.push(temp);
            }
            let area = length[0] * length[1];
            println!("{}", area);
        } // baekjoon - 27323, 직사각형

        8 => {
            //
            thread_run();
        }

        9 => {
            let ff = 0xFF;
            let n = 339319;

            println!("{:032b}\n{:032b}", ff, n);
        }
        _ => (),
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let args_len = args.len();
        if args_len > 5 {
            return Err(String::from("not allowed arguments"));
        }
        let mut query = String::new();
        let mut file_path = String::new();
        match args_len {
            1 => {
                query = "1".to_owned();
                file_path = String::from("-");
            }
            2 => {
                query = args[1].clone();
                file_path = String::from("-");
            }
            3 => {
                query = args[1].clone();
                file_path = args[2].clone();
            }
            _ => (),
        }
        Ok(Config { query, file_path })
    }
}

pub struct Settings {
    menu_id: String,
    file_path: String,
    ignore_case: bool,
}

impl Settings {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Settings, &'static str> {
        // args.next();

        let menu_id = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Settings {
            menu_id,
            file_path,
            ignore_case,
        })
    }
}

#[allow(dead_code)]
fn unused_function() {}
/*
- search for  ... bench: 19,620,300 (+/- 915,700)
- search iter ... bench: 19,234,900 (+/- 657,200), zero-cost abstraction

pre·dic·tion | pridíkʃən |
명사
[불][가]〔…에 대한; …이라는〕예언, 예보, 예견, 예측〔about, of …;that절〕
▸ make a prediction about the next election
다음 선거에 대한 예상을 하다
▸ His prediction of a large earthquake didn't come true.
대지진이 일어날 것이라는 그의 예언은 빗나갔다.
*/
