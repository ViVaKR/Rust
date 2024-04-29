#![allow(unused)]
use ansi_term::{Colour, Style};
use data_encoding::HEXUPPER;
use rand::Rng;
use ring::agreement;
use ring::digest::{Context, Digest, SHA256};
use ring::error::Unspecified;
use rusqlite::ffi::SQLITE_LIMIT_FUNCTION_ARG; // (5)
use rusqlite::DatabaseName::Temp;
use rusqlite::{params, Connection, Result};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write}; // (2)
use std::mem::size_of_val;
use std::path::PrefixComponent;
use std::process::{exit, Child};
use std::{array, io}; //  (1)

// (3)
fn random_number() {
    // Random
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}
fn number_types() {
    let is_true = true;
    let my_grade = 'A'; // char
    let num_f32: f32 = 1.111111111111111;
    let num_f64: f64 = 1.111111111111111;

    // 정밀도 차이
    println!("f32 : {}", num_f32 + 0.111111111111111); // 1.2222223
    println!("f64 : {}", num_f64 + 0.111111111111111); // 1.2222222222222219

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;

    // Operator
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
}
fn if_statement(age: i32) {
    if age >= 1 && age <= 18 {
        println!("You ar nice days...");
    } else if (age == 21 || age == 50) {
        println!("21 or 50 years old");
    } else if age >= 65 {
        println!("65 over ...");
    } else {
        println!("Not a Important");
    }

    let can_vote = if age > 18 { true } else { false };
    println!("Can Vote : {}", can_vote);
}

fn match_statement(age: i32) {
    match age {
        1..=18 => println!("You are 1..18 years old"),
        21 | 50 => println!("You are 21 or 50 years old"),
        19..=64 => println!("You are very important human"),
        65.. => println!("You are old man!"),
        _ => println!("You are Unknow human.."),
    };

    let voting_age = 18;

    match age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
        _ => println!("Unknown is can Vote"),
    }
}

fn loops_ex() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st : {}, length : {}", arr[0], arr.len());

    let mut loop_idx = 0;
    loop {
        if (arr[loop_idx] % 2 == 0) {
            loop_idx += 1;
            continue;
        }
        if arr[loop_idx] == 9 {
            break;
        }
        println!("Array Odd Number : {} value -> {}", loop_idx, arr[loop_idx]);

        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr.len() {
        println!("while loop => {}", arr[loop_idx]);
        loop_idx += 1;
    }

    for val in arr {
        println!("for loop -> {}", val);
    }
}

fn tuple_ex() {
    let my_tuple: (u8, String, f64) = (47, "Viv".to_string(), 50_000_000.458);
    println!(
        "Age : {}, Name : {}, Money : {}",
        my_tuple.0, my_tuple.1, my_tuple.2
    );

    let (age, name, money) = my_tuple;
    println!("{} {} {}", age, name, money);
}

fn string_ex() {
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" B C D E F Hello World");

    println!("{str1}");

    for word in str1.split_whitespace() {
        println!("Splited String : {word}");
    }

    // Replace
    let str2 = str1.replace("D", "Z");
    println!("{}", str2);

    // 문자열 정렬
    let random_text = String::from("x r t b h k z a m c");
    let mut v1: Vec<char> = random_text.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let str4: &str = "Random String";
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    let byte_arr1 = str5.as_bytes();
    let str6 = &str5[0..6];
    println!("String length : {}", str6.len());

    str5.clear();

    // string print unicode
    let str7: String = String::from("Just some");
    let str8: String = String::from(" words 123456");
    let str9: String = str7 + &str8;
    for char in str9.bytes() {
        println!("{}", char);
    }
}

fn stdin_readline() {
    // 빈 문자열 반환.
    let mut name = String::new();
    let greeting = "Nice to meet you";

    println!("What's Your name? :  ");
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {} {}", name.trim_end(), greeting);
}
/* [ 열거형 ]
    -

*/

enum IpAddrKind {
    V4,
    V6,
}

enum IpKindAddr {
    V4(String),
    V6(String),
}

enum IpKindAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // 관련된 데이터 없음
    Move { x: i32, y: i32 },    // 구조체 처럼 필드 이름을 지정
    Write(String),              // String 하나 포함.
    ChangeColor(i32, i32, i32), // 3개의 값을 포함.
}

/* [ 위 열겨형을 구조체로 정의 ]
    struct QuitMessage;
    struct MoveMessage { x:i32, y:i32}
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

/* [ 구조체 ]
    - 데이터의 그룹화
    - 의미 있는 그룹을 구성하는 여러 관련 값을 함께 패키지하고 이름을 지정할 수 있는
    - 사용자 정의 데이터 유형
    - 객체의 데이터 속성과 동일
    - 튜플과 다른점 : 각 데이터 조각의 이름을 지정하여 값이 의미하는 바가 명확하여 더 유연함
    - 키워드 : struct

*/

struct User {
    // (25)
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 필드가 없는 단위형 구조체 (단위 유사 구조체)
// 유형 자체에 저장하려는 데이터가 없을 때 유용함
struct AlwaysEqual;

/* 메소드 (Method)
    -- 함수와 유사함.
    - 매개변수와 반환값을 가질 수 있음
    - 구조체의 컨텍스트 내에서 정의되며 첫번째 매개변순는 항상 self 구조체의 인스턴스를 나타냄
    -
*/

impl Rectangle {
    // 메소드
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn struct_build(_active: bool, _username: String, _email: String, _sign_in_count: u64) -> User {
    User {
        active: _active,
        username: _username,
        email: _email,
        sign_in_count: _sign_in_count,
    }
}
fn struct_runner() {
    // (25)

    let mut user = User {
        active: true,
        username: String::from("Viv"),
        email: String::from("bm@live.co.kr"),
        sign_in_count: 1,
    };

    user.email = String::from("admin@vivabj.com");

    println!(
        "User A -> {} {} {} {}",
        user.active, user.username, user.email, user.sign_in_count
    );

    let userb = struct_build(
        false,
        String::from("Kim Bum Jun"),
        String::from("iam@kimbumjun.com"),
        9,
    );

    println!(
        "User B -> {} {} {} {}",
        userb.active, userb.username, userb.email, userb.sign_in_count
    );

    // 튜플 구조체
    let color = Color(125, 125, 125);
    let point = Point(100, 100, 100);

    println!(
        "Color => {} {} {}, {} {} {}",
        color.0, color.1, color.2, point.0, point.1, point.2
    );

    let subject = AlwaysEqual;
    println!("{:?}", subject.type_id());

    // Rectangle
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_tup = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels (tupple)",
        area_tup(rect_tup)
    );
    println!(
        "The area of the rectangle is {} squar pixels. (struct)",
        area_struct(&rect)
    );

    // Struct 의 모든 필드의 값 보기
    //-> rect = Rectangle { width: 30, height: 50 }
    println!("rect = {:?}", rect);
    let scale = 2;

