#![allow(unused)]
use ::std::fmt::Debug;
use ::std::fmt::Display;
use bootcamp::{
    array_check, cfg_attr,
    closure::adds,
    closure_run, print_data, read_text,
    result::{adder, multiply, prints},
    structs::Data,
    thread_run, user_error, vector_iterator,
};
use std::collections::HashMap;
use std::str;
use std::{env, hash::Hash, io::Write, net::IpAddr, process};

fn print_str(s: String) {
    print!("{}\n", s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn borrow_object(s: &String) {
    println!("{}", s);
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn greeting(s: &str) {
    println!("{}", s);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let mut menu: i32 = 105;
    if let Ok(temp) = config.query.parse::<i32>() {
        menu = temp;
    }
    match menu {
        110 => {
            // All elements in an array must be of the same type.
            // Indexing starts at 0.
            // The length of an array is fixed.
            let arr: [i32; 5] = [1, 2, 3, 4, 5];
            assert!(arr.len() == 5);

            let arr1: [char; 3] = ['a', 'b', 'c'];

            // Fill the blank
            let arr2: [i32; 100] = [1; 100];
            assert!(arr2[0] == 1);
            assert!(arr2.len() == 100);

            let names: [String; 2] = [String::from("vivakr"), String::from("kimbumjun")];

            // 'Get' returns an Option<T>, it's safe to use
            let name0 = names.get(0);

            // But indexing is not safe
            let _name1 = &names[1];
            println!("{}, {}", name0.unwrap(), _name1);

            let arr3: [i32; 5] = [1, 2, 3, 4, 5];
            let arr4 = &arr3[0..2];
            assert_eq!([1, 2], arr4);

            let arr6 = "Hello, World!";

            let kor: [char; 4] = ['대', '한', '민', '국'];
            for c in kor.iter() {
                print!("{}", c);
            }
            println!("\n{}", &kor[kor.len() - 1]);

            println!("Success!");
        } // Array
        109 => {
            // 한글 유니코드 출력
            let data = b"Kim BumJun";
            // lower case
            println!("{:x?}", data);
            // upper case
            println!("{:X?}", data);

            let data = [0x00..0xFF];
            // print the leading zero
            println!("{:02X?}", data);
            // It can be combined with the pretty modifier as well
            println!("{:#04X?}", data);

            // AC00-D7AF
            let mut v = (0xAC00..=0xD7A3).collect::<Vec<u32>>();
            for t in v.into_iter().enumerate() {
                let c = std::char::from_u32(t.1).unwrap();
                print!("{:X?} {}, ", t.1, c);
            }
            println!();

            for c in "가나다라마바사아자차카타파하".chars() {
                print!("{:X?} ", c);
            }

            println!();

            let kimbumjun = "\u{AE40}\u{BC94}\u{C900}";
            println!("{}", kimbumjun);
        }
        108 => {
            let kimbumjun = "\u{AE40}\u{BC94}\u{C900}";
            let byte_escapt = "I'm writeng Ru\x73\x74";
            println!("{} What are you doing\x3F {}", kimbumjun, byte_escapt);

            let long_string = "String literals may span multiple lines. \
            The line break and indentation in the source will be included \
            in the string verbatim.";

            println!("{}", long_string);

            let raw_string = r"Escapes don't work here: \x3F \u{211D}";
            println!("raw string: (r\"\") -> {}", raw_string);

            // slice
            // &str[start..end]
            let s: String = String::from("Hi, I'm vivakr");
            let s1: &str = &s[4..];
            let s2: &str = &s[..4];
            let s3: &str = &s[4..8];
            println!("{}\n{}\n{}", s1, s2, s3);

            println!("{}", kimbumjun);
        }

        107 => {
            let mut s: String = String::from("I like to eat");
            let s: String = s.replace("eat", "sleep");
            println!("{}", s);

            let s1: String = String::from("hello");
            let s2: String = String::from(" world");
            let s3: String = s1 + s2.as_str();
            println!("{}", s3);

            let s4 = "hi";

            let s5 = String::from(s4); // s4.to_owned(); // &str -> String
            println!("{}", s5);

            let s6: String = "Hello, World~~".to_string(); // String::from("Hello, World~~");
            let s7: &str = &s6;
            let s8 = s6.as_str();
            println!("{} - {}", s7, s8);

            let s6 = "hello".to_string();
        }
        106 => {
            let mut s: String = String::from("Hi");
            s.push(','); // push a character
            s.push_str(" Everyoe"); // push a string
            s += "!"; // push a character

            println!("{}", s);
        }
        105 => {
            let s = String::from("Hello World");
            let h = &s[0..5];
            let w = &s[6..11];

            let k: Box<str> = "hello, world".into();
            greeting(&k);
        }
        105 => {
            let mut s: String = String::from("hello, ");

            let r2: &mut String = &mut s;
            r2.push_str("World!");

            println!("{}", r2);
        }
        104 => {
            // Dangling Reference
            let reference_to_nothing = dangle();
            println!("{}", reference_to_nothing);

            let x: i32 = 5;
            let y: &i32 = &x;
            let p: &i32 = &x; // p is a reference to x

            println!("\u{26EC} {:p} {}", p, *y); // 0x7ffeeb1b3a7c
            let mut s = String::from("hello");
            borrow_object(&s);

            let mut sss: String = String::from("hello");
            let p: &mut String = &mut sss;
            p.push_str(" world");

            let c: char = '글';
            let h1: &char = &c;
            let ref h2 = c;
            println!("{:?} {:?}", get_addr(h1), get_addr(h2));
            // println!("{:?} {:?}", h1, h2);
        }
        103 => {
            // Borrowing
            let mut s = String::from("hello");
            {
                let r1 = &mut s;
                r1.push_str(", world!");
                println!("{}", r1);
            }
            change(&mut s);

            let mut ss = String::from("hello");
            let r2 = &ss;
            let r3 = &ss;
            println!("{}, {}", r2, r3);

            let r4 = &mut ss;
        }
        102 => {
            #[derive(Debug)]
            struct Person {
                name: String,
                age: Box<u8>,
            }

            let person = Person {
                name: String::from("vivakr"),
                age: Box::new(30),
            };

            //
            let Person { name, ref age } = person;

            let t: (String, String) = (String::from("hello"), String::from("world"));
            let t2: (String, String) = (String::from("hello"), String::from("world"));
            println!("{:?} {:?}", name, age);
            let (s3, s4) = t2;
            println!("{:?} {:?}", s3, s4);
        }
        101 => {
            let s = String::from("hello");
            assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
            print_str(s.clone());
            println!("\u{26EC} {:?}", s.as_bytes());

            let x: (i32, i32, (), &str) = (1, 2, (), "hello");
            let y: (i32, i32, (), &str) = x;
            println!("\u{26EC} {:?} {:?}", x, y);

            let mut s2 = s;
            s2.push_str(", world!");
            println!("\u{26EC} {}", s2);

            let x: Box<i32> = Box::new(5); // Box is a smart pointer
            let mut y: Box<i32> = Box::new(1);

            *y = 4;
            println!("\u{26EC} {}", *y);
            assert_eq!(*x, 5);
        }
        1 => {
            println!("Hello, World!");
            // let num = 10;
            // println!(
            //     "\u{26EC} Hello, World! {num} + 1 = {}",
            //     bootcamp::add_one(num)
            // );

            // let mut input = String::new();
            // std::io::stdout().flush().unwrap();
            // std::io::stdin()
            //     .read_line(&mut input)
            //     .expect("fail to read");
            // let number = input.trim().parse::<i32>().unwrap();
            // println!("-> {:#32b}", number);
            // println!();
            // let bit_length = 24;
            // let bits: u32 = (!0) << (32 - bit_length);
            // let net_mask = IpAddr::V4(bits.into());
            // println!("{:?}\n{:#b}\n{:#32b}\n{:#32b}", net_mask, bits, !0, !3);

            // println!("Get bit\n{:032b}", -47);
            // for i in (0..32).rev() {
            //     print!("{}", (number >> i) & 1);
            // }
            // println!("\n");

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

        10 => {
            // Arbitrary vertices V
            // Arbitrary edges E
            // Support for loops & multiple edges
            // Clean API
            // Safe code only
            // Memory efficient
        }
        _ => (),
    }
}

pub struct Graph<VId, E = (), V = ()> {
    vertices: HashMap<VId, V>,
    adjacecy: HashMap<VId, Vec<(VId, E)>>,
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
