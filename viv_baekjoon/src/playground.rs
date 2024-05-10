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

/// 3.
pub fn playground_match() {
    //

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
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    //
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