    // view debug info
    // 코드가 수행하는 작업을 파악할때 유용함.
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    // Method
    println!(
        "The area of the rectangle is {} square pixels.\nwidth is gt 0 ? = {} (Method)",
        rect.area(),
        rect.width()
    );

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };

    let r3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r2? {}", r1.can_hold(&r3));
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area_tup(demensions: (u32, u32)) -> u32 {
    demensions.0 * demensions.1
}
/* [ Slice ] */
fn slice_target() {
    // (24)
    let mut s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}, {world}");
    let word = first_word_a(&s);
    println!("first word -> {word}");

    // let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    println!("hello[3..len] -> {slice}");
    let slice = &s[..];
    println!("hello[..] -> {slice}");

    let str_literal = "Hello, World";
    let word = first_word_b(&str_literal);
    println!("{word}");

    let a = [1, 2, 3, 4, 5];
    let sl = &a[1..3];
    assert_eq!(sl, &[2, 3]);
    println!("Success!");
}

fn first_word_b(s: &str) -> &str {
    &s[0..7]
}

fn first_word_a(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/* 소유권 */
fn ownership() {
    /*
       [ 메모리 관리 방식 ]
       1. 정기적인 가비지 수집
       2. 명시적인 메모리 할당 해제
       3. 소유권 시스템을 통한 메모리 관리

       [ 스택 ]
       - 후입선출 (Last In First Out)
       - 순서대로 값을 저장하고 반대 순서로 값을 제거함.
       - 고정크기
       - 컴파일시 크기를 알수 없거나 크기가 변경될 수 있는 데이터는 힙에 저장
       - 힙에 대한 포인터는 알려진 고정 크기이므로 스택에 포인터를 자장할 수 있음.
       - 힙에 공간을 할당할 때는 데이터를 보관할 만큼 큰 공간을 찾은 다음 할당하므로 다 많은 작업이 필요함.
       - 힙의 데이터에 엑세스하려면 포인터를 따라야 하기 때문에 스택의 데이터에 엑세스하는 것보다 속도가 느림.
       - 함수에 전달된 값(힙의 데이터에 대한 포인터 포함)과 함수의 지역변수는 스택에 푸시됨
       - 함수가 끝나면 해당 값은 스택에서 제거됨.
       - 코드의 어떤 부분이 힙의 어떤 데이터를 사용하고 있는지 추적하고, 합에서 중복 데이터의 양을 최소화 하고 공간이 부족하지 않도록 사용되지 않는 데이터를 정리하는 것이 모두 소유권이 해결하는 문제임.

       [ 소유권 규칙 ]
       1. Rust 의 각 값에는 소유자가 있음.
       2. 한 번에 한명의 소유자만 있을 수 있다.
       3. 소유자가 범위를 벗어나면 값이 삭제 됨.

    */

    // 변경가능한 문자열
    let mut s = String::from("Hello");
    s.push_str(", World!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `Hello, World`

    // 얕은 복사.

    // 클론과 상호 작용하는 변수 및 데이터
    // 깊은 복사
    let s1 = String::from("fine");
    let s2 = s1.clone();
    // 얕은 복사
    // let s3 = s1; // <- s1은 유효한 상태가 아니므로, 오류 발생함.
    println!("s1 = {}, s2 = {}", s1, s2);

    // 힙 얕은 복사 : 함수에 값을 전달하는 메커니즘은 변수에 값을 할당할 때와 유사함.
    let owner = String::from("good"); // owner into scope
    takes_ownership(owner); // owner value moves into the function
                            // println!("{owner}");                     // and so is no longer valid here

    // 스택 전용 복사 : 유효함.
    let x = 5;
    let y = x;

    make_copy(x); // x copy to -> make_copy
    println!("x = {}, y = {}", x, y); // so it's okay to still

    // 반환 값과 범위
    //-> 값을 반환하면 소유권ㅇ르 이전할 수도 있음.
    let s1 = gives_ownership(); // gives_ownership moves its return
    println!("gives_ownership : {s1}"); // value into s1

    let s2 = String::from("hello"); // s2 is moved into takes_and_gives_back
    let s3 = takes_and_gives_back(s2); // also moves its return value into s3
    println!("{s3}");

    // 튜플
    let s4 = String::from("nice");
    let (r1, len) = calculate_length(s4);
    println!("The length of '{r1}' is {len}");

    // 참조 -> `&`, borrowing
    let s6 = String::from("vivakr");
    let len_s6 = calculate_length_ref(&s6); // 값을 소유하지 않는 &s6 참조, s6 는 삭제되지 않음.
    println!("The length of '{s6}' is {len_s6}");

    // borrowing
    let book = Book {
        pages: 5,
        rating: 9,
    }; // book 컬리 브레이스 중괄호에서 자동 삭제됨으로 Brow

    display_rating(&book); // 소유권이 이전됨..
    display_page_count(&book);

    // Grocery Item
    let my_item = GroceryItem {
        quantity: 3,
        id: 99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
} // 닫는 중괄호에서 메모리 반환이 자동 호출됨. (drop)

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
} // drop book

// 변경가능한 참조 (
// 불변형 참조 (reference)
fn calculate_length_ref(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope, But because it dows not have onwership of what it refers to, it is not droped. 그러나 참조 역시 불변이므로 참조한 내용을 수정하는 것은 허용되지 않음.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string + ", world"
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// ownership (heap)
fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope -> memory is freed.

// ownership (stack)
fn make_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer goes out of scope -> Nothing special happens.

/* 제어흐름 (Control Flow) : if, loop */
fn control_flow(number: i32) {
    /*
         [ if statement ]
         - 표현식 if 를 사용하면 조건에 따라 코드를 분기할 수 있습니다.
         - 조건을 제공한 다음
         - 이조건이 충족되면 이 코드 블록을 실행하고
         - 충족되지 않으면 이 코드 블록을 실행하지 마세요..

         [ loop, while, for statement ]
         - 코드 블록을 두 번 이상 실행하는 것이 유용한 경우가 많음.
         - 코드 블록 끝에서 다시 즉시 처음 부터 시작한는 여러 루프를 제공함.
         - loop : 무한 반복, 명시적인 중지 지시(break, ctrl + c)할 때 까지 반복

    */

    /* if statement */
    if number < 100 {
        // 조건 100 보다 파라미터 number 가 작은 값인지 확인하기
        println!("{number} -> condition was true");
    } else {
        println!("{number} -> condition was false");
    }

    // 다중 조건식
    if number % 4 == 0 {
        // 조건 1
        println!("number  is divisible by 4")
    } else if number % 3 == 0 {
        // 조건 2
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 조건 3
        println!("number is divisible by 2");
    } else {
        // 그외 모든 조건
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 문에서 let 사용
    // 숫자 자체도 표현식이됨
    let check = number % 3;
    let cond = if check == 0 { 300 } else { 500 };
    println!("The value of number ({number} % 3) is: {check}, choice = {cond}");

    /* loop */
    let mut count = 0;
    let mut sum = 0;
    let result = loop {
        if count == 50 {
            break sum;
        }
        count += 1;
        sum += count;
    };

    println!("The result is {result}");

    // while

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The array value is: {}", a[index]);
        index += 1;
    }

    // for
    for element in a {
        println!("The for loop value is: {element}");
    }

    for num in (1..4).rev() {
        // 역순 카운트
        println!("{num}!");
    }
    println!("LIFTOFF!!");
}

// 명령문 (Statement), 표현식 (Expression)
fn statement_expression() -> i32 {
    /*
        [ Statements (명령문) & Expressions (표현식) ]

        1. statements (명령문) : 어떤 동작을 수행하고 값을 반환하지 않는 명령.
            -> 명령문 : 결과값을 반환하지 않음
            -> (오류) -> `let x = (let y = 6);`
            -> (오류, 다른 언어와 다른 점) -> `let a:i32 = b:i32 = 10;`

        2. expression (표현식) :
            -> 명령문의 일부일 수 있음
            -> 결과 값으로 평가합니다.
            -> 중괄호로 만든 새 범위 블록은 표현식입니다.
            -> 함수호출 매크로 호출은 표현식입니다.

        함수는 선택적으로 표현식으로 끝나는 일련의 문장으로 구성되어 있다.
        러스트는 표현 기반 언어

        - statement (구문, 명령문) : 일부 작업을 수행하고 값을 반환하지 않는다. (let y = 6 ),
            함수의 정의도 구문이다.
            다른 변수에 할당할 수 없음
            다른 언어는 할당이 할당값을 반환하는 다른 언어와 다른점이 다. (x = y = 6, Rust 에서는 안됨)

        - express (표현) : 결과 값으로 평가됨
            함수를 호출하는 것은 표현식
            중괄호로 만든 새 범위 블록은 표현식임
    */

    let number = rand::thread_rng().gen_range(1..=100);
    let odd_even = if number % 2 == 0 { "even" } else { "odd" };

    println!("{number} : {odd_even}");

    // Expression (표현식)
    let y = {
        let x = 3;
        x + 1 // semicolon 이 없음에 유의.
    };

    // 함수 호출 -> 표현식
    println!("{}", y);

    y * 128 // 세미콜론이 없는 표현식, 반환값.
}

// 값을 반환하는 함수.

fn five() -> i32 {
    // return 키워드를 사용하여 값을 지정하면 조기에 반환할 수 있지만
    // 대부분의 함수는 마지막 표현식을 암시적으로 반환 함
    5 // return value, expression
}
fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; <- error, statement
}
fn five_caller() {
    let x = five();
    println!("The return value of function is: {x}");
    let result = plus_one(33);
    println!("The return value of plus_one: {result}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("The value of x is : {x}");

    // error due to String vs usize
    // let mut spaces = "    ";
    // spaces = spaces.len();
}

fn guess_game() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Is Not Number. Please Input Number");
                continue;
            }
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too bing!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn condition() {
    println!("{}", (2f32) < 3f32);
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
fn var_constant() {
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

    println!("Hollo");
    println!("Exercise Failed if printing out this line!");
}

// Implement this function, don't modify the fn signatures

// ( macros ) //
// panic!() or
// unimplemented!() or
fn size_of(c1: char, c2: char) {
    println!("{:?}", size_of_val(&c1));
    println!("{:?}", size_of_val(&c2));
}

// loop statement
fn loop_ex(mut a: i32) {
    println!("Start");
    loop {
        println!("{:09}", a);
        a = a + 1;
        if a == 20 {
            break;
        }
    }
    println!("Done");
}

// (9) while statement
fn while_statement(mut a: i32) {
    while a < 10 {
        println!("{:#08b}", a);
        a = a + 1;
    }
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    // body
    return a + b;
}

fn data_type() {
    // Rust 는 정적으로 입력된 언어
    // 이는 컴파일 타임에 모든 변수의 유형을 알아야 한다는 것을 의미함.
    // 스칼라 , 혼합형
    // (1) 스칼라 : 4가지 유형의 기본 스칼라 유형을 가짐
    //-> Integer, Float, Boolean, String

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

    // Tuple (튜플) //
    //-> 다양한 유형을 가진 여러 밧을 하나의 복합유형으로 그룹화 하는 일반적인 방법.
    //-> 튜플의 길이는 고정되어있음.
    //-> 선언후에는 길이를 변경할 수 없음.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("Tuple Example : {} {} {}", tup.0, tup.1, tup.2);
    println!("Tuple Example : {} {} {}", x, y, z);

    // Array (배열) //
    //-> 배열은 모두 같은 유형을 가져야 함.
    //-> 다른 언어와 달리 Rust 의 배열은 고정된 길이를 가지고 있음.
    //-> 배열의 값은 대괄호 안에 쉼표로 구분된 목록으로 사용함.

    // 동일한 값으로 초기화
    let arr: [i32; 5];
    arr = [1, 2, 3, 4, 5];
    for item in arr {
        println!("{}", item);
    }
    // 배열 요소 접근
    println!("arr 2 -> {}, arr 4 -> {}", arr[2], arr[4]);
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

fn casting_ex() {
    let int_u8 = 5;
    let int_u8 = 4;

    let int_u32 = (int_u8 as u32) + (int_u8 as u32);
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saterday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saterday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => {
            println!("Donut day")
        }
        Day::Wednesday => {
            println!("Hump day")
        }
        Day::Thursday => {
            println!("Pay day")
        }
        Day::Friday => {
            println!("Almost Weekend")
        }
        Day::Saterday => {
            println!("Weekend")
        }
        Day::Sunday => {
            println!("Weekend")
        }
    }

    println!("Is today the weekend ( {} )", today.is_weekend());
}

fn vector_ex() {
    // 벡터 : 배열과 유사함.
    // 동일한 유형만 저장 가능.

    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    vec2.push(7);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        None => {
            println!("No 2nd value")
        }
        Some(second) => {
            println!("2nd : {}", second)
        }
    }

    // 순환
    for i in &mut vec2 {
        *i *= 2;
    }
    println!("Vector length {}", vec2.len());
    for j in &vec2 {
        println!("{}", j);
    }
    println!("Pop : {:?}", vec2.pop());
    for j in &vec2 {
        println!("{}", j);
    }
}

// Topic: Basic arithmetic
fn sum(x: i32, y: i32) -> i32 {
    // (8)
    x + y
    // 쎄미 콜론 이 있으면 반환 값이 없이 끝남.
    // 리턴 타입은 쌔미 콜론을 넣지 않음.
}

fn display_result(result: i32) {
    let people = "Rustaceans";
    let temp = format!("Hello {people}!");
    let temp2 = format!("{:#?}", (100, 200));

    println!("{:010}, {}, {}", result, temp, temp2);
}

/*
    [ 매크로 ]
    -> 재사용 가능한 코드 덩어리를 정의하는 방법.
    -> 함수와 유사 하지만 값을 생성하는 대신 코드를 생성함.
    -> 컴파일 타임에 평가되므로 보다 효율적인 코드를 생성할 수 있음을 의미.

*/

macro_rules! say_hello {
    () => {
        println!("Hello, World")
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

fn clear_screen() {
    // print!("{}[2J", 27 as char); // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Enum Examples
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32), // X, Y Position
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum TicketDiscount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f32,
}

// enum
fn enum_ex(go: Direction) {
    let direction = match go {
        Direction::Up => "up",
        Direction::Down => "donw",
        Direction::Left => "left",
        Direction::Right => "right",
    };
    println!("Go To The {}", direction);
    print!("Hello {}!", "world");
}

//
fn write_ex() {
    let mut w = Vec::new();
    write!(&mut w, "Hello {}!", "World");

    for char in w {
        println!("{}", char);
    }
}

fn draw_line(title: &str, count: usize) {
    println!("{:=^1$}", title, count);
}

/* [ impl ] */
struct Temperature {
    degree_f: f64,
}

//impl
impl Temperature {
    // 대문자 Self == Temperature
    // 이름 변경시에도 사용 가능함으로
    // Self 사용 권장.
    fn freezing() -> Self {
        Self { degree_f: 32.2 }
    }

    fn boiling() -> Self {
        Self { degree_f: 212.0 }
    }

    fn show_temp(&self) {
        println!("{:?} degree F", self.degree_f);
    }
}

enum Colors {
    Brown,
    Red,
    Green,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Brown => {
                println!("Brown")
            }
            Colors::Red => {
                println!("Red")
            }
            Colors::Green => {
                println!("Green")
            }
        }
    }
}

struct Dimensions {
    // 치수
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("widht: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    weight: f64,
    color: Colors,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Colors, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

/* Vector */
struct VecScore {
    score: i32,
}

// String and &str
fn print_it(data: &str) {
    println!("{:?}", data);
}

struct Employee {
    name: String,
}

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

struct PersonB {
    name: String,
    fav_color: String,
    age: i32,
}
fn print_person(data: &str) {
    println!("\u{2766} =>  {:?}", data);
}

// Derive, Clone Copy not borrowing
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Biz {
    position: Position,
    work_hours: i64,
}

fn print_biz(biz: Biz) {
    // not borrowing
    println!("\u{2766} {:?}", biz);
}

/* ------------------------------------------ */
/* [ 메뉴 ] */
fn menus(menu: &str) {
    println!("{}", menu);
}

fn menu_items() {
    draw_line(" ( menu )", 40);
    menus(
        "\
        1. Impl(1)\t\t2. Impl(2)\t3. Ownership (1)\t4. Vector (1)\t5. String and &str (1)\n\
        6. String and &str (2)\t7. Derive\t8. BitWise Operate\t9. While\t10. Match (1)\n\
        11. Type Annotations\t12. Constant\t13. Data Types (1)\t14. Loop (1)\t15. User Input\n\
        16. Arithmetic\t\t17. Condition\t18. Guess Game\t\t19. Shodowing\t20. Expression\n\
        26. Random Number\t27. if\t\t28. Number Types\t29. Match (2)\t30. Loop (2)\n\
        31. Tuple \t\t32. String\t33. Casting\t\t34. Enum (1)\t35. Say Hello!\n\
        36. Write!\t\t37. Vector (2)\t38. Var (1)\t\t39. Var (2)\t40. OwnerShip (2)\n\
        41. Function\t\t42. Reference\t43. Slice\t\t44. Struct\t45. Method\n\
        46. ANSI Terminal\t47. Sqlite DB\t48. Enum Match\t\t49. Enum\t50. Boolean\n\
        51. Intermediate Memory\t52. Match (3)\t53. Option Type\t\t54. Library API\t55. Result\n\
        56. Hashmamp",
    );

    draw_line(" ( end )", 40);
}

/* [ 메뉴선택 ] */
fn choice_menu() -> u32 {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    println!("메뉴선택 (1 ~ 100, Exit: 0)");
    print!(">> ");
    io::stdout().flush().unwrap();
    input.clear();
    let b = io::stdin().read_line(&mut input).expect("Not integer");
    if let Ok(val) = input.trim().parse::<u32>() {
        val
    } else {
        1000
    }
}

fn pause_screen(choice: u32) {
    let mut pause = String::new();
    println!("\n({}) Complete! Press Enter Show Menu...", choice);
    let pause = io::stdin().read_line(&mut pause);
    clear_screen();
}

fn main() -> Result<()> {
    loop {
        menu_items();
        let mut choice: u32 = choice_menu();
        if (choice == 1000) {
            println!("Please enter a valid number.");
            continue;
        }
        clear_screen();
        println!("\n");

        match choice {
            0 => {
                clear_screen();
                exit(0);
            } // [ Exit ]
            1 => {
                // impl, C# 확장 메서드와 유사함.
                let hot = Temperature { degree_f: 99.9 };
                hot.show_temp();
                let cold = Temperature::freezing(); // :: 식별.
                cold.show_temp(); // borrowing
                cold.show_temp();
                cold.show_temp();
                cold.show_temp();
                let boiling = Temperature::boiling();
                boiling.show_temp();
            } // [ impl(1) ]
            2 => {
                // [ impl(2) ]
                // Impementing functionality with the impl keyword
                let small_dimensions = Dimensions {
                    width: 1.0,
                    height: 2.0,
                    depth: 3.0,
                };

                let small_box = ShippingBox::new(5.0, Colors::Red, small_dimensions);

                small_box.print();
            } // [ impl(2) ]
            3 => {
                ownership();
            } // [ OwnerShip ]
            4 => {
                // [ Vector ]
                // Multiple pieces of data
                // Must be the same type
                // Used for lists of information
                // Can add, remove, and traverse the entries

                // let my_numbers = vec![1, 2, 3];

                let mut my_numbers = Vec::new();
                my_numbers.push(1);
                my_numbers.push(2);
                my_numbers.push(3);

                my_numbers.pop(); // remove last item. -> 3,
                                  // index : square braces.
                println!(
                    "vector length: {}, my_number[1] = {}",
                    my_numbers.len(),
                    my_numbers[1]
                );

                let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

                let mut sum = 0;

                for i in &numbers {
                    sum = sum + i;
                    println!(
                        "{:?} - {:?}",
                        i,
                        if (i ^ 1) == (i + 1) {
                            "짝수"
                        } else {
                            "홀수"
                        }
                    );
                }

                println!("Sum o numbers = {:?}", sum);

                let scores = vec![
                    VecScore { score: 90 },
                    VecScore { score: 88 },
                    VecScore { score: 77 },
                    VecScore { score: 93 },
                ];

                for score in scores {
                    println!("Score = {:?}", score.score);
                }

                for i in numbers {
                    match (i ^ 1) == (i + 1) {
                        true => {
                            println!("{:?} 는 짝수", i);
                        }
                        false => {
                            println!("{:?} 는 홀수", i);
                        }
                    }
                }
            } // [ Vector ]
            5 => {
                // String - owned
                // &str - borrowd String slice
                // Must use an owned String to store in a struct
                // use &str when passing to a function

                // Pass to function
                print_it("a string slice");
                let owned_string = "owned string".to_owned();
                let another_owned = String::from("another");

                print_it(&owned_string); // borrow
                print_it(&another_owned); // borrow

                let emp_name = "Jayson".to_owned();
                let emp_name = String::from("Viv");
                let emp = Employee { name: emp_name };
                println!("{:?} - {}", emp.name, emp.name);

                let receipt = vec![
                    LineItem {
                        name: "ceream".to_owned(),
                        count: 1,
                    },
                    LineItem {
                        name: String::from("fruit"),
                        count: 3,
                    },
                ];

                for item in receipt {
                    print_name(&item.name);
                    println!("{} {}", item.name, item.count);
                }
            } // [ String and &str A]
            6 => {
                //
                let people = vec![
                    PersonB {
                        name: String::from("Viv"),
                        fav_color: String::from("green"),
                        age: 7,
                    },
                    PersonB {
                        name: "JangGilSan".to_owned(),
                        fav_color: String::from("Magenta"),
                        age: 45,
                    },
                    PersonB {
                        name: String::from("LimBa"),
                        fav_color: String::from("SkyBlue"),
                        age: 34,
                    },
                ];
                for p in people {
                    println!("\u{2766} {} {} {}", p.name, p.age, p.fav_color);
                    if p.age <= 10 {
                        print_person(&p.name);
                    }
                }
            } // [ String adn &str B]
            7 => {
                // {:?} - 디버그 토큰 사용
                // #[Derive(Debug)]
                let me = Biz {
                    position: Position::Worker,
                    work_hours: 40,
                };

                println!("\u{2766} {:?}", me);
                println!("\u{2766} {:?}", me.position);
                print_biz(me); // First Copy
                print_biz(me); // Second Copy

                let n = 3;
                match n {
                    3 => println!("\u{2766} three {:?}", n),
                    other => println!("\u{2766} number: {:?}", other),
                }
            } // [ Derive ]
            8 => {
                let (x, y) = (1, 2);
                let s = sum(x, y); // 3
                assert_eq!(s, 3);
                println!("Success!");
                let a = 123;
                let b = 456;
                // Comparison of boolena fuctions same = 0
                // 홀수 인 짝인 판단.
                // 패리티 비트 찾기
                // 특정비트를 반전 시킬 때
                // x ^ 0 = x
                // x ^ x = 0

                // 번호 맞 교환.
                // x^= y    => (x ^ y, y)
                // y ^= x   => (x ^ y, y ^ x ^ y)
                // x ^= y   => (x ^ y ^ x, x)
                //          => (y, x)

                // 비트 뒤집기
                // 0x0A ^ 0xFF = 0x03 (0000 1010 ^ 1111 1111 = 1111 0101)

                // 누락된 숫자 찾기

                // 0 으로 xor 을 수행하면 아무변화가 없으나
                // 1 로 수행 하면 항상 비트가 토글됨.

                // 암호화.

                // 최대값

                let mut array: [i32; 7] = [1, 1, 2, 2, 3, 4, 4];
                let mut x = 0;
                for i in array {
                    x ^= i;
                }
                println!("\u{2766} 중복되지 않은 값 = {}", x);

                println!("\u{2766} {} {}", a ^ a, a ^ b);

                let result = 0;
                let arr = vec![1, 2, 3, 4, 6, 7, 8, 9];

                // 비트 토글 a = b ^ (1 << n)
            } // XOR
            9 => while_statement(1), // [ While statement]
            10 => matches_fn(),      // [ Match ]
            11 => {
                // [ Recap ]
                // Required for function signatures
                // Types are usually inferred
                // Can also be specified in code
                // Explicti type annotations
            } // [ Type Annotations ]
            // 11 => var_01(),
            12 => var_constant(), // [ Constant ]
            13 => {
                data_type();
                data_types_01();
            } // [ Data Type ]
            14 => loop_ex(10),
            15 => user_input(),
            16 => arithmethic(),
            17 => condition(),
            18 => guess_game(), // [ Guess Game ]
            19 => shadowing(),
            20 => {
                expr();
                expr_exercies();
                _ = statement_expression();
            } // [ Expressions ]
            21 => five_caller(),
            22 => control_flow(rand::thread_rng().gen_range(1..=100)),
            23 => ownership(),
            24 => slice_target(),
            25 => struct_runner(),
            26 => random_number(),
            27 => if_statement(18),
            28 => number_types(),
            29 => {
                /* Recap */
                let n = 3;
                match n {
                    3 => println!("\u{2766} three {}", n),
                    other => println!("\u{2766} number {:?}", other),
                }
                match_statement(18)
            } // [ Advanced match ]
            30 => loops_ex(),
            31 => tuple_ex(),
            32 => string_ex(),
            33 => casting_ex(),
            34 => {
                enum_ex(Direction::Left);
            } // [ Enum (1) ]
            35 => say_hello!(), // macros
            36 => write_ex(),
            37 => vector_ex(),
            38 => {
                // TODO
            }
            39 => variables(),
            40 => {
                // Ownership
                // Programs must track memory
                // Rust utilizes an "ownership" model to manage memory
                //  - The "owner" of memory is responsible for cleaning up the memory
                // Memory can either be "moved" or "borrowed"

                let s1 = String::from("Hello, World");
                let (s2, len) = calculate_length(s1);
                println!("{} - {}", s2, len);

                // 소유권 이전
                let access = Access::Guest;
                display_right(access); // 값이 이동됨.
                                       // display_right(access); // 삭제 된 변수이므로 재사용 할 수 없음, 오류 발생.

                // 소유권 대여/차용
                let acc = Access::Admin;
                borrow_right(&acc);
                borrow_right(&acc);
            }
            41 => {
                // function
                function();
            }
            42 => {
                // 참조
                let mut s1 = String::from("Hello");
                // `&` : 참조자, 앰퍼센드 기호
                // 참조자는 어떤값의 소유권을 가져오지 않고 해당 값을 참조할 수 있도록 해줌.
                // `*` : 역참조, & 참조의 반대
                // `&s1` : s1을 참조 하지만 해당 값을 소유하지 않는 참조자를 생성함.
                // `&mut s1` : 가변 참조자

                // 가변 참조자는 1개 이상 중복하여 만들수 없으나, 중괄호로 새로운 스코프를 만들면 가능함.
                let mut s2 = String::from("World");

                {
                    let mut r1 = &mut s2;
                    r1.push_str(" inside");
                    println!("inside {}", r1);
                }
                let r2 = &mut s2;
                println!("outside {}", r2);

                // (1) 데이터 경합을 방지함.(data race)
                // (2) 둘 이상의표인터가 동시에 같은 데이터에 접근 방지
                // (3) 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행 방지
                // (4) 데이터 접근 동기화 메커니즘이 없음
                let len = reference(&mut s1);
                println!("참조자 : {}, length = ( {} )", s1, len);

                let mut str = String::from("Hi");
                let t1 = &str;
                let t2 = &str;
                println!("ref {} {} {}", str, t1, t2);
                // 이지점 부터는 t1, t2 는 사용되지 않음.

                // &mut str 사용가능해짐.
                let t3 = &mut str;
                println!("enable mut {}", t3);

                // [ dangling pointer ]
                // 어떤 메모리를 가리키는 포인터가 남아 있는 상황에서
                // 일부 메모리를 해제해 버림으로써, 다른 개체가 할당 받았을 지도 모르는 메모리를 참조하게 된 포인터.
                // 러스트에서는 참조자를 만들면,
                // 해당 참자가 스코프를 벗어나기 전에 데이터가 먼저 스코프를 벗어나는지 컴파일러에서 확인하여
                // 댕글링 참조가 생성되지 않도록 보장함.
                /* (dangle example)
                    fn main() {
                        let reference_to_nothing = dangle();
                    }
                    fn dangle() -> &String {  // dangle 은 String 참조자를 반환함.
                        let s = String::from("hello"); // s 는 새로운 String 입니다.
                        &s // String s 의 참조자를 반환함.
                    } // 여기서 s 는 스코프 밖으로 벗어나고 버려집니다. 해당 메모리는 해제 됨으로
                      // 위험해집니다. !!

                    따라서 이런 경우에는
                    &s 대신
                    s  로 직접반환해야 합니다.
                */

                // [ 참조자 규칙 ]
                // 단 하난의 가변 참조자만 갖거나, 여러개의 불변 참조자를 가질 수 있음.
                // 참조자는 항상 유효해야 합니다.
            } // [ 참조, Reference ]
            43 => {
                // 문자열 슬라이스, slice
                // `&str` : 문자열 슬라이스를 나타내는 타입
                // String 의 일부를 가리키는 참조자를 말함.

                let mut s = String::from("Hello World");
                let len = s.len();
                let hello = &s[..5]; // 0부터 시작일 때는 생략 가능
                let world = &s[6..]; // 맨 마지막 바이트 까지 포함 할때는 뒤의 값 생략 가능
                let random = &s[3..len];
                let all = &s[..]; // 앞뒤를 생략하면 모두.
                println!("{}, {}. {}, all = {}", hello, world, random, all);

                let word = first_word(&s); // word 는 값 5을 받음.
                s.clear(); // String 을 비움으로 "" 으로 만듦

                // 여기서는 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는
                // 문자열은 더 이상 없음으로 word 는 이제 전혀 유효하지 않으나?

                println!("slice -> {}", word); // 출력은 가능

                // get first word
                let hi = String::from("Hi Everyone");
                let str = slice_first_word(&hi);
                println!("first word : {}", str);

                // [ 문자열 리터럴 ]
                // letter -> 바이너리의 특정 지점을 가르키는 슬라이스
                // 불변 타입
                let letter = "Fine Thanks And You?";
                // 슬라이스는 바로 인수로 전달할 수 있음.
                // String 이라면 String의 슬라이스 혹은 String 의 참조자를 전달할 수 있음.
                // String 에 대한 참조자 대신 문자열 슬라이스를 매개 변수로 하는 함수를 정의하면
                // 기능면에서 손해 보지 않으면서
                // API 를 더 일반적으로 유하게 만들어 줌.
                let word = second_word(letter);
                println!("fine : {}", word);

                let a = [1, 2, 3, 4, 5];
                let slice = &a[1..3];
                assert_eq!(slice, &[2, 3]);

                // 소유권, 참조, 슬라이스는 컴파일 타임에 메모리 안정성을 보장.
            } // [ Slice ]
            44 => {
                // 구조체.
                // 튜플과 같이 각각 다른 타입의 구성요소를 가질수 있음.
                // 각각에 이름을 부여, 순서에 의존할 필요가 없음.
                // field 필드 라고 부르는 각 구성요소.
                // 구조체 사용 : 인스턴스 생성

                // 일부 필드만 가변형으로 만들 수 없음.
                let mut person_kim = PersonA {
                    active: true,
                    username: String::from("Kim Bum Jun"),
                    email: String::from("admin@vivabj.com"),
                    sign_in_count: 1,
                };
                person_kim.email = String::from("bm@live.co.kr");
                let person = build_user(person_kim.email, person_kim.username);
                println!(
                    "{} {} {} {}",
                    person.username, person.email, person.active, person.sign_in_count
                );

                let black = VivColor(0, 0, 0);
                let origin = VivPoint(0, 0, 0);

                println!("black color : {}, {}, {}", black.0, black.1, black.2);
                println!("zero point : {}, {}, {}", origin.0, origin.1, origin.2);

                let subject = AlwaysSame;

                //
                let scale = 2;
                let rect = Rect {
                    width: dbg!(30 * scale),
                    height: 50,
                };

                let area = area(&rect);
                println!("30 * 50 area = {}", area);
                println!("rect is {:?}", rect);
                dbg!(&rect);
            } // [ Struct, 구조체 ]
            45 => {
                // 함수와 유사하나
                // 구조체 컨텍스트에 정의되고
                // 첫번째 매개변수가 항상 self 라는 차이점이 있음.
                // self 매개변수는 메서드를 호출하고 있는 구조체 인스턴스를 의미함.
                // self : 매서드 수신자.
                // &self : 읽기 전용
                // &mut self : 변경가능
                // self : 소비용.

                let rect = Rect {
                    width: 29,
                    height: 15,
                };
                println!(
                    "The area of the rectanble is {} square pixels.",
                    rect.to_area()
                );
                println!(
                    "The rectangle has a greate than 30 width? {}",
                    rect.check_width()
                );

                // 자동 참조 및 역참조 (automatic referencing and dereferencing)
                // p1.distance(&p2); == (&p1).distance(&p2);
                let rect1 = Rect {
                    width: 30,
                    height: 50,
                };
                let rect2 = Rect {
                    width: 10,
                    height: 40,
                };
            } // (확장) 메서드, method
            46 => {
                // Hashing
                println!(
                    "{} {} {} \n{}\n{}",
                    Colour::Red.paint("red"),
                    Colour::Blue.paint("blue"),
                    Colour::Green.paint("green"),
                    Style::new().bold().paint("This is Bold"),
                    Colour::Yellow.paint("This is colored"),
                );
            } // [ Ansi Color Enum ]
            47 => {
                //
                let result = sqlite_db();
                println!("{}", result.iter().enumerate().len());
            } // [ Sqlite DB ]
            48 => {
                //
                let item = Menu::Burger;
                let drink_type = "water";
                let paid = true;
                let order = match item {
                    Menu::Burger => {
                        if drink_type == "water" {
                            true
                        } else {
                            false
                        }
                    }
                    Menu::Fries => true,
                    Menu::Drink => true,
                };

                if order {
                    println!("Ordered");
                }
            }
            49 => {
                // Expressions
                // secret file : admins only
                let access_level = Access::Guest;
                let can_access_file = match access_level {
                    Access::Admin => true,
                    Access::Manager => true,
                    Access::User => false,
                    Access::Guest => false,
                };

                println!("Can Access");
            }
            50 => {
                // boolean expression
                let value = 100;
                print_message(value > 100);
            }
            51 => {
                // Intermediate Memory
                // Offsets
                // Address 4, offset 1 (rows[4], columns[1], Data[1]) , index number
                // each byte get address
                // Memory uses addresses & offsets
                // Addresses are permanent, data difers
                // Offsets can be used to "index" into some data
            }
            52 => {
                // Advanced match
                let flat = TicketDiscount::Flat(2);
                match flat {
                    TicketDiscount::Flat(2) => println!("flat 2"),
                    TicketDiscount::Flat(amount) => println!("flat discount of {:?}", amount),
                    _ => (),
                }

                let concert = Ticket {
                    event: "concert".to_owned(),
                    price: 50.0,
                };

                match concert {
                    Ticket { price, .. } => println!("price = {:?}", price), // ignore others (two dot ..)
                }

                // Advanced match
                let tks = vec![
                    AdvTicket::Backstage(50.0, "Viv".to_owned()),
                    AdvTicket::Standard(15.0, "Hello".to_owned()),
                    AdvTicket::Vip(30.0, "Amy".to_owned()),
                ];

                for item in tks {
                    match item {
                        AdvTicket::Backstage(price, holder) => {
                            println!("Backstage Ticket Holder: {:?}, price: {:?}", holder, price)
                        }
                        AdvTicket::Standard(price, holder) => {
                            println!("Hello Ticket Holder: {:?}, price: {:?}", holder, price)
                        }
                        AdvTicket::Vip(price, holder) => {
                            println!("Vip Ticket Holder: {:?}, price: {:?}", holder, price)
                        }
                    }
                }
            } // Advanced match

            53 => {
                // - Some data of a specified type
                // - Nothing

                // Scenarios
                // 1. Unable to find something
                // 2. Ran out of items in a list
                // 3. Form field not filled out

                let viv = Customer {
                    age: Some(22),
                    email: "viv@example.com".to_owned(),
                };

                let becky = Customer {
                    age: None,
                    email: "becky@example.com".to_owned(),
                };

                match becky.age {
                    Some(age) => println!("customer is {:?} years old", age),
                    None => println!("customer age not provided"),
                }

                println!("Check Bread {:?}", find_quantity("bread"));

                let response = Survey {
                    q1: None,
                    q2: Some(true),
                    q3: Some("A".to_owned()),
                };

                match response.q1 {
                    Some(ans) => println!("q1: {:?}", ans),
                    None => println!("q1: no response"),
                }

                match response.q2 {
                    Some(ans) => println!("q2: {:?}", ans),
                    None => println!("q2: no response"),
                }

                match response.q3 {
                    Some(ans) => println!("q3: {:?}", ans),
                    None => println!("q3: no response"),
                }

                let mary = Student {
                    name: "Mary".to_owned(),
                    locker: Some(3),
                };
                println!("student : {:?}", mary.name);

                match mary.locker {
                    Some(num) => println!("locker number: {:?}", num),
                    None => println!("no locker assigned"),
                }
            } // [ Option (Data) Type ]

            /// Library API
            /// $ `cargo doc --open`
            /// $ `rustup doc``
            54 => {
                //
                let numbers = vec![1, 2, 3];
                match numbers.is_empty() {
                    true => println!("Is Empty"),
                    false => println!("Is Not Empty"),
                }

                // Utilizing Standard Library functionality (표준라이브러리 기능 활용)
                // Notes
                // * Use 'rustup doc' in a terminal top open the standard library docs
                // * navigate to the API Documentation section
                // * Search for functionality to transform a string (or str) to uppercase and lowercase
                //      * Try  searching for: to_uppercase, to_lowercase

                let my_str = "this is my STRING";
                println!(
                    "uppercase: {:?}, lowercase: {:?}",
                    my_str.to_uppercase(),
                    my_str.to_lowercase()
                );

                //
            } // [ Libarary API ]

            55 => {
                // Succesful
                // Error

                // 최적화.
                let choice = pick_choice("end");
                println!("choice: {:?}", choice);

                // Adult
                let child = Adult::new(15, "Anita");
                let adult = Adult::new(21, "Viv");
                match child {
                    Ok(child) => println!("{} is {} years old", child.name, child.age),
                    Err(err) => println!("{err}"),
                }

                match adult {
                    Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
                    Err(e) => println!("{e}"),
                }

                // Result
            } /* [ Result ] */

            56 => {
                // HashMap //
                // --> Collection that stores data as key-value pairs
                // --> Data is located using the "key"
                // --> The data is the "value"

                // Similar to definitions in a dictionary
                // Vary fast to retrieve data usin the key

                let mut people = HashMap::new();
                people.insert("Susan", 21);
                people.insert("Viv", 32);
                people.insert("Will", 14);
                people.insert("Cathy", 22);
                people.remove("Susan");

                match people.get("Ed") {
                    Some(age) => println!("age = {:?}", age),
                    None => println!("not found"),
                }
            } // [ HashMap ]

            57 => {} // [ Macro ]

            _ => {}
        } // excute match
        pause_screen(choice);
        println!("\n");
    }
} // main

// (56) HashMap
/// (55) Result
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_owned(),
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}
fn print_menu(choice: &MenuChoice) {
    println!("choice => {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_menu(&choice);
    Ok(())
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu not found!".to_owned()),
    }
}

/// (53) Option Data Type
struct Customer {
    age: Option<i32>,
    email: String,
}

struct StoreItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let items = vec![
        StoreItem {
            name: "bananas".to_owned(),
            qty: 4,
        },
        StoreItem {
            name: "eggs".to_owned(),
            qty: 12,
        },
        StoreItem {
            name: "bread".to_owned(),
            qty: 45,
        },
    ];

    for item in items {
        if item.name == name {
            return Some(item.qty);
        }
    }

    return None;
}

struct Student {
    name: String,
    locker: Option<i32>,
}

/// (52) Advanced match
enum AdvTicket {
    Backstage(f64, String),
    Standard(f64, String),
    Vip(f64, String),
}

struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

/// borrow
fn borrow_right(acc: &Access) {
    match acc {
        Access::Admin => {}
        Access::Manager => {}
        Access::User => {}
        Access::Guest => {}
    }
}

fn display_right(access: Access) {
    match access {
        Access::Admin => {}
        Access::Manager => {}
        Access::User => {}
        Access::Guest => {}
    }
}

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}
enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

