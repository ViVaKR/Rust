use std::io::{self};

pub fn data_type() {
    // Scalar Type
    // integer, float , boolean, char

    println!(
        "\u{26EC} {0:>10}\t{1} ~ {2}, {1:#08b} ~ {2:#08b}",
        "i8",
        i8::MIN,
        i8::MAX
    );
    println!(
        "\u{26EC} {0:>10}\t{1} ~ {2}, {1:#08b} ~ {2:#08b}",
        "u8",
        u8::MIN,
        u8::MAX
    );
    println!("\u{26EC} {:>10}\t{} ~ {}", "i16", i16::MIN, i16::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "u16", u16::MIN, u16::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "i32", i32::MIN, i32::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "u32", u32::MIN, u32::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "i64", i64::MIN, i64::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "u64", u64::MIN, u64::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "i128", i128::MIN, i128::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "u128", u128::MIN, u128::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "iszie", isize::MIN, isize::MAX);
    println!("\u{26EC} {:>10}\t{} ~ {}", "usize", usize::MIN, usize::MAX);

    // --> panic : 에러가 발생하면서 프로그램이 종료되는 경우 패닉이라는 용어를 사용함.
    // --> integer overflow : 2의 보수 감싸기 (two's complement wrapping)
    //      -- 해당 타입이 가질 수 있는 최댓값 보다 더 큰 값은 허용되는 최솟값으로 돌아감 (wrap around)
    //      -- u8일 경우 256 은 0, 257은 1이 되는 방식, 패닉을 발생시키지는 않으나 에러로 간주.

    /* 부동 소수점 */
    // f64 : double-precision (2배수 정밀도), 기본 타입
    // f32 : signle-precision (1배수 정밀도)

    /* 복합 타입, Compound type */
    // 1. tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("\n\u{26EC} {} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("\n\u{26EC} {} {} {}", five_hundred, six_point_four, one);

    // 2. array
    let a = [1, 2, 3, 4, 5];
    for item in a {
        println!("\u{26EC} {} ", item);
    }

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "Septempber",
        "October",
        "November",
        "December",
    ];
    for item in months {
        println!("\u{26EC} {} ", item);
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("\u{26EC} {:#?} ", a);

    let a = [3; 10]; // init same value
    println!("\u{26EC} {:#?} ", a);

    println!("\n\u{269E} 출력할 배열 인덱스 번호를 입력하세요 \u{269F}\n ");
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("\u{26EC} The value of the element at index {index} is: {element}");
}

pub fn operation() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    let t = true;
    let f: bool = false;
    let c = 'z';
    let z: char = '\u{2766}';

    println!(
        "\u{26EC} {}, {}, {}, {}, {}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder, t, f, c, z
    );
}

pub fn vector() {
    // 벡터 만들기
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(7);
    v.push(11);
    v.push(14);
    v.push(33);

    let third = &v[2];
    println!("\u{26EC} vec = {:?} third = {}", v, third);
    let last = v.get(v.len() - 1);

    match last {
        Some(val) => println!("\u{26EC} last = {}", val),
        None => todo!(),
    }

    let v2 = vec![1, 2, 3];
    println!("\u{26EC} v2 = {:?}", v2);

    println!("\u{26EC} (0..10).collect ");
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("\u{26EC} {:?}", collected_iterator);

    println!("\n\u{269E} vec! macro can be used to initialize a vector \u{269F}");
    let mut xs = vec![1i32, 2, 3];
    xs.push(4);
    xs.push(5);
    xs.push(12);
    xs.push(14);
    let element = match xs.pop() {
        Some(val) => val,
        None => -1,
    };

    println!(
        "\u{26EC} vec! {:?}, len ({}), {}, pop {}",
        xs,
        xs.len(),
        xs[2],
        element,
    );

    // yeild return with index
    for (i, item) in xs.iter().enumerate() {
        println!("\u{26EC} {} - {}", i, item);
    }

    // modifying each value
    for item in xs.iter_mut() {
        *item *= 3;
    }
    println!("\u{26EC} Update vector: {:?}", xs);

    // step by
    println!("\n\u{269E} Step by \u{269F}");
    for item in (1..=10).step_by(2) {
        println!("\u{26EC} {}", item);
    }

    println!("\n\u{269E} - \u{269F}");
    let mut i = 0;
    while i <= 10 {
        println!("\u{26EC} {}", i);
        i += 2;
    }
}
