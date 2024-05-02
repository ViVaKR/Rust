use ansi_term::{Colour, Style};
use common::{
    add, add_as, addm, adds,
    algorithm::{fibnacci, is_prime},
    closures::{capture_types, sort_list, Inventory, ShirtColor},
    data_encrpt::{cipher_run, encrpt_run, signature_hmac_run},
    datatype::{data_type, operation, vector},
    function::{function, largest_char, largest_i32},
    generic::{generic_run, longest, GData},
    interface::runner,
    iterator::{iter_filter, iter_map, iter_repeat},
    loop_a::{for_a, for_b, loop_a, loop_b, while_a},
    make_public, ok_or_return,
    ownership::{first_wrod, first_wrod_ref, ownership_a},
    panic_result::{
        panic_run, read_from_file, read_from_file2, read_from_file3, result_run, Guess,
    },
    some::{check_optional, divide},
    std_deviation::{mean, std_deviation},
    strings::make_string,
    structs::struct_run,
    vectors::create_vectors,
    viv_listener::viv_listener,
};

use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};
use rand_distr::{Alphanumeric, Normal, NormalError};
use rust_decimal_macros::dec;
use snippet::example::{array_ex, devide_by, std_fmt, Operator};
use std::{
    cmp::Reverse,
    env,
    fs::File,
    io::{self, Read},
    process,
    time::Instant,
};
use util::{baekjoon::baekjoon_run, start};

extern crate communicator;
extern crate snippet;
extern crate util;

