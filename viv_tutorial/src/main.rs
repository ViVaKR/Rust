use common::{
    algorithm::{fibnacci, is_prime},
    closures::{capture_types, sort_list, Inventory, ShirtColor},
    datatype::{data_type, operation, vector},
    function::{function, largest_char, largest_i32},
    generic::{generic_run, longest},
    interface::runner,
    iterator::{iter_filter, iter_map, iter_repeat},
    loop_a::{for_a, for_b, loop_a, loop_b, while_a},
    ownership::{first_wrod, first_wrod_ref, ownership_a},
    panic_result::{panic_run, result_run, Guess},
    some::{check_optional, divide},
    structs::struct_run,
};
use rust_decimal_macros::dec;
use snippet::example::{array_ex, devide_by, std_fmt, Operator};
use std::{
    env,
    fs::File,
    io::{self, Read},
    process,
    time::Instant,
};
use util::start;

extern crate communicator;
extern crate snippet;
extern crate util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut file = File::open(filename).expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    println!("\u{26EC} With text:\n{}", contents);

    //clear_screen();
    let now = Instant::now();
    loop {
        start::display_menu();
        let choice: i32 = start::choice_menu();
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
                //

                let x = Some(5);
                match x {
                    Some(n) => println!("\u{26EC} {}", n),
                    None => todo!(),
                }
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
            }

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

            _ => {
                continue;
            }
        }
        let elapse_time = now.elapsed();
        let elapse = elapse_time.as_secs();
        pause_screen(choice, elapse);
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
    // clearscreen::clear().unwrap();
}
