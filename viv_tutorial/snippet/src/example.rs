pub(crate) use std::time::Duration;

pub fn std_fmt() {
    let five_sec = Duration::from_secs(5);
    assert_eq!(five_sec, Duration::from_millis(5_000));
    assert_eq!(five_sec, Duration::from_micros(5_000_000));
    assert_eq!(five_sec, Duration::from_nanos(5_000_000_000));

    println!(
        "{a:10}\n{b:10}\n{c}",
        a = five_sec.as_secs(),
        b = five_sec.as_millis(),
        c = five_sec.as_nanos()
    );

    let mut tmp = format!("{:?}", (3, 4));
    println!("\u{26EC} {:?}", tmp);

    tmp = format!("{1} {} {0} {}", 1, 2);
    println!("{}", tmp);
    println!("\n\u{269E} 채우기 \u{269F}");
    println!("\u{26EC} {:5}!", "v");
    println!("\u{26EC} {:1$}!", "v", 5);
    println!("\u{26EC} {1:0$}!", 5, "v");
    println!("\u{26EC} Hello, {:width$}!", "World", width = 10);

    println!("\n\u{269E} 정렬 \u{269F}");
    // [fill]< - the argument is left-aligned in width columns
    // [fill]^ - the argument is center-aligned in width columns
    // [fill]> - the argument is right-aligned in width columns
    println!("\u{26EC} {:-<5}!", "v");
    println!("\u{26EC} {:-^5}!", "v");
    println!("\u{26EC} {:->5}!", "v");

    println!("\u{26EC} Hello {:^15}!", format!("{:?}", Some(", World")));

    println!("\n\u{269E} Sign \u{269F}");
    // +
    // #? - 디버그 형식을 예쁘게 인쇄 (줄바꿈 및 들여쓰기 추가)
    // #x - 0x
    // #b - 0b
    // #o - 0o
    println!("\u{26EC} Hello {:+}!", 5); // + 기호 표시
    println!("\u{26EC} {:#?}", (2 + 5));
    println!("\u{26EC} {:#010x} - {:#010X}", 14, 247);
    println!("\u{26EC} {:#032b}", -12);

    println!("\n\u{269E} Precision \u{269F}");
    println!("\u{26EC} Hello {0} is {1:.5}", "x", 3.14);
    println!("\u{26EC} Hello {1} is {2:.0$}", 5, "x", 3.14);
    println!("\u{26EC} Hello {1} is {2:.*}", 5, "x", 3.14);
    println!(
        "\u{26EC} Hello {} is {number:.pre$}",
        "x",
        pre = 5,
        number = 3.14
    );
    println!("\u{26EC} {name:>16.0$}", 3, name = "1234.56");

    println!("\n\u{269E} Escape \u{269F}");

    println!("\u{26EC} Hello {{}}");

    /*
    format_spec
        [[fill]align] [sign] ['#']['0'][width]['.' precision] type
        - fill : character
        - align : '<', '|', '>'
        - sign : '+', '-'
        - width : count
        - precision : count | '*'
        - type : '', '?', 'x?', 'X?' identifier
        - parameter : argument '$'
    format specifier
        - nothing : Display
        - ? : Dubug
        - x? : LowerCase Hex Debug
        - X? : UpperCase Hex Debug
        - o : Octal
        - x : LowerHex
        - X : UpperHex
        - p : Pointer
        - b : Binary
        - e : LowerExp
        - E : UpperExp
     */
}

pub fn array_ex() {
    let mut array: [i32; 3] = [0; 3];

    array[1] = 1;
    array[2] = 2;

    for i in &array {
        print!("{i} ");
    }
    println!("\n");
    for j in array {
        print!("{j} ");
    }

    let bytes: [u8; 3] = [0, 1, 2];

    for i in bytes {
        println!("\u{26EC} {}", i);
    }

    let [hello, world] = ["hello".to_string(), "world".to_string()];
    println!("\u{26EC} {} - {}", hello, world);

    let tuple: (u32, u32, u32) = (1, 2, 3);
    let arr: [u32; 3] = tuple.into();

    for i in arr {
        println!("\u{26EC} {}", i);
    }

    let array: [i32; 3] = [0; 3];

    for item in array.into_iter().enumerate() {
        let (i, x) = item;
        println!("\u{26EC} array[{i}] = {x}");
    }

    for item in array.iter().enumerate() {
        let (i, x) = item;
        println!("\u{26EC} array[{i}] = {x}");
    }

    let x2 = ["Ferris", "Bueller's", "Day", "Off"];
    for item in x2.map(|v| v.len()) {
        println!("\u{26EC} {}", item);
    }
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn devide_by(a: f64, b: f64, operator: Operator) -> Option<f64> {
    match operator {
        Operator::Add => Some(a + b),
        Operator::Sub => Some(a - b),
        Operator::Mul => Some(a * b),
        Operator::Div => {
            if a == 0.0 || b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
    }
}
