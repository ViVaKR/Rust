#![allow(dead_code)]
#![allow(unused)]
pub fn playground() {
    // let s = dangle();

    let mut s = String::from("Hello World");
    let i = first_word(&s);
    println!("{}", i);
    s.clear(); // s == ""

    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];

    println!("{} {}", hello, world);

    let s = String::from("hello");
    let len = s.len();

    println!("{} {}, {}, {:?}", len, &s[..len], &s[..], &s.as_bytes());

    let s = String::from("hello world");

    println!("\u{26EC} - {}", get_first_word(&s));

    let s = "Hello, world!";
    println!("{}", get_word(&s));

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("\u{26EC} - {:?}", slice);
}

/// Adds one to the number given.
///
/// # Example
///
/// ```
/// let arg = 5;
/// let answer = my_crete::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// 3.
pub fn playground_match() {
    // add_one(46);
    let five = Some(5);
    let six = plus_one(five).unwrap();
    let none = plus_one(None);

    println!("\u{26EC} {:?}, {:?}", six, none);
    if_let(3);

    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one throught five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 5, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p;

    println!("{vara} - {varb}, {x} - {y}", vara = a, varb = b);

    let pa = Point { x: 0, y: 7 };
    match pa {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neiter axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(Color::Rgb(125, 125, 125));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, value {}",
                h, s, v
            )
        }
        _ => (),
    }

    let txt = Message::Write(String::from("Hi, Everyone!"));

    match txt {
        Message::Quit => todo!(),
        Message::Move { x, y } => todo!(),
        Message::Write(message) => println!("{}", message),
        Message::ChangeColor(_) => todo!(),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}, {inches}, {x}, {y}");

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let _x = 5;
    let y = 10;
    println!("{} {}", _x, y);

    let s = Some(String::from("Hello"));
    if let Some(temp) = &s {
        println!("found a string {}, World", temp);
    }
    println!("{}", s.unwrap());

    let origin = PointB { x: 0, y: 0, z: 0 };
    match origin {
        PointB { x, .. } => println!("x is {}", x),
    }

    let nums = (2, 4, 8, 16, 32);
    match nums {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // 매치 가드
    let n = Some(4);
    match n {
        Some(n) if n % 2 == 0 => println!("The number {} is even", n),
        Some(n) => println!("The number {} is odd", n),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        (4 | 5 | 6) if y => println!("yes"),
        _ => println!("no"),
    }

    // let msg = MessageA::Hello { id: 5 };

    // let rs: MessageA = msg;

    // match msg {
    //     MessageA::Hello { id: find @ 3..=7 } => println!("Found an id in range: {}", find),
    //     MessageA::Hello { id: 10..=12 } => {
    //         println!("Found and id in another range")
    //     }
    // }
}

enum MessageA {
    Hello { id: i32 },
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

struct PointB {
    x: i32,
    y: i32,
    z: i32,
}
struct Point {
    x: i32,
    y: i32,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn if_let(_max: u8) {
    let config_max = Some(3u8);
    if let Some(_max) = config_max {
        println!("The maximum is configured to be {}", _max);
    }
    if let Some(temp) = config_max {
        println!("\u{26EC} if let - {}", temp);
    }
}
fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
// fn dangle() -> String {
//     let s = String::from("hello");
//     // &s // 실체가 없으므로 오류.
//     s
// }

/*
    --> slice, 슬라이스 : 참조자의 일종으로 소유권을 갖지 않음.
*/

/// 문자속의 공백 문자 찾기
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter : collection 의 각 요소를 반환
    // enumerate : iter 의 각 결과값을 튜플로 감싸서 반환 -> (요소의 인덱스, 요소의 참조자)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