enum Menu {
    Burger,
    Fries,
    Drink,
}

fn sqlite_db() -> Result<()> {
    let conn = Connection::open("vivakr.db");

    &conn.unwrap().execute(
        "create table if not exists people (\
        id integer primary key, \
        name text not null )",
        (),
    )?;

    Ok(())
}

/// [ impl ]
// implementation, 구현
// Rect 컨텍스트에 함수를 정의하기 위해서
// Rect 에 대한 impl 블럭을 만드는 것으로 시작함.
// impl 블록 내의 모든 것은 Rect 타입과 연관됨.
// to_area 함수를 impl 의 중괄호 안으로 옮기고 함수 시그니처의 첫번째 매개변수를
// self (닷넷의 this) 로 지정한 후.
// 닷넷의 확장 메서드 호출 방식과 동일한 방식...
// 메서드 문법 을 사용하여 Rect 인스턴스의 to_area 메서드를 호출 하여 사용함.
// &self 과 같이 & 를 붙여둠, 인스턴스를 변경하고 싶다면 `&mut self` 를 사용함.

impl Rect {
    fn to_area(&self) -> u32 {
        self.height * self.width
    }
}

impl Rect {
    fn check_width(&self) -> bool {
        self.width > 30
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// (44) 구조체

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}

struct PersonA {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 명명된 필드없는 튜플 구조체
struct VivColor(i32, i32, i32);
struct VivPoint(i32, i32, i32);

// 필드가 없는 유사 유닛 구조체
// 테스트 용도로 사용됨.
struct AlwaysSame;

fn build_user(email: String, username: String) -> PersonA {
    PersonA {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
// (43)
fn second_word(s: &str) -> &str {
    let arr = s.as_bytes();
    for (i, &item) in arr.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
fn slice_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// (42) reference
// s 가 참조자 타입임을 나타내줌.
// s 는 소유권이 없으므로 s 가 더이상 사용되지 않을 때도 참조자가 가리킨 값이 버려지지 않음.
fn reference(s: &mut String) -> (usize) {
    s.push_str(", World");
    s.len()
}

// 반환값을 반환하는 함수의 예
// 러스트에서 함수의 반환 값은 본문의 마지막 표현식의 값과 동일
// return 키워드와 값을 지정하여 함수로 부터 일찍 값을 반환할 수 있지만
// 대부분의 함수들은 암묵적으로 마지막 표현식 값을 반환함.
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn ten() -> i32 {
    10
}

fn function() {
    // namming convention : snake case
    // parameter (매개변수), argument (인수)
    // 함수의 본문은 필요에 따라
    // 표현식 (expression) : 결과값을 평가함.
    // 구문 (statement) : 어떤 동작을 수행하고 값을 반환하지 않는 명령.
    // 으로 구성됨
    // 러스트는 표현식 기반의 언어이므로 대부분의 코드는 표현식이며 이는 어떤 값을 평가함.
    // 5 + 6 -> 이수식은 11이라는 값을 평가하는 표현식.
    // 표현식은 구문의 일부일 수 있음.
    // let y = 6; // 구문에서 6은 값을 평가하는 표현식
    // 함수를 호출하는 것도 매크로를 호출하는 것도 표현식.
    // 함수 내부의 스코프 블록도 표현식.
    let y = {
        let x = 3;
        x + ten()
    };
    println!("expression => {}", y);
    // 함수 정의 : 구문
    // let x = (let y = 6); //  잘못된 문장, 다른 언어와의 차이점.
    // 표현식은 종결을 나타내는 세미콜론 을 사용하지 않음.

    println!("{} - {} = {}", 8, 4, sub(8, 4));
    println!("{3} {2} {1} {0}", 1, 2, 3, 4);
    let rem = 6 % 3;
    let rem2 = 6 % 4;
    display_result(rem + rem2);
}

// 변수
fn variables() {
    // 스칼라 타입 : 하나의 값을 표현함.
    // 정수, 부동소수점, 부우린, 문자.
    // panic : 에러가 발생하면서 프로그램이 종료 될때 사용되는 용어.
    // 릴리즈 모드로 컴파일 (release flag) 시, 정수 오버플로우 발생시 -> 2의 보수 감싸기를 수행
    // two's complement wrapping : 해당 타입이 가질수 있는 최댓값 도다 더 큰값은 허용되는 최솟값으로 돌아감 (wrap around, u8-> 256=0, 257=1 이됨.)
    // wrapping_* : 메서드 감싸기 동작
    // checked_* : None
    // overflowing_* : 오버플로우 발생이 있었는지 알려주는 부울린 값 반환
    // saturating_* : 값의 최대 혹은 최소값 사이로 제한

    // 부동소수점 : IEEE-754
    // f32 : single-precision
    // f64 : double-precision

    // 정수 나눗셈은 가장 가까운 정숫값으로 버림.

    // [ 문자, char ]
    // 4bytes
    // 유니코드 스칼라 값을 표현, 한글 이모지 넓이가 0인 공백 문자 표현가능
    // U+0000 - U+D7FF, U+E000 - U+10FFFF

    // [ 복합타입, compound type ]
    // 여러값을 하나의 타입으로 묶을 수 있음.
    // 튜플 (tuple) : 고정길이, 한번 선언되면 그 크기를 변경할 수 없음.

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let temp: (i32, f64, u8) = tup;
    println!("{} {} {}, {} {} {}", x, y, z, temp.0, temp.1, temp.2);

    // 배열(array) : 요소의 갯수가 변경되지 않을 때.
    let array: [i32; 6] = [4, 5, 6, 7, 8, 9];
    let arr = [3; 3]; // 같은 요소로 초기화
    for item in arr {
        println!("arr --> {}", item);
    }
    for item in array {
        println!("array --> {item}");
    }

    print!("Enter Index number >> ");
    io::stdout().flush().unwrap();

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];
    println!("Index is {}", element);
    // 백터 (vector) : 배열보다 보다 유연함, 요소가 변경될 때.

    // max value
    println!("Max u8 : {}", u8::MAX);
    println!("Max u16 : {}", u16::MAX);
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
    println!("Max isize : {}, usize : {}", isize::MAX, usize::MAX);
    // isize, usize : 64-bit arch, 32-bit arch
    // Byte(u8 only) : b'A'
    // Decimal : 98_222
    // Hex : 0xf
    // Octal : 0o77
    // Binary : 0b1111_0000
    // i32 : rust default

    const ONE_MIL: u32 = 1_000_000;
    const PI: f64 = std::f64::consts::PI;
    // (쉐도잉) 같은 이름의 다른 형식의 변수 선언 가능
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age was't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