/// --> main
/// --> (run) $ ./run.sh <menu choice number>
fn main() -> Result<(), NormalError> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let selection = &args[2];

    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    println!("\u{26EC} Welcom: {}", contents);

    let mut arg_num = 0;
    if let Ok(val) = selection.trim().parse::<i32>() {
        arg_num = val;
    }

    #[cfg(target_endian = "big")]
    println!("This is a BigEndian system.");
    #[cfg(target_endian = "little")]
    println!("This is a LittleEndian system.");

    //clear_screen();
    let now = Instant::now();
    loop {
        start::display_menu();
        let choice: i32 = if arg_num > 0 {
            println!("\n\u{269E} \u{262C} \u{269F}");
            arg_num
        } else {
            start::choice_menu()
        };
        //clear_screen();
        match choice {
            -1 => {
                println!("잘못된 메뉴 선택입니다.");
                continue;
            } /* [ -1. not a number ] */

            0 => {
                process::exit(0x0100);
            } /* [ 0. Exit ] */

            1 => {
                let mut rng = rand::thread_rng();
                for _ in 1..=2 {
                    let _u8 = rng.gen::<u8>();
                    let _u16 = rng.gen::<u16>();
                    let _u32 = rng.gen::<u32>();
                    let _i32 = rng.gen::<i32>();
                    let _f64 = rng.gen::<f64>(); // 1

                    println!(
                        "\nu8:\t{} \
                        \nu16:\t{} \
                        \nu32:\t{} \
                        \ni32:\t{} \
                        \nf64:\t{}",
                        _u8, _u16, _u32, _i32, _f64
                    );
                }

                // 범위 내에서 난수 생성
                let ra = rng.gen_range(1..=10);
                let rb = rng.gen_range(1.0..=10.0);
                println!("\u{26EC} integer random: {}\nfloat random: {} ", ra, rb);

                // 균일한 분포, Uniform
                let die = Uniform::from(1..7);

                loop {
                    let throw = die.sample(&mut rng);
                    println!("\u{26EC} Roll the die: {}", throw);
                    if throw == 6 {
                        break;
                    }
                }

                // 주어진 분포로 난수를 생성
                let noraml = Normal::new(2.0, 3.0)?;
                let v = noraml.sample(&mut rng);
                println!("\u{26EC} {} is from a N(2, 9) distrubution", v);

                // Random Tuple
                let rand_tuple = rng.gen::<(i32, bool, f64)>();
                println!("\u{26EC} Random tuple: {:?}", rand_tuple);

                // Random Char
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();

                println!("\u{26EC} Random String: {}", rand_string);

                // 사용자 정의 문자 세트에서 임으의 비밀번호
                const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                        abcdefghijklmnopqrstuvwxyz\
                                        0123456789)(*&^%$#@!~";
                const PASSWORD_LEN: usize = 30;
                let password: String = (0..PASSWORD_LEN)
                    .map(|_| {
                        let idx = rng.gen_range(0..CHARSET.len());
                        CHARSET[idx] as char
                    })
                    .collect();
                println!("\u{26EC} Password: {}", password);
            } /* [ 1. Random ] */

            2 => {
                array_ex();
            } /* [ 2. Array ] */

            3 => {
                std_fmt();
            } /* [ 3. Format ] */

            4 => {
                let a: f64 = 3.14;
                let b: f64 = 0.54;

                let value = devide_by(a, b, Operator::Add);

                match value {
                    Some(rs) => {
                        println!("\u{26EC} {} + {} = {:.3}", a, b, rs);
                    }
                    None => println!("\u{26EC} 0",),
                }
            }

            5 => {
                data_type();
            } // [ Data Type ]
            6 => {
                operation();
            } // [ Operation ]

            7 => {
                function();

                // 제너릭
                let numbers = vec![34, 50, 25, 100, 65];
                let result = largest_i32(&numbers);
                // let result = largest(&numbers);
                println!("\u{26EC} The largest number is {}", result);

                let chars = vec!['y', 'm', 'a', 'q'];
                let result = largest_char(&chars);
                // let resutl = largest(&char);
                println!("\u{26EC} The largest char is {}", result);
            }

            8 => {
                loop_a();
                loop_b();
                while_a();
                for_a();
                for_b();
            } /* [ 8. loop ] */

            9 => {
                println!("\n\u{269E} OwerShip \u{269F}");
                ownership_a();

                // 슬라이스
                println!("\n\u{269E} Slice \u{269F}");
                let mut s = String::from("Hello, World");

                let result = first_wrod(&s);

                println!("\u{26EC} first word = {}", result);
                s.clear();

                let s = String::from("Hi Everyone");
                let len = s.len();
                let hi = &s[..2];
                let everyone = &s[3..len];
                let all = &s[..];
                println!("\u{26EC} Hi = {hi}, Everyone = {everyone} == All = {all}");

                let s2 = String::from("Fine Thanks And You"); // &str : 불변 참조자.
                let result2 = first_wrod_ref(&s2);

                println!("\u{26EC} String from = {} ", result2);

                let s3 = "Good Morning";
                let result3 = first_wrod_ref(s3);
                println!("\u{26EC} leteral = {}", result3);

                // Common Slice
                let a = [1, 2, 3, 4, 5];
                let slice = &a[1..3];
                println!("\u{26EC} a = {:?}, b = {:?}", a, slice);
            } // [ 소유권, 슬라이스, 대여 ]

            10 => {
                for i in (2..100).into_iter().enumerate() {
                    let prime = if is_prime(i.1, 2) {
                        "소수".to_owned()
                    } else {
                        "-".to_owned()
                    };
                    println!("\u{26EC} {} {}", i.1, prime);
                }
            } // [ prime number ]

            11 => {
                let number = 46;
                let r = fibnacci(number);
                println!("\u{26EC} {} -> {}", number, r);
                assert_eq!(2971215073, r);
            } // [ fibonacci ]

            12 => {
                // --> Closure, 클로저
                // 자신의 환경을 캡처하는 익명함수.
                // 변수에 저장하거나 다른 함수에 인수로 전달할 수 있는 익명 함수.
                // 매개변수 혹은 반환 값의 타입을 명시하도록 요구하지 않음, 타입명시를 추가할 수는 있음.

                let store = Inventory {
                    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
                };

                // 빨간색 셔츠로 설정한 고객.
                let user_pref1 = Some(ShirtColor::Red);
                let giveaway1 = store.giveaway(user_pref1);

                println!(
                    "\u{26EC} The user with preference {:?} gets {:?}",
                    user_pref1, giveaway1
                );

                // 색상 설정이 없는 고객.
                let user_pref2 = None;
                let giveaway2 = store.giveaway(user_pref2);
                println!(
                    "\u{26EC} The user with preference {:?} gets {:?}",
                    user_pref1, giveaway2
                );

                // 캡처 타입
                capture_types();

                // Sort
                println!("\n\u{269E} Sorted List \u{269F}");
                sort_list();
            } // [ Closure ]

            13 => {
                // --> 부동 소수점 (1/3 => 0.3333... 과 같은 의미.)
                // x = 1.3
                // 1                 .    0.5  0.25    0.125   0.00625     0.03125     0.015625        0.0078125

                // Integer Side : 1  .    Decimal Side : .3
                // 0.3 * 2 = 0.6    0
                // 0.6 * 2 = 0.2    1
                // 0.2 * 2 = 0.4    0
                // 0.4 * 2 = 0.8    0
                // 0.8 * 2 = 0.6    1
                // 0.6 * 2 = 0.2    1
                // 0.2 * 2 = 0.4    0

                // 1.3 => 1 . 0 1 0 0 1 1 0
                // => 1001 이 영원히 반복됨.

                // 0 + 1/4  + 0/8  +  0/16   +        1/32   +   1/64    ...
                // 0 + 0.25 +  0   +   0     +     0.03125   +  0.015625 ...
                // --> 0.296875 + ...

                let x: f64 = 0.7;
                let y: f64 = 0.6;
                // --> 1.2999999999999998 (64비트, 16자릿수)
                println!("\u{26EC} {} + {} = {sum}", x, y, sum = x + y);

                // --> 해결책
                // $ cargo add rust_decimal
                // $ cargo add rust_decimal_macros

                let number = dec!(0.6) + dec!(0.7);
                println!("\u{26EC} {} + {} = {}", 0.6, 0.7, number); // 1.3
            } // [ Floating point ]

            14 => {
                //
                println!("\n\u{269E} Iterator \u{269F}");
                iter_repeat();
                iter_map();
                iter_filter();
            } // [ 반복 ]

            15 => {
                vector();
            } // [ Vector ]

            16 => {
                let result = divide(3.24, 0.872);

                match result {
                    Some(v) => println!("\u{26EC} result = {}", v),
                    None => println!("\n\u{269E} Cannot divid by 0 \u{269F}"),
                }

                let optional = None;
                check_optional(optional);
                let optional = Some(Box::new(9000));
                check_optional(optional);
            } // [ Option ]

            17 => {
                // let tweet = Tweet {
                //     name: String::from("horse_ebooks"),
                //     content: String::from("of course , as you probably already know people"),
                //     reply: false,
                //     retweet: false,
                // };

                // println!("\u{26EC} I new tweet {}", tweet.summarize())

                println!("\n\u{269E} Trait \u{269F}");
                runner();
            } // [ Trait (interface) ]

            18 => {
                struct_run();
                let t: GData<i32> = GData { value: 125 };
                println!("\u{26EC} {}", t.value);
            } // [ struct, impl ]

            19 => {
                let rs = result_run();

                match rs {
                    Ok(ok) => println!("\u{26EC} Ok: {} ", ok),
                    Err(err) => println!("\u{26EC} Error: {}", err),
                }

                panic_run();

                let guess = Guess::new(55);
                println!("\u{26EC} Guess: {}", guess.value);
            } // [ panic & result ]

            20 => {
                generic_run();

                {
                    let x = 5;
                    let r = &x;
                    println!("\u{26EC} r: {}", r);
                }

                // 라이프타임 명시 문법 : 어퍼스트로피 ' 로 시작.
                // 제너릭 라이프타미 파라미터 : <'a>
                // &i32 : a reference
                // 'a i32  : a reference with an explicit lifetime.
                // 'a mut i32 : a mutable reference with an explicit lifetime.
                /*
                 --> &i32       : 참조
                 --> &'a i32     : 명시적인 수명을 가진 참조입니다.
                 --> &'a mut i32 : 명시적인 수명을 가진 변경 가능한 참조입니다.
                */

                let string1 = String::from("long string is long");
                {
                    let string2 = String::from("xyz");
                    let result = longest(string1.as_str(), string2.as_str());
                    println!("\u{26EC} The longest string is {}", result)
                }

                let novel = String::from("Call me Ishmael. Some years age...");
                let first_sentence = novel.split('.').next().expect("Could not find a '.'");

                println!("\u{26EC} first_sentence: {}", first_sentence);
            } // [ 제너릭, Generic ]

            21 => {
                let n = 0b1010_1110isize;
                let one = 1isize;
                let num = 0x1Aisize;

                if cfg!(target_endian = "big") {
                    println!(
                        "\nBig Endian: {:#x}\nto_be: {:#x}\nSwap Bytes: {:#x}",
                        num,
                        num.to_be(),
                        num.swap_bytes()
                    );
                } else {
                    println!(
                        "\nLittle Endian: {:#x}\nto_be: {:#x}\nSwap Bytes: {:#x}",
                        num, // 0x0000_0000_0000_001a
                        num.to_be(),
                        num.swap_bytes() // 0x1a00_0000_0000_0000
                    );
                }

                println!(
                    "\u{26EC}\n{:#064b} ({})\n{:#016b}\n{:#016b}\n{:#016b}\n{:#016b} ({})\n{:#016b}",
                    n,
                    n,
                    n.swap_bytes(),
                    one,
                    one.rotate_left(3),
                    one.swap_bytes(),
                    one.swap_bytes(),
                    num
                );
            }

            22 => {
                let mut vec = vec![1, 5, 3, 10, 2, 15];
                println!("\u{26EC} [ Vector Sort ]\n\t- Origin: {:?}", vec);
                vec.sort();
                println!("\u{26EC} Sort ASC: {:?}", vec);
                vec.reverse();
                println!("\u{26EC} Sort DESC: {:?}", vec);

                vec.sort_by_key(|x| Reverse(*x));
                println!("\u{26EC} Sort by key: {:?}", vec);

                // float
                let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
                vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
                println!("\u{26EC} float sort: {:?}", vec);

                // struct vector
                let mut people = vec![
                    Person::new("Zoe".to_string(), 25),
                    Person::new("Al".to_string(), 58),
                    Person::new("John".to_string(), 12),
                ];

                people.sort();
                println!("\u{26EC} People: {:?}", people);

                people.sort_by(|a, b| b.age.cmp(&a.age));
                println!("\u{26EC} People: {:?}", people);
            } /* [ 22. Vector Sort ] */

            23 => {
                println!(
                    "This is {} in color, {} in color and {} in color",
                    Colour::Red.paint("red"),
                    Colour::Blue.paint("blue"),
                    Colour::Green.paint("green")
                );

                println!(
                    "\u{26EC} {}, {} and {}",
                    Colour::Yellow.paint("This is colored"),
                    Style::new().bold().paint("this is bold"),
                    Colour::Yellow.bold().paint("this is bold and colored"),
                );
            } /* [ 23. Colour, Style ] */

            24 => {
                let rs = encrpt_run();
                println!("\u{26EC} {:?}", rs);

                let rs = signature_hmac_run();
                println!("\u{26EC} result: {:?}", rs);

                _ = cipher_run();
            } /* [ 24. Hashing, Cipher ] */

            25 => {
                let content = read_from_file();
                match content {
                    Ok(txt) => println!("\u{26EC} text: {}", txt),
                    Err(err) => println!("\u{26EC} Error: {}", err),
                }

                // unwrap
                let rs = read_from_file2();
                match rs {
                    Ok((text, count)) => println!("\u{26EC} {}, ({}) ", text, count),
                    Err(_) => todo!(),
                }

                // expect
                let rs = read_from_file3();
                match rs {
                    Ok((text, count)) => println!("\u{26EC} {}, ({})", text, count),
                    Err(_) => todo!(),
                }
            } /* [ 25. Result ] */

            26 => {
                make_string();
            } /* [ 26. String ] */

            27 => {
                create_vectors();
            }

            28 => {
                let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
                let data_mean = mean(&data);
                let data_std_deviation = std_deviation(&data);
                println!("\u{26EC} Standard deviation is {:?}", data_std_deviation);

                let zscore = match (data_mean, data_std_deviation) {
                    (Some(mean), Some(std_deviation)) => {
                        let diff = data[4] as f32 - mean;

                        Some(diff / std_deviation)
                    }
                    _ => None,
                };

                println!(
                    "\u{26EC} Z-score of data at index 4 (with value {}) is {:?} ",
                    data[4], zscore
                );
            } /* [ 28. 표준편차 ] */

            29 => {
                _ = viv_listener();
            } /* [29. Socket Listener ] */

            30 => {
                let sum = add!(1, 2);
                println!("\u{26EC} sum(1, 1) -> {}", sum);

                // add_as
                println!("\u{26EC} add_as: {}", add_as!(0, 2, u8));

                // adds
                let sum = adds!(1, 2, 3, 4);
                println!("\u{26EC} sum: {}", sum);

                // addm
                println!(
                    "\u{26EC} addm: {}, addm: {},  addm: {}",
                    addm!(33),
                    addm!(10, 45),
                    addm!(4, 5, 6, 7, 8, 9)
                );

                //
                println!("\u{26EC} ok_or_return: {:?}", macro_run());

                // struct
                make_public! {
                    #[derive(Debug)]
                    struct Name {
                        n:i64,
                        t:i64,
                        g:i64,
                    }
                }

                // --> Procedural macros in Rust
                // 1. Attribute-like macros
                // 2. Derive macros
                // 3. Function-like macros

                // $ cargo new macro-demo --lib
            } /* [ 30. macro ] */

            31 => {} /* [ 31. Directory Traversal] */

            32 => {
                let mut number = 0;
                loop {
                    number += 1;
                    if number > 5 {
                        break;
                    }
                    println!("\u{2708} {}", number);
                }

                // let mut array = vec![i32::MIN; 10];
                // for (i, v) in array.iter().enumerate() {
                //     loop {
                //         io::stdin()
                //             .read_line(&mut input)
                //             .expect("Failed to read line");

                //         if let Ok(val) = input.trim().parse::<i32>() {
                //             array[i] = val;
                //             break;
                //         } else {
                //             continue;
                //         }
                //     }
                // }

                // let idx = 0;
                // let max = 0;

                // for item in array {
                //     item =
                // }

                // loop {
                //     if let Ok(val) = input.trim().parse::<i32>() {
                //         if max < array[idx] {}
                //     }
                // }
            }

            33 => {
                //

                baekjoon_run();
            } /* [ 33. baekjoon ] */
            _ => {
                continue;
            }
        }
        let elapse_time = now.elapsed();
        let elapse = elapse_time.as_secs();
        pause_screen(choice, elapse);
        arg_num = -1;
    } // [ loop ]
}

fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

fn macro_run() -> Result<(), String> {
    // ok_or_return
    ok_or_return!(some_work(1, 4));
    ok_or_return!(some_work(1, 0));
    Ok(())
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn pause_screen(choice: i32, elapse: u64) {
    let mut pause = String::new();
    println!(
        "\n\n\u{2728} ({} / {}초) 완료되었습니다. \u{2728}",
        choice, elapse
    );
    io::stdin().read_line(&mut pause).expect("Complete!");
    clear_screen();
}

fn clear_screen() {
    // println!("\x1b[2J\x1b[1;1H"); // clear screen & set cursor 1,1
    clearscreen::clear().unwrap();
}
