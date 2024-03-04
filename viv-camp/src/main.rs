use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::mem::size_of_val;

// Entry Point : `main`
//
fn main() {
    // Rust code uses snake case as the conventional style for function and variable names.

    let mut input = String::new();

    loop {
        println!("Select Menu (1 ~ 100, Exit: 0)");
        print!(">> ");
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
            4 => data_type(),
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
            18 => guess_game(),
            19 => shadowing(),
            20 => _ = statement_expression(),
            21 => five_caller(),
            22 => control_flow(rand::thread_rng().gen_range(1..=100)),
            _ => break,
        }
    }
}

// 제어흐름 (Control Flow) : if, loop
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
    */

    let y = {
        let x = 3;
        x + 1 // semicolon 이 없음에 유의.
    };

    println!("{}", y);

    y * 128 // 세미콜론이 없는 표현식, 반환값.
}

// 값을 반환하는 함수.
fn five() -> i32 {
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

    println!("Hollo");
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

fn data_type() {
    // Rust 는 정적으로 입력된 언어
    // 이는 컴파일 타임에 모든 변수의 유형을 알아야 한다는 것을 의미함.
    // 스칼라 , 혼합형
    // (1) 스칼라 : 4가지 유형의 기본 스칼라 유형을 가짐
    //-> Integer, Float, Boolean, String
    //-

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
