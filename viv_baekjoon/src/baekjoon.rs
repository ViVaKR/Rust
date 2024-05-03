use std::io::Write;

pub fn demo() {
    let mut str = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .expect("Failed to read");

    let array: Vec<_> = str.trim().split(' ').collect();
    let number: String = array[0].trim().parse().expect("Wanted string");
    let base: i32 = array[1].trim().parse().expect("Wanted a number");

    // let mut number = String::new();
    // std::io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed to read");

    // let base: i32 = match number.trim().parse::<i32>() {
    //     Ok(val) => val,
    //     Err(_) => 36,
    // };

    let mut sum: i32 = 0;
    for (i, c) in number.trim().chars().rev().enumerate() {
        let m: i32 = if c.is_numeric() {
            c as i32 - '0' as i32
        } else {
            c as i32 - 'A' as i32 + 10
        };
        sum += m * base.pow(i as u32);
    }
    println!("{}", sum);
}
/// 윤년, Leap Year
pub fn bj_2753() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let year: usize = input.trim().parse().expect("Not a number");

    let result = match year {
        temp if (temp % 4 == 0 && temp % 100 != 0) => 1,
        temp if (temp % 4 == 0 && temp % 400 == 0) => 1,
        temp if (temp % 100 == 0 && temp % 400 != 0) => 0,
        temp if (temp % 400 == 0) => 1,
        _ => 0,
    };
    println!("{}", result);
}

/// 11382 : 데이터 타입 주의
pub fn bj_11382() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let array: Vec<_> = input.trim().split(' ').into_iter().collect();

    let a: i64 = array[0].trim().parse().expect("Wanted a number");
    let b: i64 = array[1].trim().parse().expect("Wanted a number");
    let c: i64 = array[2].trim().parse().expect("Wanted a number");
    let sum = a + b + c;
    println!("{}", sum);
}

/// 2743
pub fn bj_2743() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim().len());
}

/// 1330
pub fn bj_1330() {
    let mut input = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let numbers: Vec<_> = input.trim().split(' ').into_iter().collect();

    let a: i32 = numbers[0].trim().parse().expect("Wanted a number");
    let b: i32 = numbers[1].trim().parse().expect("Wanted a number");

    let result = match a.cmp(&b) {
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Equal => "==",
        std::cmp::Ordering::Greater => ">",
    };

    println!("{}", result);
}

/// 2739. 구구단
pub fn bj_2739() {
    println!("\n\u{269E} 단을 입력하세요. \u{269F}");
    let mut input = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let grade: u8 = input.trim().parse().expect("Not a number");

    for item in (1..=9).into_iter() {
        println!("{} * {} = {}", grade, item, grade * item);
    }
}

pub fn bj_18108() {
    let buddha_year = 543;
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let year: i32 = input.trim().parse().expect("Wanted buddha year");
    println!("{}", year - buddha_year);

    /*
        2024년 부처님 오신 날
        ● 몇주년 : 불기 2568년
        ● 날짜 : 양력 2024년 5월 15일 수요일
        ● 날짜 : 음력 2024년 4월 8일 수요일
    */
}

pub fn bj_10926() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let len = input.len();
    input.truncate(len - 1);
    println!("{}??!", input);
}

pub fn bj_10869() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<&str> = input.split(' ').into_iter().collect();
    let a: i32 = digits[0].trim().parse().expect("Wanted a number");
    let b: i32 = digits[1].trim().parse().expect("Wanted a number");

    println!("{}\n{}\n{}\n{}\n{}", a + b, a - b, a * b, a / b, a % b)
}

pub fn bj_1008() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<_> = input.split(' ').into_iter().collect();
    let a: f64 = digits[0].trim().parse().expect("Wanted a number");
    let b: f64 = digits[1].trim().parse().expect("Wanted a number");
    println!("{:16}", a / b);
}

/// 곱셈
pub fn bj_10998() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<_> = input.split(' ').into_iter().collect();
    let a: i32 = digits[0].trim().parse().expect("Wanted a number");
    let b: i32 = digits[1].trim().parse().expect("Wanted a number");
    println!("{}", a * b);
}

/// max value and order number
pub fn bj_2562() {
    println!("\n\u{269E} BAEKJOON 2565 \u{269F}");
    let mut array: Vec<usize> = Vec::new();
    let mut index: usize = 0;
    let mut max = usize::MIN;
    let mut max_order: usize = 0;

    loop {
        if index == 9 {
            break;
        }
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let value = input.trim().parse::<usize>();
        match value {
            Ok(val) => {
                array.push(val);
                if max < array[index] {
                    max = array[index];
                    max_order = index + 1;
                }
                index += 1;
            }
            Err(_) => continue,
        }
    }
    println!("{}\n{}", max, max_order);
}
